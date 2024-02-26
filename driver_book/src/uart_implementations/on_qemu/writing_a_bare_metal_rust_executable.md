# writing_a_bare_metal_rust_executable

references :
1. https://os.phil-opp.com/minimal-rust-kernel/#a-minimal-kernel

## 1. NO_STD

A bare metal executable is a rust program that can run on a piece of hardware without needing an operating system.  

Since we are building our own operating system, we need to write it as a program that is not dependent on another operating system.  
Normal Rust programs depend on the rust standard library. The Rust standard library itself contains functions that call OS-specific system calls. So we cannot use the Rust std library.    

We use the core Rust Library which is not OS-specific.

we add the attribute #![no_std]

## 2. NO_MAIN

Libc is a common C standard library that has been in use for a long time. It has been implemented for very many operating systems.  
Rust is a new language. It is very hard to implement the rust_std for all operating systems. To save on labour and allow compatibility, Rust creators decided to make the Rust Library to use libC functions instead of recreating the functions in pure Rust. Though there are some parts of the Rust std library that do not depend on libc.  

Now that it is clear that rust_std depends on libc, when a rust bin is executed, the following events happen.   
1. The executable program is stored in memory
2. The CPU points to the first instruction of the executable (the etry point). In this case, the entry point is the C runtime.
3. The C runtime sets up its environment in preparation for the libc functions that will get called by the rust_std functions
4. After the C runtime has set up the execution environment for the libc functions, it points to the entry point of the Rust Runtime.
5. The entry point of the Rust Runtime is marked with a language item called "start" ie [start]
6. So the Rust runtime creates an executable environment for executing the Rust functions.  
7. After the Rust runtime has finished setting up things, it looks for the "main" function.
8. Main starts executing

Our bare metal program does not depend on the C runtime. So this sequence of events is quite irrelevant to us.  
What we will do is that we will inform the compiler that we wont follow this sequence by #![no_main] and then declare our own entry point.

To declare our own entry point, we will export a function out of the crate... like so :
```rust
#[no_mangle]
pub extern "C" fn _start()
```

But that is not enough, we need to tell the linker the name of our entry_point function. We do this by writing a linker script.  
The linker will place the code as the first part of the .text section and update the elf header sections to reflect this info.  

```lds
...
OUTPUT_ARCH( "riscv" )


ENTRY( _start )

MEMORY
{
  ram : ORIGIN = 0x80000000, LENGTH = 128M
}
```


## 3. Panic Handler
Rust panics when a violatio happens. Rust requires you to define a function that will always get called after a panic happens.  
That function is tagged by the #[panic_handler] attribute   

The panic_handler function never returns anything, it diverges

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



## 5. Compile to a bare_metal target
You can now compile for the speific target that you want. In our case, it is the `riscv64-unknown-none-elf`.   