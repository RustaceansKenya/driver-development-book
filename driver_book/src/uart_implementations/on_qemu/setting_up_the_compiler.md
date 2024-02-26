# Setting Up the Compiler

The compiler is the tool that will convert our source code to machine code that suits a specific machine target.  
In our case, that specific machine target is "The RISCV CPU, bare metal".   

The rust compiler gets installed as a toolchain, so it comes with a linker attached. For this reason, our compile button will do the following : 
1. Compile rust files
2. Compile the embedded assembly files.
3. Link all the object files and produce an executable ELF file. (linker part)

Ofcourse you an use a 3rd party linker that you prefer, you are not forced to use the attached linker. But using another linker looks like a lot of unnecessary hard work.  

In the compiler world, people identify compilation targets using a standard naming convention called "Target Triple".  
Initially the Target triple specified three characteristics of the machine target : 
    - CPU Architecture                         : eg x86, RISCV, ARM
    - Vendor who manufactures the target       : eg Apple, IBM
    - The operating system running on the CPU  : eg Linux, Windows, FreeBSD, None

For example you would define a target as : ARM-Apple-IoS

But the world got more complex, now we have people naming things like... i don't know... it is not 3 characteristics anymore.   Sometimes you have 2 sometimes 5, 4 or 3.

So here is a 4 identifier description :
    - CPU architecture
    - Vendor
    - Operating System
    - ABI

Really, there is confusion, but hopefully you can tell what stands for what when you see a triple target with a weird number of identifiers.  

### Commands 

To install the Stable Rust compiler, enter the following comand :
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  
```
Alternatively, you can visit this page : [Rust Compiler installation Main Page](https://www.rust-lang.org/tools/install)

But in our project, we will use Nightly features. So you will need to install Rust Nightly :
```bash
rustup toolchain install nightly  //install nightly Compiler
rustup default nightly            // set nightly Compiler as the default toolchain
```

The Machine Target we are compiling for is "riscv64gc-unknown-none-elf" which means we are compiling for 
- "**riscv646gc** -  64-bit-Riscv CPU that  : supports all general instructions 'g' and supports [compressed instructions](./compressed_instructions.md) 'c'
- **unknown** - means that the manufaturer of the CPU is unknown or that info is irrelevant
- **none** - means that the CPU has no operating system running on top of it
- **elf** - This component identifies the format of the output binary file that will be generated by the compiler. In this case, it specifies that the binary file will be in the ELF (Executable and Linkable Format) format, which is a common format used for executables and shared libraries on Unix-based systems.

To check out all the targets that the compiler can support by default, type the following comand :
```bash
rustup target list               // list all supported targets
rustup target list --installed   // list all installed supported targets
```

To install our riscv64gc-unknown-none-elf target, enter th following command ;
```bash
rustup target add riscv64gc-unknown-none-elf  // install a supported target
```

If you come up with your own custom target, you can tweak the toolchain to support your target. Like for our case, we are going to come write an operating system. The toolchain does not know our OS, So people are not able to compile specifically for our OS. That topic is discussed [here](./Custom_targets.md)

We are done setting up the compiler!!!