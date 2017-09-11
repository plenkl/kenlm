extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("Starting to generate bindings..");
    println!("cargo:rustc-link-lib=dylib=kenlmrust");
    println!("cargo:rustc-link-search=native=../lib");
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-search=native=../build/lib");
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-I../")
        .clang_arg("-x").clang_arg("c++")
        .clang_arg("-std=c++11")
        .clang_arg("-DKENLM_MAX_ORDER=6")
        .enable_cxx_namespaces()
        .layout_tests(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    println!("Saving to bindings..");
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
