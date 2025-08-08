use std::env;
use std::path::PathBuf;

fn main() {
    // Format: DEP_<LIB_NAME>_INCLUDE
    // let include_path_str = env::var("DEP_THE_C_LIBRARY_INCLUDE").unwrap();
    let third_party_path = PathBuf::from(env::var("DEP_THE_C_LIBRARY_THIRD_PARTY_REPO_PATH").unwrap());
    let include_path = third_party_path.join("include");

    // Compile C code to library
    cc::Build::new()
        .file(third_party_path.join("src/cool.c"))
        .file(third_party_path.join("src/add.c"))
        .include(third_party_path.join("include"))
        .compile("user_a_lib"); // output: `libthe_c_library.a`

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", include_path.display()))
        .header(include_path.join("cool.h").to_str().unwrap())
        .header(include_path.join("add.h").to_str().unwrap()) 
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings for A-sys");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings for A-sys!");
    
    println!("cargo:rustc-link-lib=static=user_a_lib");
}