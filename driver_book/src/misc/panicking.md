# Panicking

Determine how your program reacts when it encounters an unrecoverable error that causes the program to crash.  
If you are building a satellite probe, you would want the program to crash differently from how a CLI app crashes... or how a kernel crashes... or how a web app crashes...  


Here is a good recommendation chapter : [Embedded Rust Book (Panicking Chapter)]()


- You can do conditional compilations using multiple compilation configs : eg use panic_rtt when debuging and use panic_abort when building a release binary. Check out the example below : 

```rust
#![no_main]
#![no_std]

// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
#[cfg(debug_assertions)]
use panic_halt as _;

// release profile: minimize the binary size of the application
#[cfg(not(debug_assertions))]
use panic_abort as _;

// ..
```