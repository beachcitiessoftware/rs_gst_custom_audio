use std::env;
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn, error, debug, trace, Level};
use tracing_subscriber;

/// Custom build script
fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    gst_plugin_version_helper::info();

    // copy the .env file to the target directory.
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let src = PathBuf::from(".env");
    let out_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let dest = PathBuf::from(&out_dir).join(format!("{}/.env", profile));

    info!("Copying {} to {}, out: {}",
        src.display().to_string(),
        dest.display().to_string(),
        out_dir
    );
    // Ensure the destination directory exists
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).expect("Failed to create output directory");
    }

    // Copy the `.env` file to the target directory
    fs::copy(&src, &dest).expect("Failed to copy .env file");
}
