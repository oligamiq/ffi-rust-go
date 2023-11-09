fn main() {
    // println!("cargo:rustc-cfg=link_dynamic");

    cgo_oligami::Build::new()
        .build_mode(cgo_oligami::BuildMode::CArchive)
        .change_dir("./golib")
        .package("main.go")
        .build("ffi_go_print");

    println!("cargo:rerun-if-changed=golib/main.go");
}
