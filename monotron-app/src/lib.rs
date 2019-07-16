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
//!
//! If you want to test your app on Linux, you'll need something like:
//!
//! ```ignore
//! #![cfg_attr(target_os = "none", no_std)]
//! #![cfg_attr(target_os = "none", no_main)]
//!
//! use monotron_app::prelude::*;
//! use monotron_app::Host;
//!
//! #[cfg(not(target_os = "none"))]
//! pub fn main() {
//!     std::process::exit(monotron_main());
//! }
//!
//! #[no_mangle]
//! pub extern "C" fn monotron_main() -> i32 {
//!     write!(Host, "Hello, Rust!\n").unwrap();
//!     0
//! }
//! ```

#![cfg_attr(target_os = "none", no_std)]
#![deny(missing_docs)]

#[cfg(not(target_os = "none"))]
use std as core;

#[cfg(not(target_os = "none"))]
extern crate sdl2;

#[cfg(not(target_os = "none"))]
extern crate vga_framebuffer;

#[cfg(not(target_os = "none"))]
extern crate lazy_static;

#[cfg(not(target_os = "none"))]
mod sdl_window;

/// Represents the Monotron we're running on. Can be passed to `write!` and
/// friends.
pub struct Host;

/// An internal representation of the context we're given by the Host.
pub struct Context;

#[derive(Debug, Clone, Copy)]
/// Represents a column on screen. Valid values are `0..=47`.
pub struct Col(pub u8);

#[derive(Debug, Clone, Copy)]
/// Represents a row on screen. Valid values are `0..=36`.
pub struct Row(pub u8);

#[derive(Debug, Clone, Copy)]
/// Represents a font we can set the screen to use. The whole screen uses the
/// same font. Custom fonts must be exactly 4096 bytes (256 chars x 16
/// bytes/char) long.
pub enum Font {
    /// Codepage 850
    Normal,
    /// ASCII with added Teletext sixel block graphics
    Teletext,
    /// A custom font
    Custom(&'static [u8]),
}

#[derive(Debug, Copy, Clone)]
/// Represents the current state of an Atari 9-pin joystick.
pub struct JoystickState(u8);

#[derive(Debug, Copy, Clone)]
/// A frequency we can give to the synthesiser.
pub struct Frequency(u32);

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
/// A channel on the synthesiser. They all run concurrently.
pub enum Channel {
    /// Channel 0
    Channel0 = 0,
    /// Channel 1
    Channel1 = 1,
    /// Channel 2
    Channel2 = 2,
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
/// A waveform on the synthesiser. You can change this note by note.
pub enum Waveform {
    /// A square wave
    Square = 0,
    /// A sine wave
    Sine = 1,
    /// A sawtooth wave
    Sawtooth = 2,
    /// A white noise (ish)
    Noise = 3,
}

impl Frequency {
    /// Convert a Frequency into centi-hz (i.e. 1 kHz is 100_000).
    pub fn as_centi_hz(&self) -> u32 {
        self.0
    }

    /// Convert a Frequency into Hz (i.e. 1 kHz is 1000).
    pub fn as_hz(&self) -> u32 {
        self.0 / 100
    }

    /// Convert a number of Hz into a Frequency.
    pub fn from_hz(hz: u16) -> Frequency {
        Frequency((hz as u32) * 100)
    }

    /// Convert a number of centi-Hz into a Frequency.
    pub fn from_centi_hz(centi_hz: u32) -> Frequency {
        Frequency(centi_hz)
    }
}

impl JoystickState {
    /// True if joystick is pointing up.
    pub fn is_up(&self) -> bool {
        (self.0 & 0b10000) != 0
    }

    /// True if joystick is pointing down.
    pub fn is_down(&self) -> bool {
        (self.0 & 0b01000) != 0
    }

    /// True if joystick is pointing left.
    pub fn is_left(&self) -> bool {
        (self.0 & 0b00100) != 0
    }

    /// True if joystick is pointing right.
    pub fn is_right(&self) -> bool {
        (self.0 & 0b00010) != 0
    }

    /// True if joystick has fire pressed.
    pub fn is_fire_pressed(&self) -> bool {
        (self.0 & 0b00001) != 0
    }
}

/// Notes on an piano keyboard, where A4 = 440 Hz.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Note {
    Rest,
    C0,
    CsDb0,
    D0,
    DsEb0,
    E0,
    F0,
    FsGb0,
    G0,
    GsAb0,
    A0,
    AsBb0,
    B0,
    C1,
    CsDb1,
    D1,
    DsEb1,
    E1,
    F1,
    FsGb1,
    G1,
    GsAb1,
    A1,
    AsBb1,
    B1,
    C2,
    CsDb2,
    D2,
    DsEb2,
    E2,
    F2,
    FsGb2,
    G2,
    GsAb2,
    A2,
    AsBb2,
    B2,
    C3,
    CsDb3,
    D3,
    DsEb3,
    E3,
    F3,
    FsGb3,
    G3,
    GsAb3,
    A3,
    AsBb3,
    B3,
    C4,
    CsDb4,
    D4,
    DsEb4,
    E4,
    F4,
    FsGb4,
    G4,
    GsAb4,
    A4,
    AsBb4,
    B4,
    C5,
    CsDb5,
    D5,
    DsEb5,
    E5,
    F5,
    FsGb5,
    G5,
    GsAb5,
    A5,
    AsBb5,
    B5,
    C6,
    CsDb6,
    D6,
    DsEb6,
    E6,
    F6,
    FsGb6,
    G6,
    GsAb6,
    A6,
    AsBb6,
    B6,
    C7,
    CsDb7,
    D7,
    DsEb7,
    E7,
    F7,
    FsGb7,
    G7,
    GsAb7,
    A7,
    AsBb7,
    B7,
    C8,
    CsDb8,
    D8,
    DsEb8,
    E8,
    F8,
    FsGb8,
    G8,
    GsAb8,
    A8,
    AsBb8,
    B8,
}

impl core::convert::Into<Frequency> for Note {
    fn into(self) -> Frequency {
        Frequency::from_centi_hz(match self {
            Note::Rest => 0,
            Note::C0 => 1635,
            Note::CsDb0 => 1732,
            Note::D0 => 1835,
            Note::DsEb0 => 1945,
            Note::E0 => 2060,
            Note::F0 => 2183,
            Note::FsGb0 => 2312,
            Note::G0 => 2450,
            Note::GsAb0 => 2596,
            Note::A0 => 2750,
            Note::AsBb0 => 2914,
            Note::B0 => 3087,
            Note::C1 => 3270,
            Note::CsDb1 => 3465,
            Note::D1 => 3671,
            Note::DsEb1 => 3889,
            Note::E1 => 4120,
            Note::F1 => 4365,
            Note::FsGb1 => 4625,
            Note::G1 => 4900,
            Note::GsAb1 => 5191,
            Note::A1 => 5500,
            Note::AsBb1 => 5827,
            Note::B1 => 6174,
            Note::C2 => 6541,
            Note::CsDb2 => 6930,
            Note::D2 => 7342,
            Note::DsEb2 => 7778,
            Note::E2 => 8241,
            Note::F2 => 8731,
            Note::FsGb2 => 9250,
            Note::G2 => 9800,
            Note::GsAb2 => 103_83,
            Note::A2 => 110_00,
            Note::AsBb2 => 116_54,
            Note::B2 => 123_47,
            Note::C3 => 130_81,
            Note::CsDb3 => 138_59,
            Note::D3 => 146_83,
            Note::DsEb3 => 155_56,
            Note::E3 => 164_81,
            Note::F3 => 174_61,
            Note::FsGb3 => 185_00,
            Note::G3 => 196_00,
            Note::GsAb3 => 207_65,
            Note::A3 => 220_00,
            Note::AsBb3 => 233_08,
            Note::B3 => 246_94,
            Note::C4 => 261_63,
            Note::CsDb4 => 277_18,
            Note::D4 => 293_66,
            Note::DsEb4 => 311_13,
            Note::E4 => 329_63,
            Note::F4 => 349_23,
            Note::FsGb4 => 369_99,
            Note::G4 => 392_00,
            Note::GsAb4 => 415_30,
            Note::A4 => 440_00,
            Note::AsBb4 => 466_16,
            Note::B4 => 493_88,
            Note::C5 => 523_25,
            Note::CsDb5 => 554_37,
            Note::D5 => 587_33,
            Note::DsEb5 => 622_25,
            Note::E5 => 659_25,
            Note::F5 => 698_46,
            Note::FsGb5 => 739_99,
            Note::G5 => 783_99,
            Note::GsAb5 => 830_61,
            Note::A5 => 880_00,
            Note::AsBb5 => 932_33,
            Note::B5 => 987_77,
            Note::C6 => 1046_50,
            Note::CsDb6 => 1108_73,
            Note::D6 => 1174_66,
            Note::DsEb6 => 1244_51,
            Note::E6 => 1318_51,
            Note::F6 => 1396_91,
            Note::FsGb6 => 1479_98,
            Note::G6 => 1567_98,
            Note::GsAb6 => 1661_22,
            Note::A6 => 1760_00,
            Note::AsBb6 => 1864_66,
            Note::B6 => 1975_53,
            Note::C7 => 2093_00,
            Note::CsDb7 => 2217_46,
            Note::D7 => 2349_32,
            Note::DsEb7 => 2489_02,
            Note::E7 => 2637_02,
            Note::F7 => 2793_83,
            Note::FsGb7 => 2959_96,
            Note::G7 => 3135_96,
            Note::GsAb7 => 3322_44,
            Note::A7 => 3520_00,
            Note::AsBb7 => 3729_31,
            Note::B7 => 3951_07,
            Note::C8 => 4186_01,
            Note::CsDb8 => 4434_92,
            Note::D8 => 4698_63,
            Note::DsEb8 => 4978_03,
            Note::E8 => 5274_04,
            Note::F8 => 5587_65,
            Note::FsGb8 => 5919_91,
            Note::G8 => 6271_93,
            Note::GsAb8 => 6644_88,
            Note::A8 => 7040_00,
            Note::AsBb8 => 7458_62,
            Note::B8 => 7902_13,
        })
    }
}

#[cfg(target_os = "none")]
/// Implementation used when building code for the Montron
pub mod target {
    use super::*;
    use monotron_api::Api;

    #[link_section = ".entry_point"]
    #[no_mangle]
    #[used]
    /// The pointer Monotron calls to start running this application.
    pub static ENTRY_POINT: fn(*const Api) -> i32 = entry_point;
    /// Pointer to the callback table we're given by the host.
    static mut TABLE_POINTER: Option<&'static Api> = None;

    #[no_mangle]
    /// The function called by the host to start us up. Does some setup, then
    /// jumps to a function called `main` defined by the actual application using
    /// this crate.
    pub fn entry_point(table: *const Api) -> i32 {
        // Turn the pointer into a reference and store in a static.
        unsafe {
            TABLE_POINTER = Some(&*table);
        };

        extern "C" {
            fn monotron_main() -> i32;
        }
        // call the user application
        unsafe { monotron_main() }
    }

    fn get_api() -> &'static Api {
        unsafe {
            if let Some(tbl) = &TABLE_POINTER {
                (tbl)
            } else {
                panic!("Bad context");
            }
        }
    }

    impl core::fmt::Write for Host {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let tbl = get_api();
            for ch in s.bytes() {
                (tbl.putchar)(ch);
            }
            Ok(())
        }
    }

    impl Host {
        /// Get the (width, height) of the Monotron TTY
        pub fn getsize() -> (u16, u16) {
            (48, 36)
        }

        /// Send a single 8-bit character to the screen.
        pub fn putchar(ch: u8) {
            let tbl = get_api();
            (tbl.putchar)(ch);
        }

        /// Send a single 8-bit character to the screen.
        pub fn puts(str8bit: &[u8]) {
            let tbl = get_api();
            for &ch in str8bit {
                (tbl.putchar)(ch);
            }
        }

        /// Return true if there is a keypress waiting (i.e. `readc` won't block).
        pub fn kbhit() -> bool {
            let tbl = get_api();
            (tbl.kbhit)() != 0
        }

        /// Read an 8-bit character from the console.
        pub fn readc() -> u8 {
            let tbl = get_api();
            (tbl.readc)() as u8
        }

        /// Wait For Vertical Blanking Interval
        pub fn wfvbi() {
            let tbl = get_api();
            (tbl.wfvbi)()
        }

        /// Move the cursor on the screen.
        pub fn move_cursor(row: Row, col: Col) {
            let tbl = get_api();
            (tbl.move_cursor)(row.0, col.0);
        }

        /// Read back what's on the screen.
        ///
        /// Returns the 8-bit glyph in the given cell, and the attribute for
        /// that cell.
        pub fn read_char_at(row: Row, col: Col) -> (u8, u8) {
            let tbl = get_api();
            let word = (tbl.read_char_at)(row.0, col.0);
            ((word >> 8) as u8, word as u8)
        }

        /// Start playing a tone. It will continue.
        pub fn play<F>(frequency: F, channel: Channel, waveform: Waveform, volume: u8)
        where
            F: Into<Frequency>,
        {
            let tbl = get_api();
            (tbl.play)(
                frequency.into().as_centi_hz(),
                channel as u8,
                waveform as u8,
                volume,
            );
        }

        /// Move the cursor on the screen.
        pub fn set_font(font: Font) -> Result<(), &'static str> {
            let tbl = get_api();
            match font {
                Font::Normal => (tbl.change_font)(0, core::ptr::null()),
                Font::Teletext => (tbl.change_font)(1, core::ptr::null()),
                Font::Custom(ram) => {
                    if ram.len() != 4096 {
                        return Err("bad font length");
                    }
                    (tbl.change_font)(2, ram.as_ptr());
                }
            }
            Ok(())
        }

        /// Get the Joystick state
        pub fn get_joystick() -> JoystickState {
            let tbl = get_api();
            let b = (tbl.get_joystick)();
            JoystickState(b)
        }

        /// Show/hide the cursor
        pub fn set_cursor_visible(visible: bool) {
            let tbl = get_api();
            (tbl.set_cursor_visible)(if visible { 1 } else { 0 });
        }

        /// Get the current calendar time. This system does not understand
        /// time zones, or leap seconds.
        pub fn gettime() -> monotron_api::Timestamp {
            monotron_api::Timestamp {
                /// The Gregorian calendar year, minus 1970 (so 10 is 1980, and 30 is the year 2000)
                year_from_1970: 49,
                /// The month of the year, where January is 1 and December is 12
                month: 7,
                /// The day of the month where 1 is the first of the month, through to 28,
                /// 29, 30 or 31 (as appropriate)
                day: 16,
                /// The hour in the day, from 0 to 23
                hour: 20,
                /// The minutes past the hour, from 0 to 59
                minute: 44,
                /// The seconds past the minute, from 0 to 59. Note that some filesystems
                /// only have 2-second precision on their timestamps.
                second: 38,
            }
        }
    }

    use core::panic::PanicInfo;
    use core::sync::atomic::{self, Ordering};

    #[inline(never)]
    #[panic_handler]
    #[cfg(all(feature = "print-panic", target_os = "none"))]
    fn panic(info: &PanicInfo) -> ! {
        use core::fmt::Write;
        // This uses about 15 KiB of our 24 KiB of RAM
        write!(
            Host,
            "\u{001B}Z\u{001B}R\u{001B}kPanic: {:?}\u{001B}W",
            info
        );
        loop {
            atomic::compiler_fence(Ordering::SeqCst);
        }
    }

    #[inline(never)]
    #[panic_handler]
    #[cfg(all(not(feature = "print-panic"), target_os = "none"))]
    fn panic(_info: &PanicInfo) -> ! {
        loop {
            atomic::compiler_fence(Ordering::SeqCst);
        }
    }

}

#[cfg(not(target_os = "none"))]
/// Implementation used when building code for Linux/Windows
pub mod target {
    use super::*;
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref VIDEO_CONTEXT: Mutex<Option<sdl_window::Context<'static>>> = Mutex::new(None);
    }

    impl std::fmt::Write for Host {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            for b in s.bytes() {
                // Ugh - this will force UTF-8 (the Rust native string format)
                // into ASCII CodePage 850, which will do bad things for
                // characters that aren't in the basic ASCII set.
                Host::putchar(b);
            }
            Ok(())
        }
    }

    impl Host {
        /// Get the (width, height) of the Monotron TTY
        pub fn getsize() -> (u16, u16) {
            (48, 36)
        }

        /// Call once at start-up to configure terminal
        pub fn init() {
            // Create SDL2 canvas and window
            *VIDEO_CONTEXT.lock().unwrap() = Some(sdl_window::Context::new());
        }

        /// Disable ncurses
        pub fn deinit() {
            // destroy SDL2 window
        }

        /// Send a single 8-bit character to the screen.
        pub fn putchar(ch: u8) {
            use vga_framebuffer::AsciiConsole;
            if let Some(ref mut ctx) = *VIDEO_CONTEXT.lock().unwrap() {
                ctx.fb.write_character(ch).unwrap();
            }
        }

        /// Send a single 8-bit character to the screen.
        pub fn puts(str8bit: &[u8]) {
            use vga_framebuffer::AsciiConsole;
            if let Some(ref mut ctx) = *VIDEO_CONTEXT.lock().unwrap() {
                for &ch in str8bit {
                    ctx.fb.write_character(ch).unwrap();
                }
            }
        }

        /// Return true if there is a keypress waiting (i.e. `readc` won't block).
        pub fn kbhit() -> bool {
            if let Some(ref mut ctx) = *VIDEO_CONTEXT.lock().unwrap() {
                ctx.keypresses.len() != 0
            } else {
                false
            }
        }

        /// Read an 8-bit character from the console.
        pub fn readc() -> u8 {
            loop {
                if let Some(ref mut ctx) = *VIDEO_CONTEXT.lock().unwrap() {
                    if let Some(ch) = ctx.keypresses.pop_front() {
                        break ch;
                    }
                }
                // Need to pump the event loop to get keypresses
                wfvbi();
            }
        }

        /// Wait For Vertical Blanking Interval
        pub fn wfvbi() {
            // redraw the screen here, as apps should call wfvbi often yes,
            // it's a kludge. It's that or we try and put the framebuffer in
            // another thread, but it's not thread-safe.
            if let Some(ref mut ctx) = *VIDEO_CONTEXT.lock().unwrap() {
                ctx.draw();
                ctx.pump();
            }
            ::std::thread::sleep(::std::time::Duration::from_micros(1_000_000 / 60));
        }

        /// Move the cursor on the screen.
        pub fn move_cursor(row: Row, col: Col) {
            use vga_framebuffer::BaseConsole;
            if col.0 as usize <= vga_framebuffer::TEXT_MAX_COL
                && row.0 as usize <= vga_framebuffer::TEXT_MAX_ROW
            {
                if let Some(ref mut ctx) = *VIDEO_CONTEXT.lock().unwrap() {
                    let p = vga_framebuffer::Position::new(
                        vga_framebuffer::Row(row.0),
                        vga_framebuffer::Col(col.0),
                    );
                    ctx.fb.set_pos(p).unwrap();
                }
            }
        }

        /// Read back what's on the screen.
        ///
        /// Returns the 8-bit glyph in the given cell, and the attribute for
        /// that cell.
        pub fn read_char_at(row: Row, col: Col) -> (u8, u8) {
            if let Some(ref mut ctx) = *VIDEO_CONTEXT.lock().unwrap() {
                let p = vga_framebuffer::Position::new(
                    vga_framebuffer::Row(row.0),
                    vga_framebuffer::Col(col.0),
                );
                if let Some((glyph, attr)) = ctx.fb.read_glyph_at(p) {
                    (glyph as u8, attr.as_u8())
                } else {
                    (0, 0)
                }
            } else {
                (0, 0)
            }
        }

        /// Start playing a tone. It will continue.
        pub fn play<F>(_frequency: F, _channel: Channel, _waveform: Waveform, _volume: u8)
        where
            F: Into<Frequency>,
        {

        }

        /// Move the cursor on the screen.
        pub fn set_font(font: Font) -> Result<(), &'static str> {
            if let Some(ref mut ctx) = *VIDEO_CONTEXT.lock().unwrap() {
                match font {
                    Font::Normal => ctx
                        .fb
                        .set_custom_font(Some(&vga_framebuffer::freebsd_cp850::FONT_DATA)),
                    Font::Teletext => ctx
                        .fb
                        .set_custom_font(Some(&vga_framebuffer::freebsd_teletext::FONT_DATA)),
                    Font::Custom(_ram) => unimplemented!(),
                }
                Ok(())
            } else {
                Err("Failed video lock")
            }
        }

        /// Get the Joystick state
        pub fn get_joystick() -> JoystickState {
            JoystickState(0)
        }

        /// Show/hide the cursor
        pub fn set_cursor_visible(visible: bool) {
            if let Some(ref mut ctx) = *VIDEO_CONTEXT.lock().unwrap() {
                ctx.fb.set_cursor_visible(visible);
            }
        }

        /// Get the current calendar time. This system does not understand
        /// time zones, or leap seconds.
        pub fn gettime() -> monotron_api::Timestamp {
            use chrono::prelude::*;
            let local: DateTime<Local> = Local::now();
            monotron_api::Timestamp {
                /// The Gregorian calendar year, minus 1970 (so 10 is 1980, and 30 is the year 2000)
                year_from_1970: (local.year() - 1970) as u8,
                /// The month of the year, where January is 1 and December is 12
                month: local.month() as u8,
                /// The day of the month where 1 is the first of the month, through to 28,
                /// 29, 30 or 31 (as appropriate)
                day: local.day() as u8,
                /// The hour in the day, from 0 to 23
                hour: local.hour() as u8,
                /// The minutes past the hour, from 0 to 59
                minute: local.minute() as u8,
                /// The seconds past the minute, from 0 to 59. Note that some filesystems
                /// only have 2-second precision on their timestamps.
                second: local.second() as u8,
            }
        }
    }
}

#[no_mangle]
/// C FFI for Host::getsize
pub extern "C" fn getsize() -> (u16, u16) {
    Host::getsize()
}

#[no_mangle]
#[cfg(not(target_os = "none"))]
/// C FFI for Host::init
pub extern "C" fn init() {
    Host::init()
}

#[no_mangle]
#[cfg(not(target_os = "none"))]
/// C FFI for Host::deinit
pub extern "C" fn deinit() {
    Host::deinit()
}

#[no_mangle]
/// C FFI for Host::putchar
pub extern "C" fn putchar(ch: u8) {
    Host::putchar(ch)
}

#[no_mangle]
/// C FFI for Host::puts
pub unsafe extern "C" fn puts(null_term_str: *const u8) {
    let mut len = 0usize;
    while *null_term_str.offset(len as isize) != 0 {
        len += 1;
    }
    Host::puts(core::slice::from_raw_parts(null_term_str, len));
}

#[no_mangle]
/// C FFI for Host::kbhit
pub extern "C" fn kbhit() -> i32 {
    if Host::kbhit() {
        1
    } else {
        0
    }
}

#[no_mangle]
/// C FFI for Host::readc
pub extern "C" fn getchar() -> u8 {
    Host::readc()
}

#[no_mangle]
/// C FFI for Host::wfvbi
pub extern "C" fn wfvbi() {
    #[cfg(not(target_os = "none"))]
    Host::wfvbi()
}

#[no_mangle]
/// C FFI for Host::move_cursor
pub extern "C" fn move_cursor(row: u8, col: u8) {
    Host::move_cursor(Row(row), Col(col))
}

#[no_mangle]
/// C FFI for Host::play
pub extern "C" fn play(frequency: u32, channel: i32, waveform: i32, volume: u8) {
    let channel = match channel {
        0 => Some(Channel::Channel0),
        1 => Some(Channel::Channel1),
        2 => Some(Channel::Channel2),
        _ => None,
    };
    let waveform = match waveform {
        0 => Some(Waveform::Square),
        1 => Some(Waveform::Sine),
        2 => Some(Waveform::Sawtooth),
        3 => Some(Waveform::Noise),
        _ => None,
    };
    if let (Some(ch), Some(wv)) = (channel, waveform) {
        Host::play(Frequency(frequency), ch, wv, volume)
    }
}

#[no_mangle]
/// C FFI for Host::set_font
pub extern "C" fn font_normal() {
    let _ = Host::set_font(Font::Normal);
}

#[no_mangle]
/// C FFI for Host::set_font
pub extern "C" fn font_teletext() {
    let _ = Host::set_font(Font::Teletext);
}

#[no_mangle]
/// C FFI for Host::get_joystick
pub extern "C" fn get_joystick() -> JoystickState {
    Host::get_joystick()
}

/// True if joystick is pointing up.
#[no_mangle]
pub extern "C" fn joystick_is_up(state: u8) -> bool {
    (state & 0b10000) != 0
}

/// True if joystick is pointing down.
#[no_mangle]
pub extern "C" fn joystick_is_down(state: u8) -> bool {
    (state & 0b01000) != 0
}

/// True if joystick is pointing left.
#[no_mangle]
pub extern "C" fn joystick_is_left(state: u8) -> bool {
    (state & 0b00100) != 0
}

/// True if joystick is pointing right.
#[no_mangle]
pub extern "C" fn joystick_is_right(state: u8) -> bool {
    (state & 0b00010) != 0
}

/// True if joystick is pointing right.
#[no_mangle]
pub extern "C" fn joystick_fire_pressed(state: u8) -> bool {
    (state & 0b00001) != 0
}

#[no_mangle]
/// Ugh
pub extern "C" fn put_separated_sixel(_char: u8) {}

#[no_mangle]
/// C FFI for Host::set_cursor_visible
pub extern "C" fn set_cursor_visible(visible: bool) {
    Host::set_cursor_visible(visible)
}

#[no_mangle]
/// C FFI for Host::gettime
pub extern "C" fn gettime() -> monotron_api::Timestamp {
    Host::gettime()
}

#[no_mangle]
/// C FFI for Host::read_char_at
pub extern "C" fn read_char_at(row: u8, col: u8) -> u16 {
    let (glyph, attr) = Host::read_char_at(Row(row), Col(col));
    ((glyph as u16) << 8) + (attr as u16)
}

#[no_mangle]
/// _sbrk is required by newlib
pub extern "C" fn _sbrk() {}

#[no_mangle]
/// _write is required by newlib
pub extern "C" fn _write() {}

#[no_mangle]
/// _close is required by newlib
pub extern "C" fn _close() {}

#[no_mangle]
/// _lseek is required by newlib
pub extern "C" fn _lseek() {}

#[no_mangle]
/// _read is required by newlib
pub extern "C" fn _read() {}

#[no_mangle]
/// _fstat is required by newlib
pub extern "C" fn _fstat() {}

#[no_mangle]
/// _isatty is required by newlib
pub extern "C" fn _isatty() {}

/// Useful things people should have in scope.
pub mod prelude {
    pub use core::fmt::Write as _monotron_prelude_core_fmt_Write;
}
