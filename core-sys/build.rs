use std::env;
use std::path::PathBuf;
fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let third_party_repo_path = manifest_dir.join("third_party/c_repo");
    println!("cargo:THIRD_PARTY_REPO_PATH={}", third_party_repo_path.to_string_lossy());
}