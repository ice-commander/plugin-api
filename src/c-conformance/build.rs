fn main() {
    println!("cargo:rerun-if-changed=src/offsets.c");
    println!("cargo:rerun-if-changed=../../include/ic_plugin.h");
    cc::Build::new()
        .file("src/offsets.c")
        .include("../../include")
        .warnings(true)
        .compile("ic_c_conformance");
}
