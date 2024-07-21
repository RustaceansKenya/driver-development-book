# The Rust Runtime

(undone chapter)

Hopefully, you've read this [intro to runtimes](./a_runtime.md).

>Unlike the JAVA runtime and JS runtime, the Rust runtime is not an application that runs independent of the user-program. The Rust runtime is a set of library files that get statically compiled together with your Rust code. Rust runtime is not an independent application, it is a dependency that gets linked and compiled together with the program that you are writing.  

The Rust runtime follows the [third meaning](./a_runtime.md#meaning-3-). ie The Rust runtime is the startup code that gets executed in preparation for the call to the `main` function.


### Note from the author...  
The rust runtime on github is focussed on unix and is `std`-dependent. see [here](https://github.com/rust-lang/rust/blob/master/library/std/src/rt.rs).  
I do not undertand exactly what the Rust-runtime on unix does.  
Here is the source code for the Runtime : [link to page](https://github.com/rust-lang/rust/blob/master/library/std/src/rt.rs)  
Here is a page that tries to explain what the runtime does : The Rust Reference book: [The Rust runtime](https://doc.rust-lang.org/reference/runtime.html)  

I am currently assuming that the functions of the Runtime are...
1. Setting up threading and creating a new special thread for 'main'
2. Inserting `clean_up` code. This is code that **MAY** get executed at the end of the `main` function. It clears up memory.  
3. Setting up backtracing functions
4. Setting up panicking  behavior, especially `panic-unwinding`.  

### Silver lining
Since the Rust runtime is `std`-dependent, we are better off spending our time understanding alternative runtimes that ...
1. are `no-std` compliant
2. implement protection features that may be missing from the C-runtime

Our main focus will bow be on [`riscv-rt`](https://github.com/rust-embedded/riscv/tree/master/riscv-rt), a riscv-runtime built by the [embedded-rust team](https://github.com/rust-embedded).  


[link pointing to the chapter covering `riscv-rt`](./riscv-rt.md)


(undone)