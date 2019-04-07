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
#![cfg_attr(target_os = "none", no_main)]

extern crate monotron_app;

use monotron_app::prelude::*;
use monotron_app::{Col, Host, Row};

static MATERIAL: &'static [u8] = include_bytes!("slides.md");
// static MATERIAL: &'static str = include_str!("presentation.md");

struct Context<'a> {
	page: usize,
	background: char,
	default: char,
	heading_top: char,
	heading_bottom: char,
	subheading: char,
	bullet: char,
	num_pages: usize,
	material: &'a [u8],
}

#[cfg(not(target_os = "none"))]
pub fn main() {
	Host::init();
	let r = monotron_main();
	Host::deinit();
	std::process::exit(r);
}

#[derive(Debug)]
enum Keypress {
	Timeout,
	Up,
	Down,
	Quit,
	Error
}

#[no_mangle]
pub extern "C" fn monotron_main() -> i32 {
	Host::set_cursor_visible(false);
	let mut ctx = Context {
		page: 1,
		background: 'k',
		default: 'W',
		heading_top: 'Y',
		heading_bottom: 'G',
		subheading: 'G',
		bullet: 'Y',
		material: &MATERIAL,
		num_pages: count_pages(&MATERIAL),
	};
	loop {
		let keypress = draw_page(&ctx);
		match keypress {
			Keypress::Error => {
				writeln!(Host, "Error finding page {}", ctx.page).unwrap();
				return 1;
			}
			Keypress::Quit => {
				writeln!(Host, "Quit selected").unwrap();
				return 0;
			}
			Keypress::Down | Keypress::Timeout => {
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
		None => return Keypress::Error
	};

	let mut slide_left = slide_left_default;
	let mut bullet_number = 1;
	writeln!(Host, "\u{001B}W\u{001B}k\u{001B}Z").unwrap();
	for (idx, line) in page_start.split(|c| *c == b'\n').enumerate() {
		if is_break(line) {
			if idx != 0 {
				break;
			}
		}
		else if line.starts_with(b";") {
			// Skip comments
		}
		else {
			write_line(ctx, line, true, &mut bullet_number);
		}
	}
	footer(ctx);
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
			if is_break(&ctx.material[idx+1..]) {
				pages += 1;
				if pages == ctx.page {
					result_index = Some(idx);
					break;
				}
			}
		}
	}
	if let Some(idx) = result_index {
		Some(&ctx.material[idx+1..])
	} else {
		None
	}
}

fn write_line(ctx: &Context, line: &[u8], newline: bool, bullet_number: &mut u8) {
	if line.starts_with(b"##") {
		write!(Host, "   \x1B^\x1B{}", ctx.subheading).unwrap();
		let remainder = &line[2..];
		write_plain_line(ctx, remainder, newline);
		write!(Host, "   \x1Bv\x1B{}", ctx.subheading).unwrap();
		write_plain_line(ctx, remainder, newline);
	} else if line.starts_with(b"#") {
		write!(Host, "   \x1B^\x1B{}", ctx.heading_top).unwrap();
		let remainder = &line[1..];
		write_plain_line(ctx, remainder, newline);
		write!(Host, "   \x1Bv\x1B{}", ctx.heading_bottom).unwrap();
		write_plain_line(ctx, remainder, newline);
		write!(Host, "   \x1B{} ", ctx.default).unwrap();
		let underlines = remainder.len();
		for _ in 0..underlines {
			write!(Host, "=").unwrap();
		}
		writeln!(Host, "").unwrap();
	} else if line.starts_with(b"* ") {
		let remainder = &line[2..];
		write!(Host, "     \x1B^\x1B{}\x07\x1B{}", ctx.bullet, ctx.default).unwrap();
		write_plain_line(ctx, remainder, true);
		write!(Host, "     \x1Bv\x1B{}\x07\x1B{}", ctx.bullet, ctx.default).unwrap();
		write_plain_line(ctx, remainder, true);
	} else if line.starts_with(b"1. ") {
		*bullet_number += 1;
		let remainder = &line[2..];
		write!(Host, "     \x1B^\x1B{}{}.\x1B{}", ctx.bullet, bullet_number, ctx.default).unwrap();
		write_plain_line(ctx, remainder, true);
		write!(Host, "     \x1Bv\x1B{}{}.\x1B{}", ctx.bullet, bullet_number, ctx.default).unwrap();
		write_plain_line(ctx, remainder, true);
	} else {
		write_plain_line(ctx, line, true);
	}
}

fn write_plain_line(ctx: &Context, line: &[u8], newline: bool)
{
	let mut has_escape = false;
	write!(Host, " ").unwrap();
	for &ch in line {
		if has_escape {
			match ch {
				b'^' => Host::puts(b"^"),
				b'p' => write!(Host, "{}", ctx.page).unwrap(),
				b'P' => write!(Host, "{}", ctx.num_pages).unwrap(),
				b'r' => Host::puts(b"\x1Br"),
				b'g' => Host::puts(b"\x1Bg"),
				b'b' => Host::puts(b"\x1Bb"),
				b'c' => Host::puts(b"\x1Bc"),
				b'm' => Host::puts(b"\x1Bm"),
				b'y' => Host::puts(b"\x1By"),
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
				b'D' => write!(Host, "\x1B{}", ctx.default).unwrap(),
				b'd' => write!(Host, "\x1B{}", ctx.background).unwrap(),
				b't' => {}
				_ => write!(Host, "X").unwrap(),
			}
			has_escape = false;
		} else {
			if ch == b'^' {
				has_escape = true;
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
	Host::move_cursor(Row(35), Col(0));
	write_plain_line(
		ctx,
		b"^d                                    Page ^p/^P",
		false
	);
	Host::move_cursor(Row(0), Col(0));
}

fn count_pages(contents: &[u8]) -> usize {
	contents
		.split(|c| *c == b'\n')
		.filter(|line| is_break(line))
		.count()
}

fn is_break(line: &[u8]) -> bool {
	line.starts_with(b"***") || line.starts_with(b"---") || line.starts_with(b"___")
}

// End of file
