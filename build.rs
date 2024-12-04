use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the output directory from the Cargo environment variable
    let out_dir = env::var("OUT_DIR").expect("Failed to get OUT_DIR");
    let dest_path = Path::new(&out_dir).join("joyo_kanji.csv");

    // Copy the CSV file to the build output directory
    fs::copy("joyo_kanji.csv", &dest_path)
        .expect("Failed to copy CSV file to build directory");

    // Instruct Cargo to rebuild the project if the CSV file changes
    println!("cargo:rerun-if-changed=joyo_kanji.csv");

    // Optional: Watch for changes in the `src` directory to trigger a rebuild
    println!("cargo:rerun-if-changed=src/");
}