use std::env;
use std::path::PathBuf;

fn main() {
    // Format: DEP_<LIB_NAME>_INCLUDE
    let include_path_str = env::var("DEP_THE_C_LIBRARY_INCLUDE").unwrap();
    let include_path = PathBuf::from(include_path_str);

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
    println!("cargo:rustc-link-lib=static=the_c_library");
}