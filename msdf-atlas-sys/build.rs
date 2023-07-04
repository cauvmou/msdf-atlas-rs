use std::process::Command;
use std::{env};
use std::fs::{remove_dir_all};
use std::path::{PathBuf, Path};
use fs_extra::dir::{copy, CopyOptions};

const MAKEFILE: &'static str = "
all:
	cd msdf-atlas-gen
	mkdir -p bin
	g++ -I /usr/local/include/freetype2 -I /usr/include/freetype2 -I artery-font-format -I msdfgen/include -I msdfgen -D MSDFGEN_USE_CPP11 -D MSDF_ATLAS_STANDALONE=OFF -std=c++11 -pthread -O2 -o bin/msdf-atlas-gen msdfgen/core/*.cpp msdfgen/lib/*.cpp msdfgen/ext/*.cpp msdf-atlas-gen/*.cpp -lfreetype
";

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let msdf_atlas_gen_dir = out.join("msdf-atlas-gen");

    let _ = remove_dir_all(&msdf_atlas_gen_dir);

    let options = CopyOptions {
        copy_inside: true,
        ..Default::default()
    };

    copy("msdf-atlas-gen", &msdf_atlas_gen_dir, &options).unwrap();
    std::fs::write(msdf_atlas_gen_dir.join("Makefil"), MAKEFILE).unwrap();

    //let cmake_lists = msdf_atlas_gen_dir.join("CMakeLists.txt");
//
    //let contents = fs::read_to_string(&cmake_lists).unwrap();
//
    //OpenOptions::new()
    //    .write(true)
    //    .truncate(true)
    //    .open(&cmake_lists)
    //    .unwrap()
    //    .write_all(contents.as_bytes())
    //    .unwrap();

    //let mut cmake_builder = Config::new(&msdf_atlas_gen_dir);
    //cmake_builder.build_target("msdf-atlas-gen");
    //cmake_builder.define("MSDF_ATLAS_BUILD_STANDALONE", "OFF");
    //cmake_builder.define("MSDF_ATLAS_USE_SKIA", "OFF");
    //cmake_builder.env("VCPKG_ROOT", "./vcpkg");
    //cmake_builder.profile("Release");

    let mut make = Command::new("/usr/bin/make").current_dir(msdf_atlas_gen_dir.clone()).spawn().unwrap();
    let _status = make.wait().unwrap();

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-lib=static=msdf-atlas-gen");

    let dst = msdf_atlas_gen_dir.join("bin");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=stdc++");

    let bindings = bindgen::Builder::default()
        .clang_arg("-Imsdf-atlas-gen")
        .clang_arg("-Imsdf-atlas-gen/msdfgen")
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