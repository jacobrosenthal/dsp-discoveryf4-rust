fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=libarm_cortexM4lf_math.a");

    // Link against prebuilt cmsis math
    println!(
        "cargo:rustc-link-search={}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );
    println!("cargo:rustc-link-lib=static=arm_cortexM4lf_math");
}
