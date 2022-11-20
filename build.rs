extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let ggml_src = [
        "ggml/ggml.c",
    ];

    let mut builder = cc::Build::new();

    let build = builder
        .files(ggml_src.iter())
        .include("include");

    build.compile("foo");

    println!("cargo:rerun-if-changed=ggml.h");

    let bindings = bindgen::Builder::default()
        .header("ggml/ggml.h")
        .parse_callbacks(
            Box::new(bindgen::CargoCallbacks),
        )
        .allowlist_function("ggml_.*")
        .allowlist_type("ggml_.*")
        .allowlist_var("ggml_.*")
        .allowlist_file("ggml_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path =
        PathBuf::from(
            env::var("OUT_DIR").unwrap(),
        );

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}