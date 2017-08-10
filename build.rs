// opentype-sanitizer-sys/build.rs
//
// Copyright © 2017 Mozilla Foundation

extern crate bindgen;

use bindgen::Builder;
use std::env;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use std::process::{Command, Stdio};

// From `mozjs`
fn find_make() -> OsString {
    if let Some(make) = env::var_os("MAKE") {
        make
    } else {
        match Command::new("gmake").status() {
            Ok(_) => OsStr::new("gmake").to_os_string(),
            Err(_) => OsStr::new("make").to_os_string(),
        }
    }
}

fn main() {
    let (out_dir, target) = (env::var("OUT_DIR").unwrap(), env::var("TARGET").unwrap());
    let make = find_make();
    let result = Command::new(make).args(&["-R", "-f", "makefile.cargo"])
                                   .stdout(Stdio::inherit())
                                   .stderr(Stdio::inherit())
                                   .status()
                                   .unwrap();
    assert!(result.success());

    println!("cargo:rustc-link-search=native={}", out_dir);
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }

    println!("cargo:rustc-link-lib=ots");
    println!("cargo:outdir={}", out_dir);

    Builder::default().clang_arg("-I./ots/include")
                      .disable_name_namespacing()
                      .whitelisted_function("ots::.*")
                      .whitelisted_type("ots::.*")
                      .whitelisted_type("Ots.*")
                      .whitelisted_type("off_t")
                      .header("wrapper.hpp")
                      .generate()
                      .unwrap()
                      .write_to_file(PathBuf::from(out_dir).join("bindings.rs"))
                      .unwrap();
}
