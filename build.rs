use std::env;

fn main() {
    println!("cargo:rerun-if-changed=./build.rs");
    println!("cargo:rerun-if-changed=./src/**/*.rs");

    // Configure
    let mut config = ::cpp_build::Config::new();

    // Link
    {
        if let Ok(paths) = env::var("DEP_ORTOOLS_LIB") {
            for path in env::split_paths(&paths) {
                println!("cargo:rustc-flags=-L {}", path.display());
            }
        }
        println!("cargo:rustc-link-lib=ortools");
    }

    // Build
    if let Ok(paths) = env::var("DEP_ORTOOLS_INCLUDE") {
        for path in env::split_paths(&paths) {
            config.include(path);
        }
    }
    config
        .flag("-std=c++20")
        // Removing noise outside our jurisdiction
        .flag("-Wno-unused-parameter")
        .build("src/lib.rs");
}
