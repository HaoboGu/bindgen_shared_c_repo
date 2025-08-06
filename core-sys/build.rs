use std::env;
use std::path::PathBuf;
fn main() {
    let c_code_path = PathBuf::from("third_party/c_repo");

    // Compile C code to library
    cc::Build::new()
        .file(c_code_path.join("src/cool.c"))
        .file(c_code_path.join("src/add.c"))
        .include(c_code_path.join("include"))
        .compile("the_c_library"); // output: `libthe_c_library.a`

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let include_path = manifest_dir.join(&c_code_path).join("include");
    println!("cargo:include={}", include_path.display());
}