use std::io::Write;
use std::{env};
use std::fs::{remove_dir_all, OpenOptions};
use std::path::{PathBuf};
use cmake::Config;
use fs_extra::dir::{copy, CopyOptions};

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

    // debug_print(format!("{msdf_atlas_gen_dir:?}"));

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-lib=static=msdf-atlas-gen");

    let dst = cmake_builder.build();
    
    // debug_print(format!("{dst:?}"));

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
        // TODO: Make correct allowlist: .allowlist_type("msdf-atlas-gen::.*")
        // TODO: Make correct allowlist: .allowlist_function("msdf-atlas-gen::.*")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("./src/bindings.rs")
        .expect("Couldn't write bindings!");
}

#[allow(dead_code)]
fn debug_print(s: String) {
    match std::fs::read_to_string("./debug.log") {
        Ok(content) => std::fs::write("./debug.log", format!("{content}\n{s}")).unwrap(),
        Err(_) => std::fs::write("./debug.log", s).unwrap(),
    }
}