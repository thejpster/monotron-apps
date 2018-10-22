#![no_std]
#![no_main]

use monotron_app::prelude::*;
use monotron_app::Host;

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
