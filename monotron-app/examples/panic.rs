#![no_std]
#![no_main]

extern crate monotron_app;

#[no_mangle]
pub extern "C" fn main() -> i32 {
	panic!("Test panic from user code");
}
