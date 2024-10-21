# Summary

# INTRO
- [Intro](./intro/intro.md)
- [Prerequisites for the Book](./intro/prerequisites.md) 


# UNDERSTANDING DRIVERS (theory)
- [Intro to Drivers](./understanding_drivers/understanding_drivers.md)
- [Role 1: Controlling the device below](./understanding_drivers/controlling_the_device_below.md)
- [Role 2: Providing an interface](./understanding_drivers/providing_an_interface.md)
- [Types of Drivers](./understanding_drivers/types_of_drivers.md)


# BARE METAL PROGRAMMING
- [No std preface](./bare_metal/the_no_std_preface.md)
  - [Machine Code](./bare_metal/definition.md)
  - [Dependencies](./bare_metal/dependencies.md)
  - [The Standard Library](./bare_metal/the_std_library.md)
  - [Bare Metal](./bare_metal/no_std/the_no_std_intro.md)
  - [Disabling the Standard Library](./bare_metal/no_std/removing_std_lib.md)
  - [The Core library](./bare_metal/no_std/core_library.md)
    - [Panic Symbols](./bare_metal/no_std/panic_symbols.md)
    - [Memory Symbols](./bare_metal/no_std/memory_symbols.md)
  - [Practicals - part 1](./bare_metal/no_std/pracs_1.md)
  - [Practicals - part 2](./bare_metal/no_std/pracs_2.md)
- [Cross-Compilation](./bare_metal/cross_compilation/cross_compilation.md)
  <!-- - [The Rust compiler](./bare_metal/cross_compilation/the_rust_compiler) -->
  - [LLVM](./bare_metal/cross_compilation/LLVM.md)
  - [Cranelift](./bare_metal/cross_compilation/Cranelift.md)
- [Linkers and Linking](./bare_metal/linking/linking.md)
- [Practicals - part 3](./bare_metal/no_std/pracs_3.md)
- [Binary Inspection](./bare_metal/binary_tools/bin_tools.md)



# THEORY ON THE UART
- [Intro](./uart_theory/intro.md)
- [general overview](./uart_theory/draft_1.md)
- [uart registers- part 1](./uart_theory/draft_2.md)
<!-- - [uart registers- part 2](./uart_theory/draft_3.md) -->
- [uart registers- part 2 ](./uart_theory/draft_4.md)

# ABSTRACTIONS OVER HARDWARE
- [Abstractions](./abstractions/abstractions.md)
- [Abstracting a Peripheral](./abstractions/abstracting_a_peripheral.md)
  - [System Timer - Part 1](./abstractions/system_timer_part_1.md)
  - [System Timer - Step 1 & 2](./abstractions/system_timer_part_2.md)
- [Registers and MMIO programming](./registers_and_mmio_programming.md)
- [re-definition using strict provenance](./abstractions/strict_provenance.md)
- [The datasheet](./knowing_your_hardware.md)
- [Abstraction]()
  - [MAC, PAC, HAL]()
  - [svd2rust](./svd2rust.md)
  - [SVD](./abstractions/svd2rust/svd_format.md)
  - [IP-XACT](./abstractions/ip_xact.md)
  - [UVM](./abstractions/uvm.md)
  - [SystemRDL](./abstractions/systemrdl.md)
  - [vcell](./abstractions/vcell.md)
  - [representations](./abstractions/representations.md)
- [random](./abstractions/random.md)

# THE UART IMPLEMENTATION (on Qemu and single-threaded)
- [Intro](./uart_implementations/on_qemu/intro.md)
  ## Set Ups
  - [Setting Things Up](./uart_implementations/on_qemu/setting_things_up.md)
  - [Setting up the compiler](./uart_implementations/on_qemu/setting_up_the_compiler.md)
  - [No-std recap](./uart_implementations/on_qemu/writing_a_bare_metal_rust_executable%20copy.md)
  - [Setting up the Riscv Virtual environment](./uart_implementations/on_qemu/setting_up_qemu.md)
  - [Setting up the linker](./uart_implementations/on_qemu/setting_up_LLD_linker.md)
  - [Automating build & run](./uart_implementations/on_qemu/setting_up_build_automation.md)
## Loaders and Bootloaders
- [Loaders and Bootloaders](./uart_implementations/on_qemu/loaders_and_bootloaders/intro.md)
  - [The Bootloader](./uart_implementations/on_qemu/loaders_and_bootloaders/bootloader.md)
  - [tutorial over a naive UART implementation on a Esp32 device]()

# Testing
- [no-std testing](./testing_theory/intro.md)


# Probing
- [Probing](./bare_metal/probing/probing_preface.md)
  - [Probing Theory](./bare_metal/probing/probing_theory_1.md)
  - [Probing Pracs](./bare_metal/probing/pracs.md)
   - [udev](./bare_metal/probing/udev.md)
   - [Flashing](./bare_metal/probing/flashing.md)
   - [Monitoring and Logging](./bare_metal/probing/logging_and_monitoring/monitoring_1.md)
     - [draft_1](./bare_metal/probing/logging_and_monitoring/monitoring_2.md)
  - [Debugging]()
  - [No-std testing]()

# THE UART IMPLEMENTATION (less naive, with concurrency in mind)
- [Concurrency]()
  - [critical section crate](./conurrency/critical_section.md)
  - [critical-sections in single-threaded cores]()
  - [critical-secions above multi-cores]()
  - [real-life-examples](./conurrency/examples.md)
- [tutorial over a naive UART implementation on a Qemu device]()
- [tutorial over a naive UART implementation on a Esp32 device]()


# OTHER STORIES
- [Performance testing]()
- [Driver Security]()
  - [Common Security Issues in Driver Development]()
  - [Rust's Safety Features for Driver Security]()
  - [Best Practices for Secure Driver Development]()
- [Case Studies and Examples]()
  - [Real-world Driver Development Examples](./case_studies/case_studies_and_examples.md)
  - [Analyzing an Existing Rust Driver]()
- [Iterative Implementation]()
- [core::io]()
  - [display and debuging structure]()
- [Custom Targets]()

# APPENDIX
- [Notable Crates](./notable_crates.md)
- [Notable Learning Resources]()
- [Notable core-crates](./misc/notable_core_crates.md)
- [why use Rust?](./why_embedded_rust.md)
- [The Rust Toolchain](./bare_metal/the_rust_toolchain.md)
- [Further Explanations](./misc/further_explanations.md)
  - [Firmware versus Drivers](./misc/drivers_vs_firmware.md)
  - [different_std_libs](./misc/different_std_libs.md)
  - [a-runtime](./misc/a_runtime.md)
  - [the-C-runtime](./misc/the_C_runtime.md)
  - [the-Rust-runtime](./misc/the_Rust_runtime.md)
  - [Riscv Runtime](./misc/riscv-rt.md)
  - [execution-environment](./misc/execution_environment.md)
  - [api-definition](./misc/API.md)
  - [abi-definition](./misc/abi.md)
  - [isa-definition](./misc/isa.md)
  - [factors affecting object file](./misc/target_factors.md)
  - [How to build runtimes](./misc/building_runtime_crates.md)
  - [rustup-target-add](./misc/rustup_target_add.md)
  - [Direct Memory Access](./misc/memory/DMA.md)
  - [communication protocols](./misc/comms/protocols.md)
- [tips on pointer abstraction](./abstractions/tips_for_safety.md)
- [svd2rust further explanations](./abstractions/svd2rust/further_explanations.md)


# DRAFTS
- [bench-marking](./drafts/bench_marking.md)
- [more on no-std](./drafts/more_on-no-std/intro.md)
  - [core::mem](./drafts/more_on-no-std/core_mem.md)
  - [core::iter](./drafts/more_on-no-std/core_iter.md)
- [macros](./drafts/macros.md)
- [untouched](./drafts/untouched.md)
- [Codegen](./drafts/codegen.md)

# UNBECOMING
- [HDLs](./unbecoming/hdls.md)
- [Interfaces](./unbecoming/interfaces.md)
- [community_links](./unbecoming/arbitrary_links.md)
