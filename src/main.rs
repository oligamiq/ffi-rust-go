extern "C" {
    fn ffi_gophernize_num();
    // fn ffi_ffi_gophernize_num();
    // fn A();
}

fn main() {
    loop {
        println!("####0");

        // unsafe { ffi_ffi_gophernize_num() };
        unsafe { ffi_gophernize_num() };
    }
}
