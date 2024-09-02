# Practicals

Now that you know a little bit about the core library, we can start writing programs that depend on `core` instead of `std`.  
This chapter will take you through the process of writing a no-std program.  
We will try our very best to do things in a procedural manner...step by step... handling each error slowly.  

If you do not wish to go through these practicals(1,2 & 3) in a stepwise fashion, you can find the complete no-std template [here](undone)

## Step 1: Disabling the Std library

Go to your terminal and create a new empty project :  
```bash
cargo new no_std_template --bin
```

Navigate to the `src/main.rs` file and open it.  
By default, rust programs depend on the standard library. To disable this dependence, you add the ['#[no_std]`][no-std-attribute] attribute to your code. The no-std attribute removes the standard lobrary from the crate's scope.  
```rust
#![no_std] // added the no-std attribute at macro level (ie an attribute that affects the whole crate)

fn main(){
    println!("Hello world!!");
}
```

If you build this code, you get 3 compilation errors. 
1. error 1: "cannot find macro `println` in this scope"
2. error 2: "`#[panic_handler]` function required, but not found"
3. error 3: "unwinding panics are not supported without std"

You can run this code by pressing the `play` button found at the top-right corner of the code block above. Or you can just write it yourself and run it on your local machine.  

## Step 2: Fixing the first Error

The error that we are attempting to fix is...  
```bash
# ... snipped out some lines here ... 

error: cannot find macro `println` in this scope
 --> src/main.rs:3:5
  |
3 |     println!("Hello, world!");
  |     ^^^^^^^

# ... snipped out some lines here ... 
```
The [`println! macro`][println-macro-doc] is part of the `std` library. So when we removed the `std` library from our crate's scope using the `#![no_std]` attribute, we effectively made the `std::println` macro to also go out of scope.   

To fix the first error, we either...
1. Stop using `std::println` in our code  
2. Define our own custom `println` 
3. Bring `std` library back into scope.(Doing this will go against the main aim of this chapter; to write a no-std program)

We cannot choose option 3 because the aim of this chapter is to get rid of any dependence on the `std` library.  

We could choose option 2 but implementing our own `println` will be cost us unnecessary hardwork. Right now we just want to get our no-std code compiling...  For the sake of simplicity, we will not choose option 2. We will however write our own `println` in a later chapter.  

So we choose the first option, we choose to comment out the line that uses the proverbial `println`.  
This has been demonstrated below.  

```rust
#![no_std]

fn main(){
    // println!("Hello world!!"); // we comment out this line. println is indeed undefined
}
```
Only two compilation errors remain...  

## Fixing the second and third compilation errors

This is going to be a short fix but with a lot of theory behind it.  
To solve it, we have to understand the [core library requirements][core-library-requirements] first.  

The core library functions and definitions can get compiled for any target, provided that the target provides definitions of certain linker symbols. The symbols needed are :
1. memcpy, memmove, memset, memcmp, bcmp, strlen. 
2. rust_begin_panic
3. rust_eh_personality (this is not a symbol, it is actually a [language item][rust-eh-language-item])

In other words, you can write whatever you want for any supported ISA, as long as you link files that contain the definitions of the above symbols.  

### 1. memcpy, memmove, memset, memcmp, bcmp and strlen symbols


These are all symbols that point to memory routines.  
You need to provide to the linker the ISA code that implements the above routines.  

When you compile Rust code for a specific target architecture (ISA - Instruction Set Architecture), the Rust compiler needs to know how to generate machine code compatible with that architecture. For many common architectures, such as x86, ARM, or MIPS, the Rust toolchain already includes pre-defined implementations of these memory routines. Therefore, if your target architecture is one of these supported ones, you don't need to worry about providing these definitions yourself.  

However, if you're targeting a custom architecture or an architecture that isn't directly supported by the Rust toolchain, you'll need to provide implementations for these memory routines. This ensures that the generated machine code will correctly interact with memory according to the specifics of your architecture.  

### 2. the rust_begin_panic symbol


This symbol is used by Rust's panic mechanism, which is invoked when unrecoverable errors occur during program execution. Implementing this symbol allows the generated code to handle panics correctly.  
You could say that THIS symbol references the function that the Rust runtime calls whenever a panic happens.  

This means that you have to... 
1. Define a function that acts as the overall panic handler. 
2. Put that function in a file
3. Link that file with your driver code when compiling.  

For the sake of ergonomics, the cool rust developers provided a 'panic-handler' attribute that you can attach to a divergent function. You do not have to do all the linking vodoo. This has been demonstrated later on... do not worry if this statement did not make sense.  

You can also revisit the [subchapter on `panic symbols`](./panic_symbols.md) to get a clear relationship between the `rust_begin_panic` symbol and the `#[panic_handler]` attribute.  

### 3. The rust_eh_personality 

When a panic happens, the rust runtime starts unwinding the stack so that it can free the memory of the affected stack variables. This unwinding also ensures that the parent thread catches the panic and maybe deal with it.  

Unwinding is awesome... but complicated to implement without the help of the std library. *Coughs in soy-dev*.  

The rust_eh_personality is not a linker symbol. It is a language item that points to code that defines how the rust runtime behaves if a panic happens : "does it unwind the stack? How does it unwind the stack? Or does it just refuse to unwind the stack and instead just end program execution?  

To set this language behaviour, we are faced with two solutions :  
1. Tell rust that it should not unwind the stack and instead, it should just abort the entire program.
2. Tell rust that it should unwind the stack... and then offer it a pointer to a function definition that clearly implements the unwinding process. (we are soy-devs, this option is completely and utterly off the table!!)  


## Step 3: Fixing the second compiler error  

The remaining errors were ...
```bash
error: `#[panic_handler]` function required, but not found

error: language item required, but not found: `eh_personality`
  |
  = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
  = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`

error: could not compile `playground` (bin "playground") due to 2 previous errors
```

This is our second error...
```bash
error: `#[panic_handler]` function required, but not found
```

This is our third...
```bash
error: language item required, but not found: `eh_personality`
```



Just like you guessed, the second error occured because the 'rust_begin_panic symbol' has not been defined. We solve this by pinning a '#[panic_handler]' attribute on a divergent function that takes 'panicInfo' as its input. This has been demonstrated below. A divergent function is a function that never returns.  
```rust
#![no_std]

use core::panic::PanicInfo;


#[panic_handler]
// you can name this function any name...it does not matter. eg the_coolest_name_in_the_world
// The function takes in a reference to the panic Info. 
// Kid, go read the docs in core::panic module. It's short & sweet. You will revisit it a couple of times though  
fn default_panic_handler(_info: &PanicInfo) -> !{
    loop {  
        // function does nothing for now, but this is where you write your magic //
        // This is where you typically call an exception handler, or call code that logs the error or panic messages before aborting the program
        // The function never returns, this is an endless loop... The panic_handler is a divergent function
      }
}


fn main(){
    // println!("Hello world!!");
}
```

Would you look at that... if you compile this program, you'll notice that the second compilation error is gone!!!

 

## Step 4: Fixing the Third Error
The third error states that the 'eh_personality' language item is missing.  
It is missing because we have not declared it anywhere... we haven't even defined a stack unwinding function. So we just configure our program to never unwind the stack, that way... defining the 'eh_personality' becomes optional.  

We do this by adding the following lines in the cargo.toml file : 
```toml
# this is the cargo.toml file
[package]
name = "driver_code"
version = "0.1.0"
edition = "2021"

[profile.release]
panic = "abort" # if the program panics, just abort. Do not try to unwind the stack

[profile.dev]
panic = "abort" # if the program panics, just abort. Do not try to unwind the stack
```

Now ... drum-roll... time to compile our program without any errors....  

But then ... out of no-where, we get a new diferent error ... 
```bash 
error: using `fn main` requires the standard library
  |
  = help: use `#![no_main]` to bypass the Rust generated entrypoint and declare a platform specific entrypoint yourself, usually with `#[no_mangle]`
```  

Aahh errors... headaches...  
But at least it is a new error. 🤌🏼🥹  
It's a new error guys!! 🥳💪🏼😎  


<!-- [undone] -->
<!-- - crt0 functions
- crt0 implemetations
- elf board support? How is it implemented?
- triple-targets
- what does target add command actually do and why
- Target support
- Adding custom targets



- THe boot process
- THe esp32 boot process
- Loaders : BIOS, UEFI, U-Boot SPL, CoreBoot
- Runtimes : UEFI, ATF(ARM TRUSTED FIRMWARE)
- BootLoaders : Uboot, Grub, Linux Boot
- firmware standards in the RISCV ISA
- Open SBI
- System V ABI

Bios :
- firmware that sets up environment fit to run a kernel on. It does the following
  - does a power-on-self-test
  - loads the boot loader to memory. The bootloader then loads the Kernel
- Source Material : https://riscv.org/wp-content/uploads/2019/12/Summit_bootflow.pdf
- Multiboot standard -->


[no-std-attribute]: https://doc.rust-lang.org/stable/reference/names/preludes.html#the-no_std-attribute
[core-library]: https://doc.rust-lang.org/core/
[core-library-requirements]: https://doc.rust-lang.org/core/#how-to-use-the-core-library  
[println-macro-doc]: https://doc.rust-lang.org/std/macro.println.html  
[rust-eh-language-item]: https://os.phil-opp.com/freestanding-rust-binary/#the-eh-personality-language-item

<!-- [undone: more info needed on these memory routines specifies as libcore requirements and their integration] -->
