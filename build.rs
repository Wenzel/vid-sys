extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // what library to link with
    println!(
        "cargo:rustc-link-search={}",
        project_dir.join("lib").to_string_lossy()
    );
    println!("cargo:rustc-link-lib=dylib=vid");
    let deprecated_define = if cfg!(feature = "deprecated-apis") {
        "-DVID_DEPRECATED"
    } else {
        ""
    };

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/wrapper.h")
        // deprecated APIs ?
        .clang_arg(deprecated_define)
        .allowlist_function("Vid.*")
        // specify Clang target
        .clang_arg("--target=x86_64-pc-windows-msvc")
        .rustfmt_bindings(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
