//! # Monotron Slideshow
//!
//! This is a very basic presentation/slide-show player for
//! [Monotron](https://github.com/thejpster/monotron).
//!
//! Think of it as a very very bad Powerpoint Viewer.
//!
//! It reads in files in a special text-based format, and shows them on
//! screen.
//!
//! ## Special Characters
//!
//! Monotron Slideshow interprets certain characters to provide on-screen formatting.
//!
//! * `^` is the escape character.
//! * `^^` is a literal caret.
//! * `^p` is the page number.
//! * `^P` is the total number of pages.
//! * `^[rgbcmykwd]` sets the text colour to
//!   Red/Green/Blue/Cyan/Magenta/Yellow/Black/White/Default respectively.
//! * `^[RGBCMYKWD]` sets the background colour to
//!   Red/Green/Blue/Cyan/Magenta/Yellow/Black/White/Default respectively.
//! * `*****` sets an end-of-page marker where the next page slides up.
//! * `^t<nn>` sets a page timeout of NN seconds for this and subsequent
//!   pages. 0 means no timeout.
//! * `# <text>` is a heading (double-height text, underlined)
//! * `## <text>` is a sub-heading (underlined)

#![cfg_attr(target_os = "none", no_std)]

extern crate monotron_app;

use monotron_app::{Col, Host, Row};

static UNDERLINE: &'static [u8] = b"===============================================";

pub struct Context<'a> {
    page: u8,
    background: u8,
    default: u8,
    title_top: u8,
    title_bottom: u8,
    heading_top: u8,
    heading_bottom: u8,
    subheading: u8,
    bullet: u8,
    num_pages: u8,
    material: &'a [u8],
    footer: &'a [u8],
    centre: core::cell::RefCell<bool>,
}

#[derive(Debug)]
pub enum Keypress {
    Timeout,
    Up,
    Down,
    Quit,
    Error,
    Nothing,
}

enum Padding {
    ZeroPad(u8),
    NoPad,
}

pub fn main(material: &[u8], footer: &[u8]) -> i32 {
    Host::set_cursor_visible(false);
    let mut ctx = Context {
        page: 1,
        background: b'k',
        default: b'W',
        title_top: b'Y',
        title_bottom: b'C',
        heading_top: b'Y',
        heading_bottom: b'G',
        subheading: b'G',
        bullet: b'Y',
        material: material,
        num_pages: count_pages(material),
        footer,
        centre: core::cell::RefCell::new(false),
    };
    loop {
        let keypress = draw_page(&ctx);
        match keypress {
            Keypress::Error => {
                Host::puts(b"Error finding page!");
                return 1;
            }
            Keypress::Quit => {
                Host::puts(b"\x1BW\x1Bk\x1BZ");
                Host::puts(b"Thanks for watching!\n");
                return 0;
            }
            Keypress::Nothing | Keypress::Down | Keypress::Timeout => {
                if ctx.page < ctx.num_pages {
                    ctx.page += 1;
                }
            }
            Keypress::Up => {
                if ctx.page > 1 {
                    ctx.page -= 1;
                }
            }
        }
    }
}

fn draw_page(ctx: &Context) -> Keypress {
    // Skip through material to find page
    let slide_left_default = -1;
    let page_start = match find_page(ctx) {
        Some(p) => p,
        None => return Keypress::Error,
    };
    let mut bullet_number = b'1';
    Host::puts(b"\x1BW\x1Bk\x1BZ");
    *ctx.centre.borrow_mut() = false;
    for (idx, line) in page_start.split(|c| *c == b'\n').enumerate() {
        if is_break(line) {
            if idx != 0 {
                break;
            }
        } else if line.starts_with(b";") {
            // Skip comments
        } else {
            match write_line(ctx, line, slide_left_default, &mut bullet_number) {
                Keypress::Quit => return Keypress::Quit,
                Keypress::Up => return Keypress::Up,
                Keypress::Error => return Keypress::Error,
                Keypress::Nothing | Keypress::Timeout | Keypress::Down => {}
            }
        }
    }
    wait_for_key(ctx, slide_left_default)
}

fn wait_for_key(ctx: &Context, slide_left: i32) -> Keypress {
    let mut slide_left = slide_left;
    loop {
        if Host::kbhit() {
            match Host::readc() {
                // Quit program
                b'q' => return Keypress::Quit,
                // Prev page now
                b'p' => return Keypress::Up,
                // Next page now
                b' ' | b'n' => return Keypress::Down,
                // Ignore anything else
                _ => {}
            }
        }
        if slide_left == 0 {
            // Next page
            return Keypress::Timeout;
        } else {
            // Wait for a frame
            Host::wfvbi();
            // Update the timestamp
            footer(ctx);
            if slide_left > 0 {
                slide_left = slide_left - 1;
            }
        }
    }
}

fn find_page<'a>(ctx: &'a Context) -> Option<&'a [u8]> {
    let mut pages = 1;
    let mut result_index = None;
    if ctx.page == 1 {
        // Special case the first page
        return Some(&ctx.material);
    }
    // Don't use .split() here as we want a pointer
    // to the whole page, not just the next line.
    for (idx, ch) in ctx.material.iter().enumerate() {
        if *ch == b'\n' {
            if is_break(&ctx.material[idx + 1..]) {
                pages += 1;
                if pages == ctx.page {
                    result_index = Some(idx);
                    break;
                }
            }
        }
    }
    if let Some(idx) = result_index {
        Some(&ctx.material[idx + 1..])
    } else {
        None
    }
}

fn write_line(
    ctx: &Context,
    line: &[u8],
    slide_left_default: i32,
    bullet_number: &mut u8,
) -> Keypress {
    if line.starts_with(b"### ") {
        let res = wait_for_key(ctx, slide_left_default);
        Host::puts(b"\x1B^   \x1B");
        Host::putchar(ctx.subheading);
        let remainder = &line[4..];
        write_plain_line(ctx, remainder, true);
        Host::puts(b"\x1Bv   \x1B");
        Host::putchar(ctx.subheading);
        write_plain_line(ctx, remainder, true);
        Host::putchar(b'\x1B');
        Host::putchar(ctx.default);
        res
    } else if line.starts_with(b"## ") {
        Host::puts(b"\x1B^   \x1B");
        Host::putchar(ctx.heading_top);
        let remainder = &line[3..];
        write_plain_line(ctx, remainder, true);
        Host::puts(b"\x1Bv   \x1B");
        Host::putchar(ctx.heading_bottom);
        write_plain_line(ctx, remainder, true);
        Host::puts(b"   \x1B");
        Host::putchar(ctx.default);
        write_plain_line(ctx, &UNDERLINE[0..remainder.len()], true);
        Keypress::Nothing
    } else if line.starts_with(b"# ") {
        for _ in 0..13 {
            Host::putchar(b'\n');
        }
        Host::puts(b"\x1B^   \x1B");
        Host::putchar(ctx.title_top);
        let remainder = &line[2..];
        write_plain_line(ctx, remainder, true);
        Host::puts(b"\x1Bv   \x1B");
        Host::putchar(ctx.title_bottom);
        write_plain_line(ctx, remainder, true);
        Host::puts(b"   \x1B");
        Host::putchar(ctx.default);
        write_plain_line(ctx, &UNDERLINE[0..remainder.len()], true);
        Keypress::Nothing
    } else if line.starts_with(b"* ") {
        let res = wait_for_key(ctx, slide_left_default);
        let remainder = &line[2..];
        Host::puts(b"     \x1B^\x1B");
        Host::putchar(ctx.bullet);
        Host::puts(b"\x07\x1B");
        Host::putchar(ctx.default);
        write_plain_line(ctx, remainder, true);
        Host::puts(b"     \x1Bv\x1B");
        Host::putchar(ctx.bullet);
        Host::puts(b"\x07\x1B");
        Host::putchar(ctx.default);
        write_plain_line(ctx, remainder, true);
        Host::putchar(b'\n');
        res
    } else if line.starts_with(b"1. ") {
        let res = wait_for_key(ctx, slide_left_default);
        let remainder = &line[2..];
        Host::puts(b"     \x1B^\x1B");
        Host::putchar(ctx.bullet);
        Host::putchar(*bullet_number);
        Host::puts(b".\x1B");
        Host::putchar(ctx.default);
        write_plain_line(ctx, remainder, true);
        Host::puts(b"     \x1Bv\x1B");
        Host::putchar(ctx.bullet);
        Host::putchar(*bullet_number);
        Host::puts(b".\x1B");
        Host::putchar(ctx.default);
        write_plain_line(ctx, remainder, true);
        Host::putchar(b'\n');
        *bullet_number += 1;
        res
    } else {
        write_plain_line(ctx, line, true);
        Keypress::Nothing
    }
}

fn write_plain_line(ctx: &Context, line: &[u8], newline: bool) {
    let mut has_escape = false;
    let mut is_rle = 0;
    let mut rle_count = 0;
    let padding = if *ctx.centre.borrow() == true {
        let mut length: i32 = 0;
        for &ch in line {
            if has_escape {
                if ch == b'n' {
                    // special RLE
                    is_rle = 1;
                }
                has_escape = false;
            } else if is_rle == 1 {
                length += ch as i32 - 33;
                is_rle = 0;
                has_escape = false;
            } else {
                if ch == b'^' {
                    has_escape = true;
                } else {
                    length += 1;
                }
            }
        }
        let width = 48;
        (width - length) / 2
    } else {
        1
    };
    for _ in 0..padding {
        Host::putchar(b' ');
    }
    for &ch in line {
        if is_rle == 1 {
            rle_count = ch - 32;
            is_rle = 2;
        } else if has_escape {
            match ch {
                b'^' => Host::puts(b"^"),
                b'p' => print_num(ctx.page as u16, Padding::NoPad),
                b'P' => print_num(ctx.num_pages as u16, Padding::NoPad),
                b'r' => Host::puts(b"\x1Br"),
                b'g' => Host::puts(b"\x1Bg"),
                b'b' => Host::puts(b"\x1Bb"),
                b'c' => Host::puts(b"\x1Bc"),
                b'm' => Host::puts(b"\x1Bm"),
                b'y' => Host::puts(b"\x1By"),
                b'n' => is_rle = 1,
                b'k' => Host::puts(b"\x1Bk"),
                b'w' => Host::puts(b"\x1Bw"),
                b'R' => Host::puts(b"\x1BR"),
                b'G' => Host::puts(b"\x1BG"),
                b'B' => Host::puts(b"\x1BB"),
                b'C' => Host::puts(b"\x1BC"),
                b'M' => Host::puts(b"\x1BM"),
                b'Y' => Host::puts(b"\x1BY"),
                b'K' => Host::puts(b"\x1BK"),
                b'W' => Host::puts(b"\x1BW"),
                b'D' => {
                    Host::putchar(0x1B);
                    Host::putchar(ctx.default);
                }
                b'd' => {
                    Host::putchar(0x1B);
                    Host::putchar(ctx.background);
                }
                b'e' => {
                    *ctx.centre.borrow_mut() = true;
                }
                b'E' => {
                    *ctx.centre.borrow_mut() = false;
                }
                _ => Host::putchar(b'X'),
            }
            has_escape = false;
        } else {
            if ch == b'^' {
                has_escape = true;
            } else if is_rle == 2 {
                for _ in 0..rle_count {
                    Host::putchar(ch);
                }
                rle_count = 0;
                is_rle = 0;
            } else {
                Host::putchar(ch);
            }
        }
    }
    if newline {
        Host::putchar(b'\n');
    }
}

fn footer(ctx: &Context) {
    let old_centre = *ctx.centre.borrow();
    *ctx.centre.borrow_mut() = false;
    let (row, col) = Host::get_cursor();
    Host::move_cursor(Row(35), Col(0));
    write_plain_line(ctx, ctx.footer, false);
    Host::move_cursor(Row(35), Col(21));
    let date_time = Host::gettime();
    print_num(date_time.year_from_1970 as u16 + 1970, Padding::ZeroPad(4));
    Host::putchar(b'-');
    print_num(date_time.month as u16, Padding::ZeroPad(2));
    Host::putchar(b'-');
    print_num(date_time.days as u16, Padding::ZeroPad(2));
    Host::putchar(b' ');
    print_num(date_time.hours as u16, Padding::ZeroPad(2));
    Host::putchar(b':');
    print_num(date_time.minutes as u16, Padding::ZeroPad(2));
    Host::putchar(b':');
    print_num(date_time.seconds as u16, Padding::ZeroPad(2));
    write_plain_line(ctx, b" ^p/^P", false);
    Host::move_cursor(row, col);
    *ctx.centre.borrow_mut() = old_centre;
}

fn count_pages(contents: &[u8]) -> u8 {
    contents
        .split(|c| *c == b'\n')
        .filter(|line| is_break(line))
        .count() as u8
}

fn is_break(line: &[u8]) -> bool {
    line.starts_with(b"***") || line.starts_with(b"---") || line.starts_with(b"___")
}

fn print_num(number: u16, padding: Padding) {
    let mut number = number;
    let mut padding_count = match padding {
        Padding::ZeroPad(n) => n,
        Padding::NoPad => 0,
    };
    if number >= 1000 {
        let thousands = number / 1000;
        Host::putchar(b'0' + thousands as u8);
        number = number - (thousands * 1000);
        padding_count = 4;
    } else if padding_count >= 4 {
        Host::putchar(b'0');
    }
    if number >= 100 {
        let hundreds = number / 100;
        Host::putchar(b'0' + hundreds as u8);
        number = number - (hundreds * 100);
        padding_count = 3;
    } else if padding_count >= 3 {
        Host::putchar(b'0');
    }
    if number >= 10 {
        let tens = number / 10;
        Host::putchar(b'0' + tens as u8);
        number = number - (tens * 10);
    } else if padding_count >= 2 {
        Host::putchar(b'0');
    }
    Host::putchar(b'0' + number as u8);
}

// End of file
