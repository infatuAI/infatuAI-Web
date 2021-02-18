extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    // Tell cargo to tell rustc to link the cef
    // shared library.
    println!(concat!("cargo:rustc-link-search=", env!("CEF_PATH"), "/Release"));
    println!("cargo:rustc-link-lib=cef");

    // TODO: On Mac you need this...
    // println!("cargo:rustc-link-framework=cef");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")

        // Inform clang of the location of the cef headers
        .clang_arg(concat!("-I", env!("CEF_PATH")))

        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the ./bindings.rs file.
    let out_path = PathBuf::from(".");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

}
