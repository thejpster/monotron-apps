# Monotron Application Binary Interface

This crate allows you to write applications for the
[Monotron](https://github.com/thejpster/monotron). Think of this like the
Linux kernel syscall ABI, or the MS-DOS programming API, but designed for
applications written in Rust, better suited to a system with very few
resources and not as well developed.

The crate provides a entry pointer of the format:

```
pub static ENTRY_POINT: fn(*const Table, *mut Context) -> i32 = entry_point;
```

This pointer is placed at the bottom of the memory address range reserved for
applications (`0x2000_2000`). Monotron will jump to the function pointed to by
this pointer at application start time, and pass in a structure full of
callback pointers. These pointers can be used by the application to call
various OS functions. This crate wraps all that up into a `Host` object, on
which the application can call methods, e.g.:

```
Host::puts(b"Hello, world!");
```

If you compile for a non bare-metal target, you get an SDL2 window which
contains a pixel perfect rendering of the Monotron screen. Be aware though
that the screen only updates if you call `wfvbi()`, so do that in your main
loop.

This crate compiles as a static library which you can also link against C
applications if you prefer to use C instead of Rust - see `monotron.h` for the
C compatible declarations.
