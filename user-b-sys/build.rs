use std::env;
use std::path::PathBuf;

fn main() {
    // Format: DEP_<LIB_NAME>_INCLUDE
println!("cargo:warning=Available environment variables for core-sys:");
    for (key, value) in env::vars() {
        if key.starts_with("DEP_") {
            println!("cargo:warning={} = {}", key, value);
        }
    }
    let third_party_path = PathBuf::from(env::var("DEP_THE_C_LIBRARY_THIRD_PARTY_REPO_PATH").unwrap());
    let include_path = third_party_path.join("include");
    // Compile C code to library
    cc::Build::new()
        .file(third_party_path.join("src/cool.c"))
        .file(third_party_path.join("src/add.c"))
        .include(third_party_path.join("include"))
        .compile("user_b_lib");

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", include_path.display()))
        .header(include_path.join("add.h").to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings for B-sys");


    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings for B-sys!");
    
    // Forward the linked static library
    println!("cargo:rustc-link-lib=static=user_b_lib");
}