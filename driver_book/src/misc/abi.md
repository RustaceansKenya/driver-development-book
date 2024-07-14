# ABIs

Tbh, the current author is really having a hard time defining what an ABI really is. 
- What it entails, 
- what it doesnt entail, 
- why it has what it has, 
- which components are affected by the ABI specifications {compiler, linker, language-runtime, processor, loader}

The author is also having a hard time differentiating the different ABI standards and defining their trade-offs in different situations.  

So far, [this document](https://d3s.mff.cuni.cz/files/teaching/nswi200/202324/doc/riscv-abi.pdf) has been helpful.

(undone)



<br><br><br><br><br><br><br><br><br><br><br>




## What is an ABI?  

An ABI (Application Binary Interface) is a set of specifications defining.... 

### Post runtime specs 
1. **Data representation**: How data types are represented in memory, including issues like endianness (byte order), alignment, and padding.  
2. **Object file formats**: The structure and layout of object files, which contain compiled code and data before they are linked into an executable. 

### Runtime specs
3. **Dynamic linking**: How dynamically linked libraries are loaded and resolved at runtime.  
4. **Function calling conventions**: How parameters are passed to functions, how return values are returned, and how functions are invoked.



- https://stackoverflow.com/questions/2171177/what-is-an-application-binary-interface-abi



THe ABI defines the :  
 1. Calling conventions
 2. How parameter are passed
 3. Object file format
 4. Executable file format
 5. stack frame layout
 6. How types get encoded 
    - endianness
    - lengths
    - encode pattern (eg characters use UTF-8)



### The C ABI?  
C itself as a language standard doesn't define a specific ABI (Application Binary Interface). Instead, the ABI is typically determined by the platform and compiler being used. An ABI defines how functions are called, how parameters are passed, how data is laid out in memory, and other low-level details necessary for binary compatibility between separately compiled modules.

Different compilers and platforms may have their own ABIs. For example:

    x86-64 System V ABI: This is the ABI commonly used on many Unix-like operating systems for 64-bit x86 processors. It specifies how parameters are passed, how the stack is managed, how functions are called, etc.

    Windows x64 ABI: Microsoft Windows uses its own ABI for 64-bit x86 processors, which differs in certain aspects from the System V ABI.

    ARM EABI: The Embedded Application Binary Interface for ARM processors, which defines how code should be compiled and linked for ARM-based systems.

    RISC-V ABIs: As discussed earlier, there are several ABIs for RISC-V processors, such as ilp32, lp64, etc.


### Riscv ABIs





#### Messes
- see symbol table and learn to understand it
- see relocation table and learn to understand it
- disassemble a program
- assemble a program
- link a couple of object files into an executable, observe the relocation
- see the undefined reference in object symbol and relocation table. Before and after linking


### On using GNU-based toolchains 
If you intend to primarily write your code in Rust and want to leverage the Rust ecosystem, using the Generic ELF/Newlib toolchain might not be the most desirable option. Here's why:

    Compatibility: The Generic ELF/Newlib toolchain is primarily tailored for C development, particularly in embedded systems or bare-metal environments. While Rust can interoperate with C code, using Newlib might introduce additional complexity when working with Rust code.

    Standard Library: Rust provides its standard library (std), which is designed to work across different platforms and environments. By default, Rust code targets libstd, which provides a rich set of functionality beyond what Newlib offers. Using the Generic ELF/Newlib toolchain might limit your ability to leverage Rust's standard library and ecosystem.

    Community Support: Rust has a vibrant community and ecosystem, with many libraries and tools developed specifically for Rust. Using the Generic ELF/Newlib toolchain might limit your access to these resources, as they are often designed to work with Rust's standard library (std) rather than Newlib.

    Maintenance: While it's possible to use Rust with the Generic ELF/Newlib toolchain, maintaining Rust code alongside C code compiled with Newlib might introduce challenges, especially if you're not already familiar with both languages and their respective toolchains.

Instead, if you intend to write mostly in Rust, consider using toolchains and libraries that are specifically designed for Rust development in embedded systems or bare-metal environments. For example, you could use toolchains targeting libcore or libraries like cortex-m, embedded-hal, or vendor-specific hal crates, which provide idiomatic Rust interfaces for interacting with hardware and low-level system functionality. These options are more aligned with Rust's design principles and ecosystem and might provide a smoother development experience for Rust-centric projects.
