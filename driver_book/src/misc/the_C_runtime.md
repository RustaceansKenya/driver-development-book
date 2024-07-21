# The C Runtime

Hopefully, you've read this [intro to runtimes](./a_runtime.md).

The C runtime follows the [third meaning](./a_runtime.md#meaning-3-). ie The C runtime is the startup code that gets executed in preparation for the call to the `main` function.   

>Unlike the JAVA runtime and JS runtime, the C runtime is not an application that runs independent of the user-program. The C runtime is a set of library files that get statically compiled together with your C code. C runtime is not an independent application, it is a dependency that gets linked and compiled together with the program that you are writing.  

This C runtime is usually nick-named `CRT-0` and is typically written in assembly code.  
 

### Functions of the C runtime code  

There are many variations of the C-runtime.  
It is a free-world. It is up to you to decide what ***your*** C-runtime does.  
However, here are some of the typical functions found in any C-runtime library.  

1. Loading elf programs from ROM/secondary_memory to RAM. 
2. Allocating space for a software stack and initialize the stack pointer
3. Allocating space for a heap (if used)
4. Initializing static variables before the program entry point. This is achieved by copying values from Flash into variables declared with initial values
5. Zero-ing out all uninitialized global varibles.
6. Clearing uninitialized RAM
7. Populating the vector table so the device can properly dispatch exceptions and interrupts.  
8. Calling the `main()` function saely. 

Extra functions include :  
1. Setting up overlay control code.  
2. Setting up Stack protection code.
3. Setting up stack unwinding code.

Quite a mouthful ha? So many functions.  
Here are resources that will help you understand the C-runtime : 
- [The C Runtime Environment](https://developerhelp.microchip.com/xwiki/bin/view/software-tools/c-programming/c-runtime-enviorment/) by microchip.com. This summarizes things in a clean way. Best resource here.  
- This [c_startup](http://bravegnu.org/gnu-eprog/c-startup.html) blog by Vijay Kumar B on [bravegnu.com](http://bravegnu.org) takes you through writing a simple Runtime targeting the ARM board. If you have no interest in ARM, you can just skim through the tutorial and get the gist.  
- Good old [wikipedia](https://en.wikipedia.org/wiki/Crt0).

### Examples of C runtimes 
You can look at the code found in these repos in order to get a gist of how the internals of a C-runtime look like.  
- An implementation in Rust : [r0 by rust-embedded group](https://github.com/rust-embedded/r0/blob/master/src/lib.rs)
- An implementation in Assembly + Rust targeting Riscv boards : [riscv-rt by rust-embedded group](https://github.com/rust-embedded/riscv-rt/blob/master/src/lib.rs) 

> Later, we will write our own Runtime targeting the Riscv Board that we will be working with  

> You can always skip the whole process of writing your own runtime and instead use [`riscv-rt`](https://docs.rs/riscv-rt/latest/riscv_rt/), but where is the fun in that?

