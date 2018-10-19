//! Monotron run-time application support for Rust.
//!
//! You need to supply a function with this prototype, marked `#[no_mangle]` in your crate.
//!
//! ```ignore
//! #![no_std]
//! #![no_main]
//! extern crate monotron_app;
//! #[no_mangle]
//! pub extern "C" fn main() -> i32 {
//!     123
//! }
//! ```

#![no_std]

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[link_section = ".entry_point"]
pub static ENTRY_POINT: fn(*const Context) -> i32 = entry_point;

#[repr(C)]
pub struct Context {
	p_context: *const u8
}

#[no_mangle]
pub fn entry_point(_raw_ctx: *const Context) -> i32 {
	// Do stuff with _raw_ctx

	// call the user application
	extern "C" {
		fn main() -> i32;
	}
	unsafe {
		main()
	}
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	// eprintln!("Panic: {:?}", _info);
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
