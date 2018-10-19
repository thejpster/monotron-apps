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

use core::fmt::Write;
use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[repr(C)]
pub struct Table {
    p_context: *const Context,
    putchar: extern "C" fn(*const Context, u8) -> i32,
    puts: extern "C" fn(*const Context, *const u8) -> i32,
    readc: extern "C" fn(*const Context) -> i32,
    wfvbi: extern "C" fn(*const Context),
    kbhit: extern "C" fn(*const Context) -> i32,
    move_cursor: extern "C" fn(*const Context, u8, u8),
    play: extern "C" fn (*const Context, u32, u8, u8, u8) -> i32,
    change_font: extern "C" fn (*const Context, u32, *const u8),
    get_joystick: extern "C" fn (*const Context) -> u8
}

#[link_section = ".entry_point"]
pub static ENTRY_POINT: fn(*const Table) -> i32 = entry_point;
pub struct Host;

struct Context;

static mut TABLE_POINTER: Option<&'static Table> = None;

#[no_mangle]
pub fn entry_point(raw_ctx: *const Table) -> i32 {
    // Turn the pointer into a reference and store in a static.
    unsafe {
        let ctx = &*raw_ctx;
        TABLE_POINTER = Some(ctx);
    };

    extern "C" {
        fn main() -> i32;
    }
    // call the user application
    unsafe { main() }
}

impl Table {
    fn get() -> &'static Table {
        unsafe {
            if let Some(tbl) = &TABLE_POINTER {
                tbl
            } else {
                panic!("Bad context");
            }
        }
    }
}

impl core::fmt::Write for Host {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let tbl = Table::get();
        for b in s.bytes() {
            (tbl.putchar)(tbl.p_context, b);
        }
        Ok(())
    }
}

impl Host {
	pub fn kbhit() -> bool {
	    let tbl = Table::get();
		(tbl.kbhit)(tbl.p_context) != 0
	}

	pub fn readc() -> i32 {
	    let tbl = Table::get();
		(tbl.readc)(tbl.p_context)
	}

	pub fn wfvbi() {
	    let tbl = Table::get();
		(tbl.wfvbi)(tbl.p_context)
	}
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // This uses a *LOT* of our 24 KiB of RAM
    // write!(Host, "\u{001B}Z\u{001B}R\u{001B}kPanic: {:?}\u{001B}W", _info);
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

pub mod prelude {
    pub use core::fmt::Write as _monotron_prelude_core_fmt_Write;
}
