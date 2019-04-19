//! # Introduction to the Monotron
//!
//! Shows a simple slideshow which introduces the viewer to the Monotron.

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate monotron_slideshow;
extern crate monotron_app;

static MATERIAL: &'static [u8] = include_bytes!("intro.md");

#[cfg(not(target_os = "none"))]
pub fn main() {
    monotron_app::Host::init();
    let r = monotron_slideshow::main(&MATERIAL);
    monotron_app::Host::deinit();
    std::process::exit(r);
}

#[no_mangle]
#[cfg(target_os = "none")]
pub extern "C" fn monotron_main() -> i32 {
    monotron_slideshow::main(&MATERIAL)
}

// End of file
