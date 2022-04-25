#![deny(warnings)]

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/errno.c");
    cc::Build::new().file("src/errno.c").compile("lib_rust_errno_sys.a");
}
