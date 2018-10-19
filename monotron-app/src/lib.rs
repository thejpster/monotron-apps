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

#[repr(C)]
pub struct Table {
    p_context: *const Context,
    putchar: extern "C" fn(*const Context, u8) -> i32,
    puts: extern "C" fn(*const Context, *const u8) -> i32,
    readc: extern "C" fn(*const Context) -> i32,
    wfvbi: extern "C" fn(*const Context),
    kbhit: extern "C" fn(*const Context) -> i32,
    move_cursor: extern "C" fn(*const Context, u8, u8),
    play: extern "C" fn(*const Context, u32, u8, u8, u8) -> i32,
    change_font: extern "C" fn(*const Context, u32, *const u8),
    get_joystick: extern "C" fn(*const Context) -> u8,
}

#[link_section = ".entry_point"]
#[no_mangle]
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
        for ch in s.chars() {
            if ch.is_ascii() {
                // CodePage 850 and Unicode are the same here
                (tbl.putchar)(tbl.p_context, ch as u32 as u8);
            } else {
                // TODO need a more intelligent mapping here
                (tbl.putchar)(tbl.p_context, b'?');
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Col(u8);

#[derive(Debug, Clone, Copy)]
pub struct Row(u8);

#[derive(Debug, Clone, Copy)]
pub enum Font {
    Normal,
    Teletext,
    Custom(&'static [u8]),
}

#[derive(Debug, Copy, Clone)]
pub struct JoystickState(u8);

#[derive(Debug, Copy, Clone)]
/// A frequency
pub struct Frequency(u32);

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Channel {
    Channel0 = 0,
    Channel1 = 1,
    Channel2 = 2,
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Waveform {
    Square = 0,
    Sine = 1,
    Sawtooth = 2,
    Noise = 3,
}

impl Frequency {
    pub fn as_centi_hz(&self) -> u32 {
        self.0
    }

    pub fn as_hz(&self) -> u32 {
        self.0 / 100
    }

    pub fn from_hz(hz: u16) -> Frequency {
        Frequency((hz as u32) * 100)
    }

    pub fn from_centi_hz(centi_hz: u32) -> Frequency {
        Frequency(centi_hz)
    }
}

impl Host {
    /// Send a single 8-bit character to the screen.
    pub fn putchar(ch: u8) {
        let tbl = Table::get();
        (tbl.putchar)(tbl.p_context, ch);
    }

    /// Send a single 8-bit character to the screen.
    pub fn puts(str8bit: &[u8]) {
        let tbl = Table::get();
        for &ch in str8bit {
            (tbl.putchar)(tbl.p_context, ch);
        }
    }

    /// Return true if there is a keypress waiting (i.e. `readc` won't block).
    pub fn kbhit() -> bool {
        let tbl = Table::get();
        (tbl.kbhit)(tbl.p_context) != 0
    }

    /// Read an 8-bit character from the console.
    pub fn readc() -> u8 {
        let tbl = Table::get();
        (tbl.readc)(tbl.p_context) as u8
    }

    /// Wait For Vertical Blanking Interval
    pub fn wfvbi() {
        let tbl = Table::get();
        (tbl.wfvbi)(tbl.p_context)
    }

    /// Move the cursor on the screen.
    pub fn move_cursor(row: Row, col: Col) {
        let tbl = Table::get();
        (tbl.move_cursor)(tbl.p_context, row.0, col.0);
    }

    /// Start playing a tone. It will continue.
    pub fn play<F>(frequency: F, channel: Channel, waveform: Waveform, volume: u8)
    where
        F: Into<Frequency>,
    {
        let tbl = Table::get();
        (tbl.play)(
            tbl.p_context,
            frequency.into().as_centi_hz(),
            channel as u8,
            waveform as u8,
            volume,
        );
    }

    /// Move the cursor on the screen.
    pub fn set_font(font: Font) -> Result<(), &'static str> {
        let tbl = Table::get();
        match font {
            Font::Normal => (tbl.change_font)(tbl.p_context, 0, core::ptr::null()),
            Font::Teletext => (tbl.change_font)(tbl.p_context, 1, core::ptr::null()),
            Font::Custom(ram) => {
                if ram.len() != 4096 {
                    return Err("bad font length");
                }
                (tbl.change_font)(tbl.p_context, 2, ram.as_ptr());
            }
        }
        Ok(())
    }

    /// Get the Joystick state
    pub fn get_joystick() -> JoystickState {
        let tbl = Table::get();
        let b = (tbl.get_joystick)(tbl.p_context);
        JoystickState(b)
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
            Note::GsAb2 => 10383,
            Note::A2 => 11000,
            Note::AsBb2 => 11654,
            Note::B2 => 12347,
            Note::C3 => 13081,
            Note::CsDb3 => 13859,
            Note::D3 => 14683,
            Note::DsEb3 => 15556,
            Note::E3 => 16481,
            Note::F3 => 17461,
            Note::FsGb3 => 18500,
            Note::G3 => 19600,
            Note::GsAb3 => 20765,
            Note::A3 => 22000,
            Note::AsBb3 => 23308,
            Note::B3 => 24694,
            Note::C4 => 26163,
            Note::CsDb4 => 27718,
            Note::D4 => 29366,
            Note::DsEb4 => 31113,
            Note::E4 => 32963,
            Note::F4 => 34923,
            Note::FsGb4 => 36999,
            Note::G4 => 39200,
            Note::GsAb4 => 41530,
            Note::A4 => 44000,
            Note::AsBb4 => 46616,
            Note::B4 => 49388,
            Note::C5 => 52325,
            Note::CsDb5 => 55437,
            Note::D5 => 58733,
            Note::DsEb5 => 62225,
            Note::E5 => 65925,
            Note::F5 => 69846,
            Note::FsGb5 => 73999,
            Note::G5 => 78399,
            Note::GsAb5 => 83061,
            Note::A5 => 88000,
            Note::AsBb5 => 93233,
            Note::B5 => 98777,
            Note::C6 => 104650,
            Note::CsDb6 => 110873,
            Note::D6 => 117466,
            Note::DsEb6 => 124451,
            Note::E6 => 131851,
            Note::F6 => 139691,
            Note::FsGb6 => 147998,
            Note::G6 => 156798,
            Note::GsAb6 => 166122,
            Note::A6 => 176000,
            Note::AsBb6 => 186466,
            Note::B6 => 197553,
            Note::C7 => 209300,
            Note::CsDb7 => 221746,
            Note::D7 => 234932,
            Note::DsEb7 => 248902,
            Note::E7 => 263702,
            Note::F7 => 279383,
            Note::FsGb7 => 295996,
            Note::G7 => 313596,
            Note::GsAb7 => 332244,
            Note::A7 => 352000,
            Note::AsBb7 => 372931,
            Note::B7 => 395107,
            Note::C8 => 418601,
            Note::CsDb8 => 443492,
            Note::D8 => 469863,
            Note::DsEb8 => 497803,
            Note::E8 => 527404,
            Note::F8 => 558765,
            Note::FsGb8 => 591991,
            Note::G8 => 627193,
            Note::GsAb8 => 664488,
            Note::A8 => 704000,
            Note::AsBb8 => 745862,
            Note::B8 => 790213,
        })
    }
}

#[inline(never)]
#[panic_handler]
#[cfg(feature = "print-panic")]
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
#[cfg(not(feature = "print-panic"))]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

pub mod prelude {
    pub use core::fmt::Write as _monotron_prelude_core_fmt_Write;
}
