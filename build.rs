use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let output_path = get_output_path();

    let input_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/day1test_A.in");
    let output_path = Path::new(&output_path).join("day1a.in");
    //std::fs::copy(input_path, output_path).expect("Could not copy file");
}

fn get_output_path() -> PathBuf {
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).join("target").join(build_type);
    return PathBuf::from(path);
}