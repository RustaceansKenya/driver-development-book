# Summary

# INTRO
- [Intro](./intro/intro.md)
- [Prerequisites for the Book](./intro/prerequisites.md) 


# UNDERSTANDING DRIVERS (theory)
- [Intro to Drivers](./understanding_drivers/understanding_drivers.md)
- [Role 1: Controlling the device below](./understanding_drivers/controlling_the_device_below.md)
- [Role 2: Providing an interface](./understanding_drivers/providing_an_interface.md)
- [Types of Drivers](./understanding_drivers/types_of_drivers.md)
- [Kernel and Driver Interaction mechanisms]()


# BARE METAL PROGRAMMING
- [No std preface](./bare_metal/the_no_std_preface.md)
  - [No std](./bare_metal/no_std/the_no_std_intro.md)
  - [Disabling the Standard Library](./bare_metal/no_std/removing_std_lib.md)
  - [Pracs 1](./bare_metal/no_std/pracs_1.md)[Title](<../../../CSC-416-4TH/main_site/Documentation_books/Developer Documentation/src/writing_a_bare_metal_rust_executable.md>)
  - [Pracs 2](./bare_metal/no_std/pracs_2.md)
- [Cross-Compilation](./bare_metal/cross_compilation/cross_compilation.md)
- [Linking](./bare_metal/linking/linking.md)
  - [Rusty Linkers](./bare_metal/linking/rusty_linkers.md)
- [Probing](./bare_metal/probing/probing_preface.md)
  - [Probing Theory](./bare_metal/probing/probing_theory_1.md)
  - [Probing Pracs](./bare_metal/probing/pracs.md)
   - [udev](./bare_metal/probing/udev.md)
   - [Flashing](./bare_metal/probing/flashing.md)
   - [Monitoring and Logging](./bare_metal/probing/logging_and_monitoring/monitoring_1.md)
     - [draft_1](./bare_metal/probing/logging_and_monitoring/monitoring_2.md)
   - [Debugging]()
  - [No-std testing]()
- [Performance testing]()


# ABSTRACTIONS
- [MMIO programming]()
  - [Registers and MMIO programming](./registers_and_mmio_programming.md)
- [The datasheet](./knowing_your_hardware.md)
- [Abstraction]()
  - [svd2rust](./svd2rust.md)


# THE UART THEORY
- [Understanding UART Theory]()
  - [draft_1](./uart_theory/draft_1.md)
  - [draft_2](./uart_theory/draft_2.md)
- [Understanding UART physical Implemetation in the esp32]()

# THE UART IMPLEMENTATION (naive, without async)
- [tutorial over a naive UART implementation on a Qemu device](./uart_implementations/on_qemu/intro.md)
  - [Setting Things Up](./uart_implementations/on_qemu/setting_things_up.md)
  - [Setting up the compiler](./uart_implementations/on_qemu/setting_up_the_compiler.md)
- [tutorial over a naive UART implementation on a Esp32 device]()


# THE UART IMPLEMENTATION (less naive, with async)
- [tutorial over a naive UART implementation on a Qemu device]()
- [tutorial over a naive UART implementation on a Esp32 device]()


# OTHER STORIES
- [Driver Security]()
  - [Common Security Issues in Driver Development]()
  - [Rust's Safety Features for Driver Security]()
  - [Best Practices for Secure Driver Development]()
- [Case Studies and Examples]()
  - [Real-world Driver Development Examples]()
  - [Analyzing an Existing Rust Driver]()
- [Iterative Implementation]()

# APPENDIX
- [Notable Crates](./notable_crates.md)
- [Notable Learning Resources]()
- [why use Rust?](./why_embedded_rust.md)
- [Out of topic]()
  - [different_std_libs](./misc/different_std_libs.md)
  - [the-C-runtime](./misc/the_C_runtime.md)
  - [api-definition](./misc/API.md)
  - [abi-definition](./misc/abi.md)
