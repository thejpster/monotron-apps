//! # Chunky Fringe Show Reel
//!
//! Uses 48x36 colour mode to simulate teletext pages.

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate monotron_app;

use monotron_app::*;

#[cfg(not(target_os = "none"))]
pub fn main() {
    Host::init();
    let r = monotron_main();
    Host::deinit();
    std::process::exit(r);
}

#[no_mangle]
pub extern "C" fn monotron_main() -> i32 {
    Host::set_cursor_visible(false);
    Host::set_font(Font::Teletext).unwrap();
    for x in 10..=86 {
        for y in 18..=90 {
            set_pixel(x, y);
        }
    }
    for x in 11..=85 {
        for y in 19..=89 {
            clear_pixel(x, y);
        }
    }
    Host::move_cursor(Row(0), Col(0));
    loop {
        wfvbi();
        if Host::kbhit() {
            break;
        }
    }
    0
}

fn set_pixel(x: u8, y: u8) {
    draw_pixel(x, y, true)
}

fn clear_pixel(x: u8, y: u8) {
    draw_pixel(x, y, false)
}

fn draw_pixel(x: u8, y: u8, is_set: bool) {
    // find the cell this pixel is in
    let col = Col(x / 2);
    let row = Row(y / 3);
    let sub_col = x % 2;
    let sub_row = y % 3;
    let sub_cell = (sub_row * 2) + sub_col;
    let (ch, _attr) = Host::read_char_at(row, col);
    let bits = decode_char(ch);
    let new_bits = if is_set {
        bits | (1 << sub_cell)
    } else {
        bits & !(1 << sub_cell)
    };
    let new_ch = encode_char(new_bits);
    Host::move_cursor(row, col);
    Host::putchar(new_ch);
}

fn decode_char(ch: u8) -> u8 {
    // 128 .. 159
    // 192 .. 223
    if ch >= 128 && ch <= 159 {
        ch - 128
    } else if ch >= 192 && ch <= 223 {
        (ch - 192) + 32
    } else {
        0
    }
}

fn encode_char(bits: u8) -> u8 {
    if bits < 32 {
        bits + 128
    } else {
        (bits - 32) + 192
    }
}

// End of file
