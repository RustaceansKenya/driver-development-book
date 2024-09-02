# rustup-target-add

What doe the command below actually do?  
```bash
rustup target add riscvi32-unknown-none-elf
```


When you run rustup target add riscvi32-unknown-none-elf, Rustup performs several actions to add the specified target to your Rust toolchain. Let's break down what happens in technical terms:

1. Downloads the Target Specification File : Rustup downloads the target specification file for the specified target architecture (riscvi32-unknown-none-elf). These file defines various attributes of the target, such as its architecture, ABI (Application Binary Interface), and features. You can view such a file by running the command below: 
```bash
# Replace `riscv32i-unknown-none-elf` with a target of your liking
rustc -Z unstable-options --target riscv32i-unknown-none-elf --print target-spec-json
```

2. Installs Associated Toolchain Components: Rustup installs the necessary components and tools required to compile Rust code for the added target. Some of the components include 
        
    - Linker Script: Rustup may download a default linker script suitable for the target architecture. The linker script defines how the compiled code should be linked together and laid out in memory.

    - Core Library (libcore): For bare metal targets like none, Rustup installs a precompiled core library (libcore) tailored for the target architecture. libcore contains essential language primitives and types required for Rust programs, such as basic types (u8, i32, etc.) and core language features. This library is stripped down compared to the standard library (std) and does not include platform-specific functionality or I/O operations.

    - Any new and necessary LLVM component. eg a new linker flavour


3. Updates Rust Toolchain Configuration: Rustup updates the configuration of your Rust toolchain to include the newly added target. This ensures that when you compile Rust code using this toolchain, the compiler knows about the added target and can generate code compatible with it.