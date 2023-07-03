use cmake::Config;
use std::{env, fs};
use std::fs::{OpenOptions, remove_dir_all};
use std::io::Write;
use std::path::PathBuf;
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

    let contents = fs::read_to_string(&cmake_lists).unwrap();

    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&cmake_lists)
        .unwrap()
        .write_all(contents.as_bytes())
        .unwrap();

    let mut cmake_builder = Config::new(&msdf_atlas_gen_dir);
    cmake_builder.build_target("msdf-atlas-gen");
    cmake_builder.define("MSDF_ATLAS_NO_ARTERY_FONT", "ON");
    cmake_builder.define("MSDF_ATLAS_USE_VCPKG", "OFF");
    // TODO: tinyxml12 is a fucking bitch.
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
        .clang_arg("-x")
        .clang_arg("c++")
        .opaque_type("std::.*")
        .allowlist_type("msdf-atlas-gen::.*")
        .allowlist_function("msdf-atlas-gen::.*")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}