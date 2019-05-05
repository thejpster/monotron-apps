# Welcome to the Monotron

I am a Cortex-M4 powered home computer. My ROM
is written in the Rust Programming Language
(https://www.rust-lang.org).

I exist to show you can you take a modern
language and use it to write hard-real time
embedded systems. Plus retro computing is fun!

I drive an 800x600 VGA display through 3 SPI
peripherals at 20 MHz. I can play music,
read from a PS/2 keyboard and interface with
an SD Card.

There's also a custom PCB, which adds lots
more old-fashioned interfaces!

I have 32 KiB of RAM, with 24 KiB available
for loading programs (like this slide show).

***
# Features

Monotron has:

* A TI TM4C123 Microcontroller
* With a 80 MHz Cortex-M4F CPU
* 32 KiB SRAM (8 KiB reserved for OS)
* 256 KiB Flash ROM
* Simple C and Rust APIs for apps
* Serial Input @ 115200 bps
* 9-pin Atari Joystick interface
* 8-bit Mono Audio Output
* I2C bus for expansion
* 3-channel 4-waveform Synth
* SD Card Interface and FAT16/32 support
* PS/2 Keyboard Input*
* PS/2 Mouse Input*
* Centronics Parallel Port*
* Battery Backed Real-time Clock*
* MIDI In/Out/Thru*

 * available on the Monotron expansion PCB,
   coming soon!

Software ports so far include TinyBASIC, Snake
and a 6502 Emulator running Enhanced BASIC.
Plus this slide show viewer!

***
# Text Mode Graphics

Monotron does not have enough RAM to hold a
full framebuffer. Instead, it renders text
in real-time, racing the beam across the
screen at a scan rate of 37.5 kHz.

It has:

* 48 cols by 36 rows
* 8px by 16px characters
* 400x600 effective resolution
* 8 colours ^RR^GG^BB^C^W^kW^d^CC^MM^YY^K^wK^d
* CodePage 850 (W.Europe) character set
* 'Teletext' block graphics mode
* Support for custom fonts in RAM
* A 384x288 cell-coloured bitmap mode
* ZX Spectrum-style attribute clash ;)

A full-screen bitmap takes up 17 KiB out of
the 24 KiB spare RAM, so block graphics are
usually a better idea!

***
# Audio Output

Monotron generates 8-bit PCM samples at
37.5 kHz using Pulse-Width Modulation.

There is a three-channel software wavetable
which can produce:

* Square Waves
* Sine Waves
* Sawtooth Waves
* White Noise

***
# Memory and Applications

Monotron's 32 KiB of SRAM is divided up into:

* 512 bytes for ROM data
* 3.5 KiB for Framebuffer
* 4 KiB for the Stack
* 24 KiB for Applications

Applications receive a table of function
pointers, which form the syscall interface to
the ROM. Functions include:

* Printing to the screen
* Playing beeps
* Reading from the keyboard
* Moving the cursor
* Setting the foreground/background colour

***
# Programming

You can use TinyBASIC or 6502 BASIC directly,
but for more power, you can run C or Rust
programs compiled on any Windows/Linux PC.

## Example Code

   #![cfg_attr(target_os = "none", no_std)]
   #![cfg_attr(target_os = "none", no_main)]
   extern crate monotron_app;

   use monotron_app::prelude::*;
   use monotron_app::Host;

   #[cfg(not(target_os = "none"))]
   pub fn main() {
       std::process::exit(monotron_main());
   }

   #[no_mangle]
   pub extern "C" fn monotron_main() -> i32 {
      for _ in 0..10 {
         Host::puts("Hello, World!");
         for _ in 0..60 {
            Host::wfvbi();
         }
      }
      0
   }

***
# Learn more

## About Rust:
* ^Cgithub.com/rust-embedded^D
* ^Crust-lang.org^D

## About the Author:
* Twitter: @therealjpster
* Keybase: ^Ckeybase.io/thejpster^D

## Get the Source:
* ^Cgithub.com/thejpster/monotron^D
* ^Cgithub.com/thejpster/monotron-apps^D

***
