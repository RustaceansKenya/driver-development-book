# Notable Crates

## For bare metal programming
- heapless
- critical-section
- portable-atomic
- bit-field, bitfield
- bit-flags
- embedded-hal
- embedded-dma : This library provides the ReadBuffer and WriteBuffer unsafe traits to be used as bounds to buffers types used in DMA operations.
- fugi : time crate for embedded systems
- nb : Minimal and reusable non-blocking I/O layer
- riscv
- riscv-rt
- volatile-register
- vcell : Just like Cell but with volatile read / write operations
- svd2rust
- svd2utra
- rtic
- tock-registers
- drone-svd
- loom
- crossbeam-utils : Utilities for concurrent programming
- serde
- sptr: This library provides a stable polyfill for Rust's [Strict Provenance] experimen

## Utility-like
- svd2rust + form + rustfmt
  - defmt : A highly efficient logging framework that targets resource-constrained devices, like microcontrollers.
          Check out the defmt book at https://defmt.ferrous-systems.com for more information about how to use it.
- embassy crates
- probe crates
- clap
- ratatui
- serde

## Panicking
- panic-abort. A panic causes the abort instruction to be executed.
- panic-halt. A panic causes the program, or the current thread, to halt by entering an infinite loop.
- panic-itm. The panicking message is logged using the ITM, an ARM Cortex-M specific peripheral.
- panic-semihosting. The panicking message is logged to the host using the semihosting technique.
- more here : https://crates.io/keywords/panic-handler


### more tertiary for now
- cfg-if  : A macro for defining #[cfg] if-else statements.
- 