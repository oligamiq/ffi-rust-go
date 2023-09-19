fn main() {
    cgo_oligami::Build::new()
        .build_mode(cgo_oligami::BuildMode::CArchive)
        .change_dir("./golib")
        .package("main.go")
        .build("nosql_db_sdk_ffi");

    // println!(
    //     "cargo:rustc-link-search=native=C:/Users/nziq5/Documents/procon2023/oracle-nosql-db-sdk-rust/testc",
    // );
    // println!("cargo:rustc-link-lib=static={}", "libA");
    // println!("cargo:rustc-link-lib=static={}", "libffi_go_rust");

    println!("cargo:rerun-if-changed=golib/main.go");
}
