fn main() {
    // Link against prebuilt cmsis math
    println!(
        "cargo:rustc-link-search={}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );
    println!("cargo:rustc-link-lib=static=arm_cortexM4lf_math");
}
