# Panic Symbols  

The panic symbols that the core library needs you to define are :  
1. `rust_begin_panic`
2. `eh_personality`

Before we discuss why those symbols are needed, we need to understand how panicking happens in Rust.  
To understand panicking, please read [this chapter](https://rustc-dev-guide.rust-lang.org/panic-implementation.html) from the [`rust-dev-guide` book](https://rustc-dev-guide.rust-lang.org/)  

From the chapter, you get the gist that when a panic happens the following events might occur depending on the situation...

## Situation 1 (std environment)
When you are in an std environment and the program reaches a state where it needs to panic....  

The program jumps to execute a `panic_hook`. A panic_hook is a function that gets executed before the actual panicking occurs. This function typically prints the expected panic_message to the `stderr` and checks for double panics before finally transferring control to the `panic_runtime`.  

A `panic_runtime` is the actual code block that handles the panic. This code block can either 'terminate the current thread' OR it can 'release the thread's stack memory before terminating the thread'.  
The `panic_runtime` that terminates without clearing the stack is called `panic-abort` and the panic_runtime that clears the stack before terminating the thread is called `panic-unwind`.  

And that's it!! That's how panicking happens (... my knowledge ends there, everything else is a black-box to me)


## Situation 2 (no-std environment)
Panic handling in a no-std setup is a bit simpler.  

Before compilation, the programmer is given a choice to choose whether the no-std program will use the `panic-abort` runtime or the `panic-unwind` runtime. You choose one by denoting it in the `Config.toml` file.  

When a panic happens, the program jumps to the function that has been tagged with the `#[panic_handler]` attribute.  
It is up to the programmer to tag a function of their choosing with the `#[panic_handler]` attribute before compilation. Like this :  
```rust
// your no-std code

#[panic_handler]  // here is the tag
fn whatever_name_you_choose ( _: &PanicInfo ) -> !{
    // write whatever you want here.  
    // Do you want to print to the stderr? Write code for that.  
    // Do you want to restart the program? Write code for that. 
    // Point is, this is your playground. You determine the actions of the panic_handler
}

// your other pieces of sh*tty code
```

During compilation, the above code gets blended with the chosen panic-runtime.   

So that's it. When a panic happens, the program executes the code inside the `whatever_name_you_choose` function above and then it executes the chosen `panic-runtime` (either panic-abort or panic-unwind)


# So what about the symbols? 

You see the function that you tagged with the `#[panic_handler]` attribute? Yea, see it? During compilation, that function gets converted to be a function called `rust_begin_panic`. You do not have to explicitly define `rust_begin_panic` because the compiler will implicitly convert any function with the `#[panic_handler]` attribute to become `rust_begin_panic`.  
This encourages portability since you can do conditional attribute assignation.   

The function marked with the `#[panic_handler]` attribute defines the behavior of panics, both library level panics (core::panic!) and language level panics (out of bounds indexing).  

So that takes care of the `rust_begin_panic` symbol.  

As for the `eh_personality` symbol, is not really a symbol. It is a languge item. In Rust, "language items" are special functions, types, and traits that the Rust compiler needs to know about in order to implement certain fundamental language features. These items are usually defined in the standard library (std) or the core library (core) and are marked with the #[lang = "..."] attribute.  

Think of language items as 'tokens that affect the functionality of the compiler'.  

The eh_personality language item marks a function that is used for implementing stack unwinding. By default, Rust uses unwinding to run the destructors of all live stack variables in case of a panic. This ensures that all used memory is freed and allows the parent thread to catch the panic and continue execution. Unwinding, however, is a complicated process and requires some OS-specific libraries (e.g. libunwind on Linux or structured exception handling on Windows)   




