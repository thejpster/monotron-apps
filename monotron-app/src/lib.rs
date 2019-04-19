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
extern crate ncurses;

#[repr(C)]
/// The callbacks supplied by the Monotron OS.
pub struct Table {
    putchar: extern "C" fn(*const Context, u8) -> i32,
    puts: extern "C" fn(*const Context, *const u8) -> i32,
    readc: extern "C" fn(*const Context) -> i32,
    wfvbi: extern "C" fn(*const Context),
    kbhit: extern "C" fn(*const Context) -> i32,
    move_cursor: extern "C" fn(*const Context, u8, u8),
    play: extern "C" fn(*const Context, u32, u8, u8, u8) -> i32,
    change_font: extern "C" fn(*const Context, u32, *const u8),
    get_joystick: extern "C" fn(*const Context) -> u8,
    set_cursor_visible: extern "C" fn(*mut Context, u8),
}

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

#[cfg(target_os = "none")]
/// Implementation used when building code for the Montron
pub mod target {
    use super::*;

    #[link_section = ".entry_point"]
    #[no_mangle]
    #[used]
    /// The pointer Monotron calls to start running this application.
    pub static ENTRY_POINT: fn(*const Table, *mut Context) -> i32 = entry_point;
    /// Pointer to the callback table we're given by the host.
    static mut TABLE_POINTER: Option<&'static Table> = None;
    /// Pointer to the context we're given by the host.
    static mut TABLE_CONTEXT: Option<&'static mut Context> = None;

    #[no_mangle]
    /// The function called by the host to start us up. Does some setup, then
    /// jumps to a function called `main` defined by the actual application using
    /// this crate.
    pub fn entry_point(table: *const Table, ctx: *mut Context) -> i32 {
        // Turn the pointer into a reference and store in a static.
        unsafe {
            TABLE_POINTER = Some(&*table);
            TABLE_CONTEXT = Some(&mut *ctx);
        };

        extern "C" {
            fn monotron_main() -> i32;
        }
        // call the user application
        unsafe { monotron_main() }
    }

    impl Table {
        fn get() -> (&'static Table, &'static mut Context) {
            unsafe {
                if let (Some(tbl), Some(ctx)) = (&TABLE_POINTER, &mut TABLE_CONTEXT) {
                    (tbl, ctx)
                } else {
                    panic!("Bad context");
                }
            }
        }
    }

    impl core::fmt::Write for Host {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let (tbl, ctx) = Table::get();
            for ch in s.bytes() {
                (tbl.putchar)(ctx, ch);
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
            let (tbl, ctx) = Table::get();
            (tbl.putchar)(ctx, ch);
        }

        /// Send a single 8-bit character to the screen.
        pub fn puts(str8bit: &[u8]) {
            let (tbl, ctx) = Table::get();
            for &ch in str8bit {
                (tbl.putchar)(ctx, ch);
            }
        }

        /// Return true if there is a keypress waiting (i.e. `readc` won't block).
        pub fn kbhit() -> bool {
            let (tbl, ctx) = Table::get();
            (tbl.kbhit)(ctx) != 0
        }

        /// Read an 8-bit character from the console.
        pub fn readc() -> u8 {
            let (tbl, ctx) = Table::get();
            (tbl.readc)(ctx) as u8
        }

        /// Wait For Vertical Blanking Interval
        pub fn wfvbi() {
            let (tbl, ctx) = Table::get();
            (tbl.wfvbi)(ctx)
        }

        /// Move the cursor on the screen.
        pub fn move_cursor(row: Row, col: Col) {
            let (tbl, ctx) = Table::get();
            (tbl.move_cursor)(ctx, row.0, col.0);
        }

        /// Start playing a tone. It will continue.
        pub fn play<F>(frequency: F, channel: Channel, waveform: Waveform, volume: u8)
        where
            F: Into<Frequency>,
        {
            let (tbl, ctx) = Table::get();
            (tbl.play)(
                ctx,
                frequency.into().as_centi_hz(),
                channel as u8,
                waveform as u8,
                volume,
            );
        }

        /// Move the cursor on the screen.
        pub fn set_font(font: Font) -> Result<(), &'static str> {
            let (tbl, ctx) = Table::get();
            match font {
                Font::Normal => (tbl.change_font)(ctx, 0, core::ptr::null()),
                Font::Teletext => (tbl.change_font)(ctx, 1, core::ptr::null()),
                Font::Custom(ram) => {
                    if ram.len() != 4096 {
                        return Err("bad font length");
                    }
                    (tbl.change_font)(ctx, 2, ram.as_ptr());
                }
            }
            Ok(())
        }

        /// Get the Joystick state
        pub fn get_joystick() -> JoystickState {
            let (tbl, ctx) = Table::get();
            let b = (tbl.get_joystick)(ctx);
            JoystickState(b)
        }

        /// Show/hide the cursor
        pub fn set_cursor_visible(visible: bool) {
            let (tbl, ctx) = Table::get();
            (tbl.set_cursor_visible)(ctx, if visible { 1 } else { 0 });
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
    use ncurses::*;

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
            unsafe {
                libc::setlocale(libc::LC_ALL, [0].as_ptr());
            }
            initscr();
            cbreak();
            noecho();
            nodelay(stdscr(), true);
            scrollok(stdscr(), true);
            // Set up 64 colour combinations
            // The colours are RGBCMYKW in that order
            // The index is `(fg + (bg * 8)) + 1`
            start_color();
            let colors = [
                COLOR_RED,
                COLOR_GREEN,
                COLOR_BLUE,
                COLOR_CYAN,
                COLOR_MAGENTA,
                COLOR_YELLOW,
                COLOR_WHITE,
                COLOR_BLACK,
            ];
            for (fgi, fg) in colors.iter().enumerate() {
                for (bgi, bg) in colors.iter().enumerate() {
                    let pair = (fgi + (bgi * 8)) + 1;
                    init_pair(pair as i16, *fg, *bg);
                }
            }
            // White (6) on Black (7)
            attron(COLOR_PAIR(6 + (7 * 8) + 1));
            resizeterm(36, 48);
        }

        fn set_fg(fgi: i16) {
            let mut attr = 0;
            let mut pair = 0;
            attr_get(&mut attr, &mut pair);
            let bgi = (pair - 1) / 8;
            let pair = ((bgi * 8) + fgi) + 1;
            attron(COLOR_PAIR(pair));
        }

        fn set_bg(bgi: i16) {
            let mut attr = 0;
            let mut pair = 0;
            attr_get(&mut attr, &mut pair);
            let fgi = (pair - 1) & 7;
            let pair = ((bgi * 8) + fgi) + 1;
            attron(COLOR_PAIR(pair));
        }

        /// Disable ncurses
        pub fn deinit() {
            endwin();
        }

        /// Send a single 8-bit character to the screen.
        pub fn putchar(ch: u8) {
            use ncurses::*;
            use std::sync::atomic::{AtomicBool, Ordering};
            static HAVE_ESCAPE: AtomicBool = AtomicBool::new(false);
            if HAVE_ESCAPE.load(Ordering::Relaxed) {
                match ch {
                    b'Z' | b'z' => {
                        let mut attr = 0;
                        let mut pair = 0;
                        attr_get(&mut attr, &mut pair);
                        bkgd(COLOR_PAIR(pair));
                        clear();
                    }
                    b'R' => Host::set_fg(0),
                    b'G' => Host::set_fg(1),
                    b'B' => Host::set_fg(2),
                    b'C' => Host::set_fg(3),
                    b'M' => Host::set_fg(4),
                    b'Y' => Host::set_fg(5),
                    b'W' => Host::set_fg(6),
                    b'K' => Host::set_fg(7),
                    b'r' => Host::set_bg(0),
                    b'g' => Host::set_bg(1),
                    b'b' => Host::set_bg(2),
                    b'c' => Host::set_bg(3),
                    b'm' => Host::set_bg(4),
                    b'y' => Host::set_bg(5),
                    b'w' => Host::set_bg(6),
                    b'k' => Host::set_bg(7),
                    /* double height top */
                    b'^' => {}
                    /* double height bottom */
                    b'v' => {}
                    /* normal height bottom */
                    b'-' => {}
                    _ => panic!("Unsupported escape sequence {}", ch),
                }
                HAVE_ESCAPE.store(false, Ordering::Relaxed);
            } else {
                match ch {
                    b'\x1B' => {
                        HAVE_ESCAPE.store(true, Ordering::Relaxed);
                    }
                    _ => {
                        let unicode_ch = cp850_to_unicode(ch);
                        let mut buffer = [0u8; 4];
                        let result = unicode_ch.encode_utf8(&mut buffer);
                        for b in result.bytes() {
                            addch(b.into());
                        }
                    }
                }
            }
            refresh();
        }

        /// Send a single 8-bit character to the screen.
        pub fn puts(str8bit: &[u8]) {
            for &ch in str8bit {
                Host::putchar(ch);
            }
        }

        /// Return true if there is a keypress waiting (i.e. `readc` won't block).
        pub fn kbhit() -> bool {
            let ch = getch();
            if ch != ERR {
                ungetch(ch);
                true
            } else {
                false
            }
        }

        /// Read an 8-bit character from the console.
        pub fn readc() -> u8 {
            let ch = getch();
            // This is a crude conversion from local locale (UTF-8?) to Code
            // Page 850. It will produce garbage for anything that's not a
            // basic ASCII character
            ch as u8
        }

        /// Wait For Vertical Blanking Interval
        pub fn wfvbi() {
            ::std::thread::sleep(::std::time::Duration::from_micros(1_000_000 / 60));
        }

        /// Move the cursor on the screen.
        pub fn move_cursor(row: Row, col: Col) {
            wmove(stdscr(), row.0 as i32, col.0 as i32);
            refresh();
        }

        /// Start playing a tone. It will continue.
        pub fn play<F>(_frequency: F, _channel: Channel, _waveform: Waveform, _volume: u8)
        where
            F: Into<Frequency>,
        {

        }

        /// Move the cursor on the screen.
        pub fn set_font(_font: Font) -> Result<(), &'static str> {
            Ok(())
        }

        /// Get the Joystick state
        pub fn get_joystick() -> JoystickState {
            JoystickState(0)
        }

        /// Show/hide the cursor
        pub fn set_cursor_visible(visible: bool) {
            if visible {
                curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
            } else {
                curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
            }
        }
    }

}

fn cp850_to_unicode(ascii_ch: u8) -> char {
    match ascii_ch {
        0 => '\u{0000}',
        1 => '\u{263A}',
        2 => '\u{263B}',
        3 => '\u{2665}',
        4 => '\u{2666}',
        5 => '\u{2663}',
        6 => '\u{2660}',
        7 => '\u{2022}',
        8 => '\u{25D8}',
        11 => '\u{2642}',
        12 => '\u{2640}',
        14 => '\u{266B}',
        15 => '\u{263C}',
        16 => '\u{25BA}',
        17 => '\u{25C4}',
        18 => '\u{2195}',
        19 => '\u{203C}',
        20 => '\u{00B6}',
        21 => '\u{00A7}',
        22 => '\u{25AC}',
        23 => '\u{21A8}',
        24 => '\u{2191}',
        25 => '\u{2193}',
        26 => '\u{2192}',
        27 => '\u{2190}',
        28 => '\u{221F}',
        29 => '\u{2194}',
        30 => '\u{25B2}',
        31 => '\u{25BC}',
        b'\t' | b'\r' | b'\n' | 32...127 => ascii_ch.into(),
        128 => '\u{00C7}',
        129 => '\u{00FC}',
        130 => '\u{00E9}',
        131 => '\u{00E2}',
        132 => '\u{00E4}',
        133 => '\u{00E0}',
        134 => '\u{00E5}',
        135 => '\u{00E7}',
        136 => '\u{00EA}',
        137 => '\u{00EB}',
        138 => '\u{00E8}',
        139 => '\u{00EF}',
        140 => '\u{00EE}',
        141 => '\u{00EC}',
        142 => '\u{00C4}',
        143 => '\u{00C5}',
        144 => '\u{00C9}',
        145 => '\u{00E6}',
        146 => '\u{00C6}',
        147 => '\u{00F4}',
        148 => '\u{00F6}',
        149 => '\u{00F2}',
        150 => '\u{00FB}',
        151 => '\u{00F9}',
        152 => '\u{00FF}',
        153 => '\u{00D6}',
        154 => '\u{00DC}',
        155 => '\u{00F8}',
        156 => '\u{00A3}',
        157 => '\u{00D8}',
        158 => '\u{00D7}',
        159 => '\u{0192}',
        160 => '\u{00E1}',
        161 => '\u{00ED}',
        162 => '\u{00F3}',
        163 => '\u{00FA}',
        164 => '\u{00F1}',
        165 => '\u{00D1}',
        166 => '\u{00AA}',
        167 => '\u{00BA}',
        168 => '\u{00BF}',
        169 => '\u{00AE}',
        170 => '\u{00AC}',
        171 => '\u{00BD}',
        172 => '\u{00BC}',
        173 => '\u{00A1}',
        174 => '\u{00AB}',
        175 => '\u{00BB}',
        176 => '\u{2591}',
        177 => '\u{2592}',
        178 => '\u{2593}',
        179 => '\u{2502}',
        180 => '\u{2524}',
        181 => '\u{00C1}',
        182 => '\u{00C2}',
        183 => '\u{00C0}',
        184 => '\u{00A9}',
        185 => '\u{2563}',
        186 => '\u{2551}',
        187 => '\u{2557}',
        188 => '\u{255D}',
        189 => '\u{00A2}',
        190 => '\u{00A5}',
        191 => '\u{2510}',
        192 => '\u{2514}',
        193 => '\u{2534}',
        194 => '\u{252C}',
        195 => '\u{251C}',
        196 => '\u{2500}',
        197 => '\u{253C}',
        198 => '\u{00E3}',
        199 => '\u{00C3}',
        200 => '\u{255A}',
        201 => '\u{2554}',
        202 => '\u{2569}',
        203 => '\u{2566}',
        204 => '\u{2560}',
        205 => '\u{2550}',
        206 => '\u{256C}',
        207 => '\u{00A4}',
        208 => '\u{00F0}',
        209 => '\u{00D0}',
        210 => '\u{00CA}',
        211 => '\u{00CB}',
        212 => '\u{00C8}',
        213 => '\u{0131}',
        214 => '\u{00CD}',
        215 => '\u{00CE}',
        216 => '\u{00CF}',
        217 => '\u{2518}',
        218 => '\u{250C}',
        219 => '\u{2588}',
        220 => '\u{2584}',
        221 => '\u{00A6}',
        222 => '\u{00CC}',
        223 => '\u{2580}',
        224 => '\u{00D3}',
        225 => '\u{00DF}',
        226 => '\u{00D4}',
        227 => '\u{00D2}',
        228 => '\u{00F5}',
        229 => '\u{00D5}',
        230 => '\u{00B5}',
        231 => '\u{00FE}',
        232 => '\u{00DE}',
        233 => '\u{00DA}',
        234 => '\u{00DB}',
        235 => '\u{00D9}',
        236 => '\u{00FD}',
        237 => '\u{00DD}',
        238 => '\u{00AF}',
        239 => '\u{00B4}',
        240 => '\u{00AD}',
        241 => '\u{00B1}',
        242 => '\u{2017}',
        243 => '\u{00BE}',
        244 => '\u{00B6}',
        245 => '\u{00A7}',
        246 => '\u{00F7}',
        247 => '\u{00B8}',
        248 => '\u{00B0}',
        249 => '\u{00A8}',
        250 => '\u{00B7}',
        251 => '\u{00B9}',
        252 => '\u{00B3}',
        253 => '\u{00B2}',
        254 => '\u{25A0}',
        255 => '\u{00A0}',
    }
}

/// Useful things people should have in scope.
pub mod prelude {
    pub use core::fmt::Write as _monotron_prelude_core_fmt_Write;
}
