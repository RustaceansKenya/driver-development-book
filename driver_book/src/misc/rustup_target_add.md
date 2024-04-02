# rustup-target-add

What doe the command below actually do?  
```bash
rustup target add riscvi32-unknown-none-elf
```


When you run rustup target add riscvi32-unknown-none-elf, Rustup performs several actions to add the specified target to your Rust toolchain. Let's break down what happens in technical terms:

1. Downloads Target Specification Files: Rustup downloads the target specification files for the specified target architecture (riscvi32-unknown-none-elf). These files define various attributes of the target, such as its architecture, ABI (Application Binary Interface), and features.

2. Installs Associated Toolchain Components: Rustup installs the necessary components and tools required to compile Rust code for the added target. This includes:  
        
    - Linker Script: Rustup may download a default linker script suitable for the target architecture. The linker script defines how the compiled code should be linked together and laid out in memory.

    - Core Library (libcore): For bare metal targets like none, Rustup installs a precompiled core library (libcore) tailored for the target architecture. libcore contains essential language primitives and types required for Rust programs, such as basic types (u8, i32, etc.) and core language features. This library is stripped down compared to the standard library (std) and does not include platform-specific functionality or I/O operations.

    - LLVM Flavour: Rustup may also download or configure the LLVM (Low-Level Virtual Machine) backend specific to the target architecture. LLVM is the compiler infrastructure used by Rust for code generation and optimization. Different target architectures may require different LLVM configurations or optimizations.


3. Updates Rust Toolchain Configuration: Rustup updates the configuration of your Rust toolchain to include the newly added target. This ensures that when you compile Rust code using this toolchain, the compiler knows about the added target and can generate code compatible with it.