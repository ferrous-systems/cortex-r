//! # Build script for the Cortex-R Examples
//!
//! This script only executes when using `cargo` to build the project.
//!
//! Copyright (c) Ferrous Systems, 2025


fn main() {
    let target = std::env::var("TARGET").expect("build script TARGET variable");

    match target.as_str() {
        "armv7r-none-eabi" => {
            // Armv7-R little-endian with soft-float ABI
            // FPU is optional
            println!("cargo:rustc-cfg=cmr_architecture=\"v7-r\"");
            println!("cargo:rustc-cfg=cmr_endian=\"little\"");
            println!("cargo:rustc-cfg=cmr_abi=\"soft-float\"");
        }
        "armv7r-none-eabihf" => {
            // Armv7-R with hard-float ABI
            // FPU is mandatory
            println!("cargo:rustc-cfg=cmr_architecture=\"v7-r\"");
            println!("cargo:rustc-cfg=cmr_endian=\"little\"");          
            println!("cargo:rustc-cfg=cmr_abi=\"hard-float\"");
        }
        "armebv7r-none-eabi" => {
            // Armv7-R big-endian with soft-float ABI
            // FPU is optional
            println!("cargo:rustc-cfg=cmr_architecture=\"v7-r\"");
            println!("cargo:rustc-cfg=cmr_endian=\"big\"");          
            println!("cargo:rustc-cfg=cmr_abi=\"soft-float\"");
        }
        "armebv7r-none-eabihf" => {
            // Armv7-R big-endian with hard-float ABI
            // FPU is mandatory
            println!("cargo:rustc-cfg=cmr_architecture=\"v7-r\"");
            println!("cargo:rustc-cfg=cmr_endian=\"big\"");          
            println!("cargo:rustc-cfg=cmr_abi=\"hard-float\"");          
        }
        "armv8r-none-eabihf" => {
            // Armv8-R little-endian with hard-float ABI
            // FPU is mandatory
            println!("cargo:rustc-cfg=cmr_architecture=\"v8-r\"");
            println!("cargo:rustc-cfg=cmr_endian=\"little\"");          
            println!("cargo:rustc-cfg=cmr_abi=\"hard-float\"");          
        }
        _ => {
            panic!("cortex-m-rt does not support TARGET {target}");
        }
    }

    println!("cargo:rustc-check-cfg=cfg(cmr_abi, values(\"soft-float\", \"hard-float\"))");
    println!("cargo:rustc-check-cfg=cfg(cmr_endian, values(\"little\", \"big\"))");
    println!("cargo:rustc-check-cfg=cfg(cmr_architecture, values(\"v7-r\", \"v8-r\"))");
}
