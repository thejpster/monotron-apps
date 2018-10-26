#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate monotron_app;

use monotron_app::prelude::*;
use monotron_app::Host;


#[cfg(not(target_os = "none"))]
pub fn main() {
	std::process::exit(monotron_main());
}

#[no_mangle]
pub extern "C" fn monotron_main() -> i32 {
    // Print 5 times in 5 seconds
    for _ in 0..5 {
        write!(Host, "Hello, Rust!\n").unwrap();
        for _ in 0..60 {
            Host::wfvbi();
        }
    }
    0
}
