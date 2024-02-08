### The Standard Library  

The standard library is a group of common function declarations that get called by applications that run on top of an OS.  
So each OS needs to provide implementations for all those common functions.   
For example, the standard library declares the thread_spawn function. Linux OS provides an implementation of that function that is different from the Windows implementation... provided they all do the same thing.  

So when you write drivers, you cannot use the standard library. You can however use the core-library.  
Losing the std library means you forget about threads, files, heap memory, the network, random numbers, standard output, or any other features requiring OS abstractions or specific hardware. If you need them, you have to implement them yourself.  


## Pracs  
It is best to do things practically... you get error messages that engrain into you PTSD.  

### Step 1: Disabling the Std library

By default, rust programs depend on the standard library. To disable this dependence, you add the 'no_std attribute' to your code. The code however switches to depending on the 'core' crate.  
```rust,editable
#![no_std]

fn main(){
    println!("Hello world!!");
}
```

If you run this code, you get 3 compilation errors. 
1. error: cannot find macro `println` in this scope
2. error: `#[panic_handler]` function required, but not found
3. error: unwinding panics are not supported without std

Println macro is part of the standard library. That is why it cannot be found in the scope of the 'no_std' crate.  
If we remove the println line, we still get the remaining two errors.  

#### Fixing the second and third errors.  
This is going to be a short fix but with a lot of theory behind it.  
To solve it, we have to understand the core library requirements. 

The core library functions and definitions can get compiled for any target, provided that the target provides definitions of certain linker symbols. The symbols needed are :
- memcpy, memmove, memset, memcmp, bcmp, strlen [undone: more info needed on these memory routines and their integration]. If your target is supported by rustc, you need not worry about these memory routine symbols.  
- rust_begin_panic
- rust_eh_personality

For now, 

[undone]
- crt0 functions
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
- Multiboot standard
- 

[core-library-requirements]: (https://doc.rust-lang.org/core/#how-to-use-the-core-library)  



