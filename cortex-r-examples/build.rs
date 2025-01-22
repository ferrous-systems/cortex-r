//! # Build script for the Cortex-R Examples
//!
//! This script only executes when using `cargo` to build the project.
//!
//! Copyright (c) Ferrous Systems, 2025

use std::io::Write;

static LINKER_FILE: &str = "linker.ld";
static LINKER_BYTES: &[u8] = include_bytes!("linker.ld");

fn main() {
    // Put `linker.ld` file in our output directory and ensure it's on the
    // linker search path.
    let out = &std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::File::create(out.join(LINKER_FILE))
        .unwrap()
        .write_all(LINKER_BYTES)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed={}", LINKER_FILE);
    println!("cargo:rustc-link-arg=-Tlinker.ld");
}
