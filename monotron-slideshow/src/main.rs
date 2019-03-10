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

static MATERIAL: &'static str = include_str!("slides.md");
// static MATERIAL: &'static str = include_str!("presentation.md");

struct Context {
	page: usize,
	background: char,
	default: char,
	heading_top: char,
	heading_bottom: char,
	subheading: char,
	bullet: char,
	num_pages: usize,
}

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
	loop {
		let mut ctx = Context {
			page: 1,
			background: 'k',
			default: 'W',
			heading_top: 'Y',
			heading_bottom: 'G',
			subheading: 'G',
			bullet: 'R',
			num_pages: count_pages(&MATERIAL),
		};
		// Set BG and Clear screen
		write!(Host, "\x1B{}\x1BZ", ctx.background).unwrap();
		footer(&ctx);
		let mut slide_left_default = -1;
		// Loop through the input
		for line in MATERIAL.lines() {
			if line.starts_with("^h") {
				// In-page hold
				let num = line[2..].parse::<i32>().unwrap();
				for _ in 0..num * 60 {
					if Host::kbhit() {
						match Host::readc() {
							// Quit program
							b'q' => return 0,
							// Next page now
							b' ' => break,
							// Ignore anything else
							_ => {}
						}
					} else {
						// Wait for a frame
						Host::wfvbi();
					}
				}
				continue;
			}
			if line.starts_with("^t") {
				// Handle page timeouts
				let num = line[2..].parse::<i32>().unwrap();
				slide_left_default = num * 60;
				continue;
			} else if line.starts_with("***") || line.starts_with("---") || line.starts_with("___")
			{
				let mut slide_left = slide_left_default;
				loop {
					if Host::kbhit() {
						match Host::readc() {
							// Quit program
							b'q' => return 0,
							// Next page now
							b' ' => slide_left = 0,
							// Ignore anything else
							_ => {}
						}
					}
					if slide_left == 0 {
						// Next page
						ctx.page += 1;
						// Clear screen
						for _ in 0..48 {
							writeln!(Host, "").unwrap();
							Host::wfvbi();
							Host::wfvbi();
						}
						write!(Host, "\x1BZ").unwrap();
						break;
					} else {
						// Wait for a frame
						Host::wfvbi();
						if slide_left > 0 {
							slide_left = slide_left - 1;
						}
					}
				}
				footer(&ctx);
			} else {
				// Normal output
				write_line(&ctx, line, true);
			}
		}
	}
}

fn write_line(ctx: &Context, line: &str, newline: bool) {
	let mut has_escape = false;
	if line.starts_with("##") {
		write!(Host, "\x1B{}", ctx.subheading).unwrap();
		let remainder = &line[2..].trim();
		write_line(ctx, remainder, newline);
		write!(Host, "\x1B{} ", ctx.default).unwrap();
		let underlines = remainder.len();
		for _ in 0..underlines {
			write!(Host, "=").unwrap();
		}
		writeln!(Host, "").unwrap();
	} else if line.starts_with("#") {
		write!(Host, "\x1B^\x1B{}", ctx.heading_top).unwrap();
		let remainder = &line[1..].trim();
		write_line(ctx, remainder, newline);
		write!(Host, "\x1Bv\x1B{}", ctx.heading_bottom).unwrap();
		write_line(ctx, remainder, newline);
		write!(Host, "\x1B{} ", ctx.default).unwrap();
		let underlines = remainder.len();
		for _ in 0..underlines {
			write!(Host, "=").unwrap();
		}
		writeln!(Host, "").unwrap();
	} else if line.starts_with("* ") {
		write!(Host, "  \x1B{}\x07\x1B{}", ctx.bullet, ctx.default).unwrap();
		write_line(&ctx, &line[2..], true);
	} else {
		write!(Host, " ").unwrap();
		for ch in line.chars() {
			if has_escape {
				match ch {
					'^' => write!(Host, "^").unwrap(),
					'p' => write!(Host, "{}", ctx.page).unwrap(),
					'P' => write!(Host, "{}", ctx.num_pages).unwrap(),
					'r' => write!(Host, "\x1Br").unwrap(),
					'g' => write!(Host, "\x1Bg").unwrap(),
					'b' => write!(Host, "\x1Bb").unwrap(),
					'c' => write!(Host, "\x1Bc").unwrap(),
					'm' => write!(Host, "\x1Bm").unwrap(),
					'y' => write!(Host, "\x1By").unwrap(),
					'k' => write!(Host, "\x1Bk").unwrap(),
					'w' => write!(Host, "\x1Bw").unwrap(),
					'R' => write!(Host, "\x1BR").unwrap(),
					'G' => write!(Host, "\x1BG").unwrap(),
					'B' => write!(Host, "\x1BB").unwrap(),
					'C' => write!(Host, "\x1BC").unwrap(),
					'M' => write!(Host, "\x1BM").unwrap(),
					'Y' => write!(Host, "\x1BY").unwrap(),
					'K' => write!(Host, "\x1BK").unwrap(),
					'W' => write!(Host, "\x1BW").unwrap(),
					'D' => write!(Host, "\x1B{}", ctx.default).unwrap(),
					'd' => write!(Host, "\x1B{}", ctx.background).unwrap(),
					't' => {}
					_ => write!(Host, "X").unwrap(),
				}
				has_escape = false;
			} else {
				if ch == '^' {
					has_escape = true;
				} else {
					write!(Host, "{}", ch).unwrap();
				}
			}
		}
		if newline {
			write!(Host, "\n").unwrap();
		}
	}
}

fn footer(ctx: &Context) {
	Host::move_cursor(Row(35), Col(0));
	write_line(
		&ctx,
		"^d                                    Page ^p/^P",
		false,
	);
	Host::move_cursor(Row(0), Col(0));
}

fn count_pages(contents: &str) -> usize {
	contents
		.lines()
		.filter(|line| {
			line.starts_with("***") || line.starts_with("---") || line.starts_with("___")
		})
		.count()
}

// End of file
