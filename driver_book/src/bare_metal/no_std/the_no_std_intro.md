# Bare-metal

So now you understand what the standard library is.  
You understand why it exists.  
You somehow understand its modules and functions.  


You understand that the standard library calls system functions in the background.  
You understand that the interface of a standard library is *'constant'*. ( i.e standardized, versionized and consistent across different platforms)  

You understand that the implementation of a standard library *is also NOT constant*; It is OS-dependent. For example the interfaces of the libc library is *constant* across all OSes but libc's implementations is different across all OSes; In fact the libc implementations have different names ... we have `glibc` for GNU-Linux's libc and `CRT` for windows' libc. GNU-inux even has extra implementations such as `musl-libc`. Windows has another alternative implementation called `MinGW-w64`  



## No-std  

Most rust programs depend on the standard library by default, including that simple 'hello world' you once wrote. The standard library on the other hand is dependent on the underlying operaring system or execution environment.   
For example, the body of `std::println!` contains lines that call OS-defined functions that deal with Input and output eg `write()` and `read()`.  


Drivers provide an interface for the OS to use, meaning that the OS depends on drivers... as a result, you have to write the driver code without the help of the OS-dependent Standard Library. This paragraph sounds like a riddle ha ha... but you get the point... to write a driver, you have to forget about help from the typical std library. That std library depends on your driver code... **the std library depends on you**.     

When software does not depend on the standard library, it is said to be a **bare-metal program.** It can just be loaded to the memory of a chip and the physical processor will execute it as it is.  




Bare metal programming is the art of writing code that assumes zero or almost-no hosted-environment. A hosted environment typically provides a language runtime + a system interface like POSIX.  

We will procedurally create a bare metal program in the next few sub-chapters.  


## Execution Environments

An Execution environment is the context where a program runs. It encompasses all the resources needed to make a program run.  
For example, if you build a video-game-plugin, then that plugin's execution environment is that video-game.  

In general software development, the word execution environment usually refers to the description of the processor-architecture, the operating system, avalable system libraries, environment variables, and other dependencies.  

Here is more jargon that you can keep at the back of your head:  

The processor itself is an execution environment.  
If you write a bare-metal program that is not dependent on any OS or runtime, you could say that the processor is the only execution environment that you are targeting.  

The kernel is also an execution environment. So if you write a program that depends on the availability of a kernel, you could say that your program has two exeution environments; The Kernel and the Processor.  

The Browser is also an execution environment. If you write a JS program, then your program has 3 execution environments: The Browser, the kernel and the Processor.  


>*Chips and boards are mostly made from silicon and other alloys that are not particularly metallic... I guess we should say `bare-silicon` programming instead of `bare-metal`?*


