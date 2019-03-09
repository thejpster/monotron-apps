# Welcome to the Monotron
^t10

I am a Cortex-M4 powered home computer. My ROM
is written in the Rust Programming Language
(https://www.rust-lang.org).
^h4

I exist to show you can you take a modern
language and use it to write hard-real time
embedded systems. Plus retro computing is fun!
^h4

I drive an 800x600 VGA display through 3 SPI
peripherals at 20 MHz. I can play music,
read from a PS/2 keyboard and interface with
an SD Card.
^h4

There's also a custom PCB, which adds lots more
old-fashioned interfaces!
^h4

I have 32 KiB of RAM, with 24 KiB available for
loading programs (like this slide show).

***
# Features

Monotron has:

^h2
* A TI TM4C123 Microcontroller
^h2
* With a 80 MHz Cortex-M4F CPU
^h2
* 32 KiB SRAM (8 KiB reserved for OS)
^h2
* 256 KiB Flash ROM
^h2
* Simple C and Rust APIs for apps
^h2
* Serial Input @ 115200 bps
^h2
* 9-pin Atari Joystick interface
^h2
* 8-bit Mono Audio Output
^h2
* I2C bus for expansion
^h2
* 3-channel 4-waveform Synth
^h2
* SD Card Interface and FAT16/32 support
^h2
* PS/2 Keyboard Input*
^h2
* PS/2 Mouse Input*
^h2
* Centronics Parallel Port*
^h2
* Battery Backed Real-time Clock*
^h2
* MIDI In/Out/Thru*
^h2

 * available on the Monotron expansion PCB,
   coming soon!

^h2
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
^h2
* 8px by 16px characters
^h2
* 400x600 effective resolution
^h2
* 8 colours ^rR^gG^bB^c^K^wW^D^cC^mM^yY^kK
^h2
* CodePage 850 character set x03\x04\x05\x06
^h2
* 'Teletext' font for block graphics
^h2
* Support for custom fonts in RAM

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

^h2
* 512 bytes for ROM data
^h2
* 3.5 KiB for Framebuffer
^h2
* 4 KiB for the Stack
^h2
* 24 KiB for Applications

^h2
Applications receive a table of function
pointers, which form the syscall interface to
the ROM. Functions include:
^h2

* Printing to the screen
^h2
* Playing beeps
^h2
* Reading from the keyboard
^h2
* Moving the cursor
^h2
* Setting the foreground/background colour

***
# Learn more

## About Rust:
* ^bgithub.com/rust-embedded^d
* ^brust-lang.org^d
^h4

## About the Author:
* Twitter: @therealjpster
* Keybase: ^bkeybase.io/thejpster^d
^h4

## Get the Source:
* ^bgithub.com/thejpster/monotron^d
* ^bgithub.com/thejpster/monotron-apps^d

***
