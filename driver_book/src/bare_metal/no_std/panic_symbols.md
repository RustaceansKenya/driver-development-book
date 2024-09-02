# Panic Symbols  

> Disclaimer: The author is not completely versed with the internals of panic. Improvement contributions are highly welcome, you can edit this page however you wish.   

The core library requires the following panic symbols to be defined :  
1. `rust_begin_panic`
2. `eh_personality`

Before we discuss why those symbols are needed, we need to understand how panicking happens in Rust.  
To understand panicking, please read [this chapter](https://rustc-dev-guide.rust-lang.org/panic-implementation.html) from the [`rust-dev-guide` book][`rust-dev-guide` book].  
It would also be nice to have a copy of the Rust source-code so that you can peek into the internals of both `core::panic` and `std::panic`.   

But before, you read those resources... let me try to explain panicking.  

## Understanding panic from the ground up.  
Panics can occur explicitly or implicitly. If this statement does not make sense, read this [Rust-book-chapter][Rust-book-chapter].

We will deal with explicit panics for the sake of uniformity and simplicity. Explicit panics are invoked by the `panic!()` macro.  

When the Rust language runtime encounters a `panic!` macro during program execution, it immediately documents it. It documents this info internally by instantiating a struct called [`Location`][location-struct]. `location` stores the path-name of the file containing `panic!()`, the faulty line and the exact column of the 'source token parser'.  
Here is the struct definition of `Location`:  
```rust
pub struct Location<'a> {
    file: &'a str,
    line: u32,
    col: u32,
}
```


The Rust-runtime then looks for the panic `message` that will add more info about the panic. Most of the time, that message is declared by the programmer like shown below. Sometimes no message is provided.  
```rust
let x = 10;
panic!("panic message has been provided");
panic!(); /*panic message has NOT been provided*/
```

The runtime then takes the `message` and the `location` and consolidates them by putting them as fields in a new instance of [`struct PanicInfo`][panic-info]. Here is the internal description of the `PanicInfo` : 
```rust
pub struct PanicInfo<'a> {
    payload: &'a (dyn Any + Send), 
    message: Option<&'a fmt::Arguments<'a>>,  // here is the panic message, as you can see...it is optional
    location: &'a Location<'a>, // here is the location field, it is not optional
    can_unwind: bool,
    force_no_backtrace: bool,
}
```  

Now that the rust-runtime has an instance of `PanicInfo`, it moves on to either one of these two steps depending on the circumstaces: 
1. It passes the `PanicInfo` to the function tagged by a `#[panic_handler]` attribute
2. It passes the `PanicInfo` to the function that has been set by [`set_hook`][set-hook] or [`take_hook`][take-hook]

If you are in a no-std environment, option 1 is taken.  
If you are in a std-environment, option 2 is taken.  

### The `#[panic_handler]` attribute and panic hook
The panic_handler is an attribute that tags the function that gets called after `PanicInfo` has been instantiated AND before the start of either stack unwinding or program abortion.  

The above ☝🏾 statement is heavy, let me explain.  

When a panic happens, the aim of the sane programmer is to : 
1. capture the panic message and panic location (file, line, column).
2. maybe print the message to the `stderr` or to some external display
3. Recover the program to a stable state using mechanisms like `unwrap_or_else`
4. Terminate the program safely if the panic is unrecoverable.  

**Step 1**  
The Runtime automatically does for you step one by creating `PanicInfo`.  

**Step 2:**  
It is then up to the programmer to define a `#[panic_handler]` function that consumes the `PanicInfo` and implement step 2.  
You can do something like this:  

```rust
#[panic_handler]
fn my_custom_panic_handler (_info: &PanicInfo) -> !{
    println!("panic message: {}", _info.message()); // print message to stdout
    println!("panic location: file: {}, line: {}". info.location.file(), info.location.line());
    loop{}
}
```  

If you are in an std environment, implementing step 2 is optional. This is because the std library already defines a default function called `panic_hook` that prints the panic message and location to the stdout.  
This means that if you define a `#[panic_handler]` function in an std environment, you will get a duplication compilation error. A program can only have one `#[panic_handler]`.  
The only way to define a new custom panic_hook in an std environment is to use the [set_hook][set-hook] function.  


**Step 3 & 4:**  
Both the `panic_hook` and the `#[panic_handler]` function transfer control to a code block called `panic-runtime`. There are two kinds of panic-runtimes provided by rust.
1. Panic-unwind runtime
2. panic-abort runtime

The programmer has the option of choosing one of the two options before compilation. In a normal std environment, `panic-unwind` runtime is usually the default.  

So what are these?  

**`panic-abort`** is a code block that causes the panicked program to immediately terminate. It leaves the stack occupied in hope that the kernel will take over and clear the stack. `panic-abort` does not care about safe program termination.  

**`panic-unwind`** runtime on the other hand cares about recovery, it will free the stack frame by frame while looking for a function that can act as a recoveror. On worst case, it will not find a reovery function and it will just safely terminate the program by clearing the stack and releasing used memory.  

So if you want to recover from panics, your bet would be to choose the `panic-unwind` runtime as your runtime of choice.  
So ... ***How is recovery implemented?***


### Panic Recovery  
As earlier mentioned, when a panic-worthy line of code gets executed, the language runtime itself creates an instance of `Location` and `Message` and eventually creates an instance of `PanicInfo` or `PanicHookInfo`. That is why you as the programmer have no way to construct your own instance of `PanicInfo`. It is something created at runtime by the language runtime.  

The language runtime then passes a reference of the `PanicInfo` to the #[panic_handler] or panic_hook.  
The panic hook and `#[panic_handler]` do their thing and eventually call either of the 2 panic runtimes.  

Hope we are on the same page till there.  
Now that we are on the same page, we need to introduce some new terms...  

#### catch_unwind and unwinding-handlers

The aim of the `panic-unwind` runtime is to achieve the following: 
1. deallocate the stack upwards, frame by frame.  
2. For every frame deallocated, it records that info as part of the Backtrace.
3. It continuously hopes that it will eventually meet an `unwinding-handler` function frame that will make it stop the unwinding process

If the `panic-unwind` finally meets a handler, it stops unwinding and transfers control to the Handler. It also hands over the PanicInfo to that handler function.  

So what are handler functions?  
Handler functions are functions that have the ability to stop the unwinding, consume the `PanicInfo` created by a panic and do some recovery magic.   

These handlers come in different forms. One of these forms is the [catch_unwind][catch-unwind] function.  
[catch_unwind][catch-unwind] is a function provided by `std` that acts as a "bomb container" for a funtion that may panic.  
This function takes in a closure that may panic and then runs that closure as an inner child. If the closure panics, catch_unwind() returns `Err(panic_message)`. If the closure fails to panic, `catch_unwind` returns an `Ok(value)`.  

Below is a demo of a catch_unwind function in use:    

```rust
use std::panic;

let result = panic::catch_unwind(|| {
    println!("hello!");
});
assert!(result.is_ok());

let result = panic::catch_unwind(|| {
    panic!("oh no!");
});
assert!(result.is_err())
```  

`catch-unwind` is not the only `unwinding-handler`, there are other implicit and explicit handlers. For example, Rust implicitly sets handlers that encompass each thread by default such that if a thread panics, it will unwind its stack till it meets either an explicit internal handler OR it eventually meets the implicit thread_handler inserted by the compiler during thread instantiation. The recoery mechanism implemented by rust in such a case is to return `Result(_)` to the parent thread.  

### What does it mean to catch a panic?
Catching a panic means preventing a program from completely terminating after a panic by either implicitly or explicitly adding `handlers` within your code.  

You can add implicit handlers by enclosing dangerous functions in isolated threads and count on the language runtime to insert `unwinding-handlers` for you.  
You can add an explicit handler by pasing a dangerous functions as an argument to a `catch_unwind` function


### Unwind Safety
UnwindSafe is a marker trait that indicates whether a type is safe to use after a panic has occurred and the stack has unwound. It ensures that types do not leave the program in an inconsistent or undefined state after a panic, thus helping maintain safety in Rust's panic recovery mechanisms.  

(undone: limited knowledge by initial author, contribution needed)
>My knowledge on unwind-safety ends there.      
Any contributor can focus on showing the `UnwindSafe`, `RefUnwindSafe` and `AssertUnwindSafe` markers in action. You can even show where they fail (thank you in advance) 

That's all concerning panic recovery, go figure out the rest.  


## Panic_impl and Symbols

During the compilation process, both the `#[panic_handler]` and `panic_hook` usually get converted into a language item called `panic_impl`.  
In Rust, "language items" are special functions, types, and traits that the Rust compiler needs to know about in order to implement certain fundamental language features. These items are usually defined in the standard library (std) or the core library (core) and are marked with the #[lang = "..."] attribute.  

Think of language items as 'tokens that affect the functionality of the compiler'.  

Below is the signature of `panic_impl`, I hope you can see the direct similarity btwn `#[panic_handler]` and `panic_impl`  
```rust
extern "Rust" {
    #[lang = "panic_impl"]
    fn panic_impl(pi: &PanicInfo<'_>) -> !;
}
```  

The reason this conversion takes place is because the language designers wanted to introduce indirection for the sake of making `std::panic` to have the ability to override `core::panic` during compilation.  

As the compilation levels go further, `panic_impl` gets compiled into the symbol `rust_begin_panic`. In the final binary file, the `panic_impl` symbol is absent.  

I guess now you understand what the core library demands from you when it says that you need to provide a definition of the `rust_begin_panic` symbol. 



### eh_personality

As for the `eh_personality` symbol, is not really a symbol. It is a languge item. 

The eh_personality language item marks a function that is used for implementing stack unwinding. By default, Rust uses unwinding to run the destructors of all live stack variables in case of a panic. This ensures that all used memory is freed and allows the parent thread to catch the panic and continue execution. Unwinding, however, is a complicated process and requires some OS-specific libraries (e.g. libunwind on Linux or structured exception handling on Windows)  

If you choose to use the `panic-unwind` runtime, then you must define the unwinding function and tag it as the `eh_personality` language item



<!-- (undone(help neded): this chapter needs more simple and correct explanations. I, the initial author(@kiarie404), is completely confused about how panicking happens in Rust) -->



[location-struct]: https://doc.rust-lang.org/std/panic/struct.Location.html 
[Rust-book-chapter]: (https://doc.rust-lang.org/book/ch09-00-error-handling.html)  
[`rust-dev-guide` book]: (https://rustc-dev-guide.rust-lang.org/)  
[panic-info]: https://doc.rust-lang.org/core/panic/struct.PanicInfo.html    
[set-hook]: https://doc.rust-lang.org/std/panic/fn.set_hook.html  
[take-hook]: https://doc.rust-lang.org/std/panic/fn.take_hook.html  
[catch-unwind]: https://doc.rust-lang.org/std/panic/fn.catch_unwind.html   