use std::env;
use std::path::PathBuf;

pub fn get_wad_dir() -> PathBuf {
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        // If the CARGO_MANIFEST_DIR environment variable is set, we're probably running with `cargo run`
        // CARGO_MANIFEST_DIR points to the directory where your Cargo.toml exists.
        PathBuf::from(manifest_dir).join("wad")
    } else {
        // Otherwise, we're probably running the program directly
        // env::current_exe() gives us the path of the current executable
        PathBuf::from(env::current_exe().unwrap().parent().unwrap()).join("assets")
    }
}
