//! # Oxidize Title Screen
//!
//! Shows a flame animation and scrolling text.

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate monotron_app;

use monotron_app::Host;

use core::fmt::Write;
use heapless::consts::*;

const SCROLL_ROW: u8 = 10;

#[cfg(not(target_os = "none"))]
pub fn main() {
    monotron_app::Host::init();
    let r = monotron_main();
    monotron_app::Host::deinit();
    std::process::exit(r);
}

#[no_mangle]
pub extern "C" fn monotron_main() -> i32 {
    let mut buf: heapless::String<U192> = heapless::String::new();
    let _ = buf.push_str("Welcome to Oxidize - Press e to edit this string.");
    let mut f = Fire::new();
    let mut msg_start = 47isize;
    let mut frames = [0, 0, 0, 1].iter().cycle();
    loop {
        Host::wfvbi();
        Host::set_cursor_visible(false);
        Host::puts(b"\x1BW\x1Bk\x1BZ");

        Host::puts(b"\x1BR \xB2\xB2\xB2  \xB2   \xB2 \xB2 \xB2\xB2\xB2  \xB2 \xB2\xB2\xB2\xB2 \xB2\xB2\xB2\xB2\xB2 \x1BB\xB2\xB2\xB2\xB2 \xB2\xB2\xB2\xB2 \xB2 \xB2\xB2\xB2\xB2");
        Host::puts(b"\x1BR\xB2   \xB2  \xB2 \xB2  \xB2 \xB2  \xB2 \xB2    \xB2 \xB2     \x1BB   \xB2 \xB2  \xB2 \xB2 \xB2  \xB2");
        Host::puts(b"\x1BR\xB2   \xB2   \xB2   \xB2 \xB2  \xB2 \xB2   \xB2  \xB2\xB2\xB2\xB2\xB2 \x1BB\xB2\xB2\xB2\xB2 \xB2  \xB2 \xB2 \xB2\xB2\xB2\xB2");
        Host::puts(b"\x1BR\xB2   \xB2  \xB2 \xB2  \xB2 \xB2  \xB2 \xB2  \xB2   \xB2     \x1BB\xB2    \xB2  \xB2 \xB2    \xB2");
        Host::puts(b"\x1BR \xB2\xB2\xB2  \xB2   \xB2 \xB2 \xB2\xB2\xB2  \xB2 \xB2\xB2\xB2\xB2 \xB2\xB2\xB2\xB2\xB2 \x1BB\xB2\xB2\xB2\xB2 \xB2\xB2\xB2\xB2 \xB2 \xB2\xB2\xB2\xB2");

        Host::puts(b"\x1BY");

        // If we're over on the right somewhere, move across to the correct place
        let col_start = if msg_start > 0 {
            msg_start as u8
        } else {
            0
        };
        Host::move_cursor(monotron_app::Row(SCROLL_ROW), monotron_app::Col(col_start));
        Host::puts(b"\x1B^");
        // Number of bytes to skip at start of scroll-text
        let skip = if msg_start < 0 {
            msg_start.abs() as usize
        } else {
            0
        };
        let mut col = col_start;
        for b in buf.bytes().skip(skip) {
            Host::putchar(b);
            col += 1;
            if col == 48 {
                // Stop when screen is full
                break;
            }
        }
        Host::move_cursor(monotron_app::Row(SCROLL_ROW + 1), monotron_app::Col(col_start));
        Host::puts(b"\x1Bv");
        col = col_start;
        for b in buf.bytes().skip(skip) {
            Host::putchar(b);
            col += 1;
            if col == 48 {
                // Stop when screen is full
                break;
            }
        }
        // Shift left one
        let sub = frames.next().unwrap();
        msg_start -= sub;
        // Re-start scrolling next time around
        if msg_start < -(buf.len() as isize) {
            msg_start = 47;
        }

        f.draw_fire();
        if Host::kbhit() {
            match Host::readc() {
                b'q' | b'Q' => {
                    return 0;
                }
                b'e' | b'E' => {
                    edit_string(&mut buf);
                    msg_start = 47;
                }
                _ => {
                    // Ignore
                }
            }
        }
    }
}

fn edit_string<T>(buf: &mut heapless::String<T>) where T: heapless::ArrayLength<u8> {
    Host::set_cursor_visible(true);
    loop {
        Host::puts(b"\x1BW\x1BB\x1BZ");
        write!(Host, "{}", buf).unwrap();
        while !Host::kbhit() {
            Host::wfvbi();
        }
        match Host::readc() {
            27 => {
                return;
            }
            b'\r' => {
                return;
            }
            b'\n' => {
                return;
            }
            8 => {
                let _ = buf.pop();
            }
            ch if ch < 127 => {
                let _ = buf.push(ch as char);
            }
            _ => {
                Host::puts(b"Ignoring char");
                // Ignore
            }
        }
    }
}

struct Fire {
    seed: u32,
    buffer: [u32; Fire::FLAME_BUFFER_LEN]
}

impl Fire {
    const WIDTH: usize = 48;
    const HEIGHT: usize = 20;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;
    const FLAME_BUFFER_LEN: usize = Self::SIZE + Self::WIDTH + 1;

    fn new() -> Fire {
        Fire {
            seed: 123456789,
            buffer: [0u32; Self::FLAME_BUFFER_LEN]
        }
    }

    /// Draws a flame effect.
    /// Based on https://gist.github.com/msimpson/1096950.
    fn draw_fire(&mut self) {
        const CHARS: [u8; 10] = [
            b' ',
            b'.',
            b':',
            b'^',
            b'*',
            b'x',
            b's',
            b'S',
            b'#',
            b'$',
        ];
        Host::move_cursor(monotron_app::Row(16), monotron_app::Col(0));
        // Seed the fire on the last line
        for _i in 0..5 {
            let idx = (Self::WIDTH*(Self::HEIGHT-1)) + self.random_up_to(Self::WIDTH as u32) as usize;
            self.buffer[idx] = 65;
        }
        // Cascade the flames
        for i in 0..Self::SIZE {
            self.buffer[i] = (self.buffer[i] + self.buffer[i+1] + self.buffer[i+Self::WIDTH] + self.buffer[i+Self::WIDTH+1]) / 4;
            if self.buffer[i] > 15 {
                Host::puts(b"\x1BB");
            } else if self.buffer[i] > 9 {
                Host::puts(b"\x1BR");
            } else if self.buffer[i] > 4 {
                Host::puts(b"\x1BY");
            } else {
                Host::puts(b"\x1BW");
            }
            let glyph = if self.buffer[i] > 9 {
                CHARS[9]
            } else {
                CHARS[self.buffer[i] as usize]
            };
            // Don't print last glyph - makes page scroll!
            if i < Self::SIZE - 1 {
                Host::putchar(glyph);
            }
        }
    }

    /// Generates a number in the range [0, limit)
    fn random_up_to(&mut self, limit: u32) -> u32 {
        let buckets = ::core::u32::MAX / limit;
        let upper_edge = buckets * limit;
        loop {
            let attempt = self.random();
            if attempt < upper_edge {
                return attempt / buckets;
            }
        }
    }

    /// Generate a random 32-bit number
    fn random(&mut self) -> u32 {
        self.seed = (self.seed.wrapping_mul(1103515245)).wrapping_add(12345);
        self.seed
    }
}

// End of file
