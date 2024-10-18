fn main() {
    println!("cargo:rustc-link-search=native={}", std::env::var("ZLIB_LIB_DIR").unwrap());
    println!("cargo:rustc-link-search=native={}", std::env::var("CFITSIO_LIB_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=z");
    println!("cargo:rustc-link-lib=static=cfitsio");
    println!("cargo:rerun-if-env-changed=ZLIB_LIB_DIR");
    println!("cargo:rerun-if-env-changed=CFITSIO_LIB_DIR");
}