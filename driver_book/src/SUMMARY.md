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
  - [Pracs 1](./bare_metal/no_std/pracs_1.md)
  - [Pracs 2](./bare_metal/no_std/pracs_2.md)
- [Writing linker scripts](./linker_scripts.md)
  - [pracs](./linking/pracs.md)
    - [Step one: understanding the memory layout of the board](./linking/step_1_understanding_memory_layout.md)
    - [Step one (continuation)](./linking/step_1_understanding_memory_layout_part_2.md)
    - [Runtime requirements](runtime_requirements.md)
  - [example linker files](./linking/sample_linking_files.md)
  - [linkall.x](./linking/samples/linkall.md)
- [Debugging](debugging.md)
  - [udev](./udev.md)
  - [probe-rs](./probe_rs.md)
    - [why is my program not flashing?](flashing_problems.md)
  - [cargo-flash](./cargo-flash.md)
  - [Logging](./logging.md)
  - [defmt](./defmt.md)
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
- [Out of topic]()
  - [different_std_libs](./misc/different_std_libs.md)
