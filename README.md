# Monotron Applications

This repo contains example applications for [Monotron]. These applications can
be compiled as ARM Cortex-M4 binaries and loaded into the Monotron's RAM via
the serial port. They can also be compiled to run on a desktop Linux PC (for
testing).

You can write Monotron applications in either C or Rust. You'll find examples
of both here.

If you build for Linux (or maybe Windows and macOS - I haven't tested), you
will get an SDL2 window. These builds pull in the same [vga-framebuffer] crate
as [Monotron] does, so you get pixel-perfect rendering. Keypress handling is a
bit hit and miss, and you do need to make sure your application calls `wfvbi`
periodically, otherwise nothing will pump the SDL2 event queue (this is a
single threaded application). PRs to make this all better would be much
appreciated.

[Monotron]: https://github.com/thejpster/monotron
[vga-framebuffer]: https://github.com/thejpster/vga-framebuffer-rs
