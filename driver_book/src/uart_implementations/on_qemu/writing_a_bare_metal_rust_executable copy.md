# Recap on writing a bare_metal_rust_executable


## 1. NO_STD

A bare metal executable is a rust program that can run on a piece of hardware without needing an operating system.  

Since we are building a driver, we need to write it as a program that is not dependent on an operating system.  
Normal Rust programs depend on the rust standard library. The Rust standard library itself contains functions that call OS-specific system calls. So we cannot use the Rust std library.    

We use the core Rust Library which is not environment-specific. The core library is dependency-free. It's only requirement is that the programmer provides the definitions of some linker symbols and language items. 

To disable the std-dependence, we add the crate attribute `#![no_std]` to our project. 

## 2. NO_MAIN

Libc is a common C standard library that has been in use for a long time. It has been implemented for very many operating systems.  
Rust is a new language. It is very hard to implement the rust_std for all operating systems. To save on labour and allow compatibility, Rust creators decided to make the Rust Library to use libC functions instead of recreating the functions in pure Rust. Though there are some parts of the Rust std library that do not depend on libc.  

Now that it is clear that rust_std depends on libc, when a rust bin is executed, the following events happen.   
1. The executable program is stored in the main memory (eg RAM)
2. The CPU points to the first instruction of the executable (the entry point). In this case, the entry point is the `_start` function found in the C runtime.
3. The C runtime sets up its environment in preparation for the libc functions that will get called by the rust_std functions
4. After the C runtime has set up the execution environment for the libc functions, it points to the entry point of the Rust Runtime.
5. The entry point of the Rust Runtime is marked with a language item called "start" ie [start]
6. So the Rust runtime creates an executable environment for executing the Rust functions.  
7. After the Rust runtime has finished setting up things, it looks for the "main" function.
8. Main starts executing

Our bare metal program does not depend on the C runtime. So this sequence of events is quite irrelevant to us.  
What we will do is that we will inform the compiler that we wont follow this sequence by : 
1. Adding the `#![no_main]` crate attribute to our project.
2. Declaring our own entry point function

To declare our own entry point, we will export a function out of the crate... like so :
```rust
#[no_mangle] // The no_mangle attribute explained below
pub extern "C" fn _start()

// Mangling is a technique used by compilers to encode the names of 
// functions, methods, and other symbols in a program in a way that includes additional information beyond just the name itself. 

// For example `main` may become `main21212jxbjbjbkjckbdsc&kbjbjksdbdjkbf`
// The primary purpose of mangling is to make sure that each variable or function is completely unique to
// the point that there are no name-conflicts during compilation and linking.  
// This also enables function overloading 

//In Rust, the #[no_mangle] attribute is used to instruct the compiler not to mangle the name of the item ...
// (function or static variable) during compilation. This is useful when you want to interface with external
//  code, like C code or assembly code, where the function names need to remain unchanged.  

// We want "_start" to be referenced as it is. We cannot gamble with the identity such a symbol name
```

But that is not enough, we need to tell the linker the name of our entry_point function. We do this by writing a linker script that uses the `ENTRY` command.  
The linker will place the code as the first part of the .text section and update the elf header sections to reflect this info.  

```lds
...
OUTPUT_ARCH( "riscv" )


ENTRY( _start )  /* See? we have used the name `_start` just like it is. If name mangling had happened, we would have had some random name that changes with every compilation.  

MEMORY
{
  ram : ORIGIN = 0x80000000, LENGTH = 128M
}
```


## 3. Panic Handler
Rust runtime panics when a violation happens. Rust requires you to define a function that will always get called after a panic happens.  
That function is tagged by the #[panic_handler] attribute.   

The panic_handler function never returns anything, it diverges. It is therefore a divergent function.  

```rust
use core::panic::PanicInfo;
#[panic_handler]
fn my_custom_function( panic_info: &PanicInfo)-> !{
    println!("message : {}", panic_info.message())
    println!("location : {}", panic_info.location())
}
```

## 4. The eh_personality  (aka error_handling personality)

Rust requires you to define a function that will always get called when it wants to unwind and free a stack.    
This function is tagged with #[eh_personality] attribute.   

When a panic happens, the program stops (theoretically). The program can decide to free the stack or just abort and let the underlying OS clear the stack.      
The thing is, to clear the stack, you have to unwind it. To unwind the stack, you have to use some hard functions...Functions that depend on some OS functionalities. This is a chicken-egg problem.  

So we resort to aborting.   

To specify this behaviour, you can tweak the cargo file as follows : 
```toml
[profile.release]
panic = "abort"

[profile.dev]
panic = "abort"
```

By default the settings are usually :
```toml
[profile.release]
panic = "unwind"

[profile.dev]
panic = "unwind"
```

Now, the #[eh_personality] tag is a tag that is pegged to the function that gets called when a rust program wants to unwind its stack. eg
```rust
#[eh_personality]
fn custom_unwind(){
    // do some unwinding statements ... MAgiC!
}
```

BUT since we have specified that our program will always abort... AND that it will never call the unwind function, we are no longer required to define the unwinding function



## 5. Compile for a bare_metal target
You can now compile for the speific target that you want. In our case, it is the `riscv64-unknown-none-elf`.   
To get a recap on how to perform cross-compiltion, re-visit [this chapter](../../bare_metal/cross_compilation/cross_compilation.md)


## Template Link


