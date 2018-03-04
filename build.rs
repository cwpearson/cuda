// example at https://github.com/alexcrichton/git2-rs/blob/master/libgit2-sys/build.rs

extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let cuda_root = match env::var("CUDA_ROOT") {
        Err(_e) => panic!("CUDA_ROOT not found"),
        Ok(val) => PathBuf::from(val),
    };

    let cuda_link_search_path = cuda_root.join("lib64");
    let cuda_include_path = cuda_root.join("include");

    match cuda_link_search_path.to_str() {
        None => panic!("couldn't convert to string"),
        Some(val) => {
            println!("cargo:rustc-link-search=native={}", val);
            println!("cargo:rustc-link-lib=cudart");
        }
    };

    let mut bindgen_include_paths = Vec::new();
    match cuda_include_path.to_str() {
        None => panic!("couldn't convert to string"),
        Some(val) => bindgen_include_paths.push(val),
    };

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut builder = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // add include paths
        // .clang_arg("/usr/lib/gcc/x86_64-linux-gnu/5/include-fixed")
        // .clang_arg("/usr/lib/gcc/x86_64-linux-gnu/5/include")

        .opaque_type("max_align_t");

    for include_path in bindgen_include_paths {
        let mut arg = String::from("-I");
        arg.push_str(include_path);
        builder = builder.clang_arg(arg);
    }

    let bindings = builder
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    return;
}

// bindgen wrapper.h -- -I/usr/lib/gcc/x86_64-linux-gnu/5/include-fixed -I/usr/lib/gcc/x86_64-linux-gnu/5/include -I/usr/local/cuda/include >> cuda.rs
