extern "C" {
    fn ffi_go_print();
}

fn main() {
    println!(":print on rust");
    unsafe { ffi_go_print() };

    println!("program end");
}
