# The C Runtime

What is the C runtime? What is a runtime?  


## 100 Meanings
Well, the word runtime is a dense word, with multiple meanings that vary with context. Here are some of the definitions :  

**Meaning 1** :  
Runtime refers to the duration of time consumed when a program was executing.  
For example, if you played a videogame for 12 hours, you could say that that videogame had a 12-hour runtime.   

**Meaning 2** :  
Runtime refers to a piece of software that continuously tends to the needs of another running program.  

For example :  
1. If you are playing an online video game that links up with a remote game server, you could say that that game-server is your runtime device.  

2. In the case of programs written with languages with garbage-collection, you could say that those programs depend on a runtime-software to continuously collect the dead variables as a background service. In this case, the garbage-collector is part of the runtime.  

3. In the case of Intepreted languages, you could say that the intepreter IS the runtime service. This is because the intepreter needs to continuously compile and execute the program as it runs.  


**Meaning 3** :  
Programs usually dont start getting executed just like that. There has to be code that makes the CPU to point to the right lines of code, make sure that there is enough stack space, make sure that some CPU registers are okay...  

Point is, there is some code that gets executed before the actual program ie. Init code or control code.  

In this context, Runtime means init-code. Runtime means control code.  
In this context, runtime code only gets executed once. It does not get executed continuously as the other program runs. It just gets executed at the very beginning.  


## C runtime
The C runtime follows the third meaning. ie The C runtime is the startup code that gets executed in preperation for calling the main program.  

This C runtime is usually nick-nammed CRT-0 and is typically written in assembly code.  
It is not an independent application, it is just a library file that can get linked and compiled together with the program that you are writing.  

### Functions of the C runtime code  

It is a free-world. It is up to you to decide what your C-runtime does.  
However, here are some of the typical functions of any C-runtime variation.  

1. Loading elf programs from ROM/secondary_memory to RAM. ie (elf_loading)
2. Allocate space for a software stack and initialize the stack pointer
3. Allocate space for a heap (if used)
4. nitializing static variables before the program entry point. This is achieved by copying values from Flash into variables declared with initial values
5. Zero out all uninitialized global varibles.
6. Clear uninitialized RAM
7. It populates the vector table so the device can properly dispatch exceptions and interrupts
8. Call the `main()` function 

Extra functions include :  
1. Set up overlay code
2. undone : add other features found in advanced CRTs

Quite a mouthful ha? So many functions.  
To understand the above functions, read these 2 resources : 
- [c_startup](http://bravegnu.org/gnu-eprog/c-startup.html) by Vijay Kumar B on [bravegnu.com](http://bravegnu.org)
- [The C Runtime Environment](https://developerhelp.microchip.com/xwiki/bin/view/software-tools/c-programming/c-runtime-enviorment/) by microchip.com


### Examples of C runtimes 
You can look at the code found in these repos in order to get a gist of what the Crt does.  
- An implementation in Rust : [r0 by rust-embedded group](https://github.com/rust-embedded/r0/blob/master/src/lib.rs)
- An implementation in Assembly + Rust targeting Riscv boards : [riscv-rt by rust-embedded group](https://github.com/rust-embedded/riscv-rt/blob/master/src/lib.rs) 

