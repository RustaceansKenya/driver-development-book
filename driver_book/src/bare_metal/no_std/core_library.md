# The Core library

The Rust Core Library is the dependency-free foundation of The Rust Standard Library.  
That is such a fancy statement... let us break it down.  

Core is a library like any other. Meaning that your code can **depend on it**. You can find its documnetation at [this page](https://doc.rust-lang.org/core/index.html)  


## What does the Core library contain? What does it do?

1. Core contains the definitions and implementations of primitive types like `i32`, `char`, `bool` etc. So you need the core library if you are going to use primitives in your code.  
2. Core contains the declaration and definitions of basic macros like `assert` and `assert_eq`.  
3. Core contains modules that provide basic functionalities. For example, the `array` module provides you with methods that will help you in manipulating an array primitive.  

## What does the core library lack that std has?  

Core lacks libraries that depend on OS-system files and OS-level services.  

For example, core lacks the following modules that are found in the std library ... mostly because the modules deal with OS-level functionalities.
1. `std::thread` module. Threading is a service that is typically provided by a kernel.
2. `std::env` module. This module provides you with ways to Inspect and manipulate a process’ environment. Processes are usually an abstration provided by an OS.
3. `std::backtrace`
4. `std::boxed`
5. `std::os`
6. `std::string`

Look for the rest of the missing modules and try to answer the following questions : 
1. "why isn't this module not found in core?", 
2. "if it were to be implemented in core, how would the module interface look like?".  

The above 2 questions are hard.  
In the past, the experimental `core::io` did not exist, but now it does because the above two questions were answered(partially). It is still an ongoing answer.  


Something to note, just because a module's name is found in both std and core, it is not a guarantee that both the modules contain identical contents. Modules with the same names have different contents.  
For example, `core::panic` exposes ZERO functions while `std::panic` exposes around 9 functions.  


## Is the Core really dependency free?  
A dependency-free library is a library that does not depend on any other external library or file. It is a library that is complete just by itself.  
The core library is **NOT fully dependency free**. It just depends on VERY FEW external definitions.  

The compiled core code typically contains undefined linker symbols. It is up to the programmer to provide extra libraries that contain the definitions of those undefined symbols.  
So there you go... Core is not 100% dependency-free.  

The undefined symbols include :  
1. Six Memory routine symbols : `memcopy`, `memmove`, `memset`, `memcmp`, `bcmp`, `strlen`.
2. Two Panic symbols: `rust_begin_panic`, `eh_personality`

## What are these symbols?  
We will discuss the above symbols in the next 2 sub-chapters



