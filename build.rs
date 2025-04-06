use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=ffi/pqm4_wrapper.c");

    let mut build = cc::Build::new();

    // Include the directory containing the ml-kem-768 API header.
    build.include("ffi/pq_crypto_code/crypto_kem/ml-kem-768/m4fspeed");
    // Include the directory where we've placed fips202.h.
    build.include("ffi/include");

    // Compile our C wrapper.
    build.file("ffi/pqm4_wrapper.c");

    // Optionally, compile all .c files in the m4fspeed directory.
    let m4fspeed_dir = Path::new("ffi/pq_crypto_code/crypto_kem/ml-kem-768/m4fspeed");
    for entry in fs::read_dir(m4fspeed_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "c" && path.file_name().unwrap() != "pqm4_wrapper.c" {
                build.file(path);
            }
        }
    }

    build.compile("pqm4");

    // Tell Cargo where to find the static library.
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=pqm4");
}
