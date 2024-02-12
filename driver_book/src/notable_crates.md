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

## Utility-like
- svd2rust + form + rustfmt
- defmt : A highly efficient logging framework that targets resource-constrained devices, like microcontrollers.
          Check out the defmt book at https://defmt.ferrous-systems.com for more information about how to use it.
- embassy crates
- probe crates
- clap
- ratatui
- serde

### more tertiary for now
- cfg-if  : A macro for defining #[cfg] if-else statements.
- 