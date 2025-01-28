//! # Build script for the Cortex-R Examples
//!
//! This script only executes when using `cargo` to build the project.
//!
//! Copyright (c) Ferrous Systems, 2025

use std::io::Write;

fn main() {
    match std::env::var("TARGET").expect("TARGET not set").as_str() {
        "armv8r-none-eabihf" => {
            write("memory.x", include_bytes!("mps3-an536.ld"));
        }
        _ => {
            write("memory.x", include_bytes!("versatileab.ld"));
        }
    }
    // Use the cortex-m-rt linker script
    println!("cargo:rustc-link-arg=-Tlink.x");
}

fn write(file: &str, contents: &[u8]) {
    // Put linker file in our output directory and ensure it's on the
    // linker search path.
    let out = &std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::File::create(out.join(file))
        .unwrap()
        .write_all(contents)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed={}", file);
}
