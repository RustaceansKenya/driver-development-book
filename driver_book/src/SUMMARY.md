# Summary

# INTRO
- [Intro](./intro.md)
- [The Roadmap](./the_roadmap.md)
- [Prerequisites for the Book]()changed


# UNDERSTANDING DRIVERS (theory)
- [What's a driver?](./whats_a_driver.md)
- [Types of Drivers]()
- [Driver Architecture and Components]()
- [Kernel and Driver Interaction]()


# BARE METAL PROGRAMMING
  - [Introduction to Operating System Kernels]()
  - [Communication between Kernel and Drivers]()
  - [Rust's Approach to Kernel Interaction]()
- [The no-std template](./the_no_std_template.md)
- [Writing linker scripts](./linker_scripts.md)
  - [pracs](./linking/pracs.md)
    - [Step one: understanding the memory layout of the board](./linking/step_1_understanding_memory_layout.md)
    - [Step one (continuation)](./linking/step_1_understanding_memory_layout_part_2.md)
    - [Runtime requirements](runtime_requirements.md)
- [Debugging](debugging.md)
  - [udev](./udev.md)
  - [probe-rs](./probe_rs.md)
  - [Logging](./logging.md)
- [No-std testing]()
- [Performance testing]()


# ABSTRACTIONS
- [MMIO programming](./mmio_programming.md)
  - [Registers and MMIO programming](./registers_and_mmio_programming.md)
- [The datasheet](./knowing_your_hardware.md)
- [Abstraction](./abstraction.md)
  - [svd2rust](./svd2rust.md)


# THE UART THEORY
- [Understanding UART Theory]()
- [Understanding UART physical Implemetation in the esp32]()

# THE UART IMPLEMENTATION (naive)
- [chapters that I dont know their titles]()


# THE UART IMPLEMENTATION (less naive)
- [Concurrency and atomic magic]()
- [chapters that I dont know their titles]()


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
