use cmake::Config;
use fs_extra::dir::{copy, CopyOptions};
use regex::Regex;
use std::env;
use std::fs::{remove_dir_all, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::string::FromUtf8Error;

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let msdf_atlas_gen_dir = out.join("msdf-atlas-gen");

    let _ = remove_dir_all(&msdf_atlas_gen_dir);

    let options = CopyOptions {
        copy_inside: true,
        ..Default::default()
    };

    copy("msdf-atlas-gen", &msdf_atlas_gen_dir, &options).unwrap();

    let cmake_lists = msdf_atlas_gen_dir.join("CMakeLists.txt");

    let contents = std::fs::read_to_string(&cmake_lists).unwrap();

    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&cmake_lists)
        .unwrap()
        .write_all(contents.as_bytes())
        .unwrap();
    let mut cmake_builder = Config::new(&msdf_atlas_gen_dir);
    cmake_builder.build_target("msdf-atlas-gen");
    cmake_builder.define("MSDF_ATLAS_BUILD_STANDALONE", "OFF");
    cmake_builder.env("VCPKG_ROOT", "./vcpkg");
    cmake_builder.profile("Release");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-lib=static=msdf-atlas-gen");

    let dst = cmake_builder.build();

    if cfg!(target_env = "msvc") {
        println!(
            "cargo:rustc-link-search=native={}/build/Release",
            dst.display()
        );
    } else {
        println!("cargo:rustc-link-search=native={}/build", dst.display());
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }

    let bindings = bindgen::Builder::default()
        .clang_arg("-Imsdf-atlas-gen")
        .clang_arg("-Imsdf-atlas-gen/msdfgen")
        .clang_arg("-x")
        .clang_arg("c++")
        .opaque_type("std::.*")
        .allowlist_type("msdfgen::.*")
        .allowlist_function("msdfgen::.*")
        .allowlist_file(".*/msdf-atlas-gen/[^/]+\\.h")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let mut replacer = Replacer::new();
    bindings
        .write(Box::new(&mut replacer))
        .expect("Couldn't write bindings to replacer!");
    replacer
        .convert()
        .expect("Couldn't convert to Utf8!")
        .replace(
            "pub type msdf_atlas_GeneratorFunction",
            "pub type msdf_atlas_GeneratorFunction<T>",
        )
        .unwrap()
        .write_to_file(out.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

struct Replacer {
    buf: Vec<u8>,
    inner: String,
}

impl Write for Replacer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.append(&mut buf.to_vec());
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Replacer {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(65535),
            inner: String::new(),
        }
    }

    pub fn convert(&mut self) -> Result<&mut Self, FromUtf8Error> {
        self.inner = String::from_utf8(self.buf.clone())?;
        Ok(self)
    }

    pub fn replace(&mut self, regex: &str, rep: &str) -> Result<&mut Self, regex::Error> {
        self.inner = Regex::new(regex)?.replace(&self.inner, rep).to_string();
        Ok(self)
    }

    pub fn write_to_file(&self, path: PathBuf) -> std::io::Result<()> {
        std::fs::write(path, self.inner.as_bytes())?;
        Ok(())
    }
}
