#![no_std]
#![no_main]

extern crate monotron_app;

#[no_mangle]
pub extern "C" fn monotron_main() -> i32 {
	panic!("Test panic from user code");
}
