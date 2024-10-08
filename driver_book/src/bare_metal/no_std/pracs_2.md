# Practicals : Part 2

At the end of the last sub-chapter, we got the following error : 
```bash 
error: using `fn main` requires the standard library
  |
  = help: use `#![no_main]` to bypass the Rust generated entrypoint and declare a platform specific entrypoint yourself, usually with `#[no_mangle]`
```  

Before we solve it, we need to cover some theory... 


## Init code 
'init code' is the code that gets called before the 'main()' function gets called. 'Init code' is not a standard name, it is just an informal name that we will use in this book, but I hope you get the meaning here. Init_code is the code that gets executed in preparation for the main function.  

![Alt text](img/init_code_birds_view.png)

To understand 'init code', we need to understand how programs get loaded into memory.  
When you start your laptop, the kernel gets loaded into memory.  
When you open the message_app in your phone, the message_app gets loaded into memory.  
When you open VLC media player on your laptop, the VLC media player gets loaded into memory (The RAM).  

To dive deeper into this loading business, let's look at how the kernel gets loaded.  


### Loading the kernel.  
When the power button on a machine(laptop) is pressed, the following events occur. (this is just a summary, you could write an entire book on kernel loading): 
1. Power flows into the processor. The processor immediately begins the fetch-execute cycle. Except that the first fetch occurs from the ROM where the firmware is.  

2. So in short, the firmware starts getting executed. The firmware performs a [power-on-self test](https://www.techtarget.com/whatis/definition/POST-Power-On-Self-Test). 

3. The firmware then makes the CPU to start fetching instructions from the ROM section that contains code for the **`primary loader`**. The primary loader in this case is a program that can copy another program from ROM and paste it in the RAM in an orderly pre-defined way. By orderly way I mean ... it sets up space for the the stack, adds some stack-control code to the RAM(eg stack-protection code), and then loads up the different sections of the program that's getting loaded. If the program has [overlays][overlay-explanation-video] - it loads up the code that implements overlay control too.  
Essentially, the loader pastes a program on the RAM in a **complete** way.  

1. The **`primary loader`** loads the **Bootloader** onto the RAM.  
2. The **`primary loader`** then makes the CPU instruction pointer to point to the RAM section where the Bootloader got pasted. This results in the execution of the bootloader.   
3. The Bootloader then starts looking for the kernel code. The kernel might be in the hard-disk or even in a bootable usb-flash.    
4. The Bootloader then copies the kernel code onto the RAM and makes the CPU pointer to point to the entry point of the kernel. An entry-point is the memory address of the first instruction for any program.  
5. From there, the kernel code takes full charge and does what it does best.  
6. The kernel can then load the apps that run on top of it... an endless foodchain.  

**Why are we discussing all these?**  
To show that programs run ONLY because these two conditions get fulfilled: 
1. They were loaded onto either the ROM or the RAM in a **COMPLETE** way.  
The word **Complete** in this context means that the program code was not copied alone; The program code was copied together with control code segments that deal with things like stack control and overlay-control. The action of copying 'control' code onto the RAM is part of **Setting up the environment** before program execution starts.  

    In the software world, this *control* code is typically called [*runtime code*](https://en.wikipedia.org/wiki/Runtime_system).  

2. The CPU's instruction pointer happened to point to the **entry point** of the already loaded program. An entry-point is the memory address of the first instruction for a certain program.  

### Loading a Rust Program
From the previous discussion, it became clear that to run a program, you have to do the following :  
1. load the program to a memory where the CPU can fetch from (typically the RAM or ROM.).
2. load the runtime for the program into memory. The runtime in this case means 'control code' that takes care of things like stack-overflow protection.  
3. make the CPU to point to the entry_point of the (loaded program + loaded runtime)

A typical Rust program that depends on the std library is ran in exactly the same way. The runtime code for such programs includes files from both the C-runtime and the Rust Runtime.  

<figure>
  <img src="./img/init_code_level_2.png" alt="init_code">
  <figcaption><i>Init code vs normal code while in memory.</i></figcaption>
</figure>


<!-- This is the **normal entry point chain**. ☝🏼   -->
When a Rust program gets loaded into memory, it gets loaded together with the C and Rust runtimes.  

#### The normal entry point chain
The `normal entry point chain` describes the order in which code gets executed and their respective entry-point labels.  

In Rust the C-runtime gets executed first, then the Rust runtime and finally the normal code.  
The entry_point function of the C-runtime is the function named `_start`.  

After the C runtime does [its thing][the-c-runtime], it transfers control to the Rust-runtime. The entrypoint of the Rust-runtime is labelled as a `start` language item.  

The Rust runtime also does [its thing][the-rust-runtime] and finally calls the `main` function found in the normal code.  
And that's it! Magic!  

During program execution, the instruction pointer occasionally jumps to appropriate Rust runtime sections. For example, during a panic, the instruction pointer will jump to the `rust_begin_panic` symbol that is part of the Rust runtime.   



### Understanding Runtimes

To understand exactly what the above two runtimes do, read these two chapters below. They are not perfect, but they are a good start.  
1. [The C runtime (CRT-0)][the-c-runtime]
2. [The Rust Runtime][the-rust-runtime]

In summary, the C-runtime does most of the heavy-lifting, it sets up ...(undone).  
The Rust Runtime takes care of some small things such as setting up stack overflow guards or printing a backtrace on panic.  

## Fixing the Error  

To save you some scrolling time, here is the error we are trying to fix.  
```bash
error: using `fn main` requires the standard library
  |
  = help: use `#![no_main]` to bypass the Rust generated entrypoint and declare a platform specific entrypoint yourself, usually with `#[no_mangle]`
```  

This error occurs because we have not specified the entrypoint chain of our program.  
If we had used the std library, the default entry-point chain could have been chosen automatically ie the entry point could have been assumed to be the '_start' symbol that directly references the C-runtime entrypoint.  

To tell the Rust compiler that we don’t want to use the normal entry point chain, we add the `#![no_main]` attribute. Here's a demo : 
```rust
#![no_std]
#![no_main]  // here is the new line. We have added the no_main macro attribute

use core::panic::PanicInfo;

#[panic_handler]
fn default_panic_handler(_info: &PanicInfo) -> !{
    loop { /* magic goes here */ }
}

// main function has just been trashed... coz... why not? It's pointless
```

But when we compile this, we get a linking error, something like this ...
```bash
error: linking with `cc` failed: exit status: 1
  |
  # some lines have been hidden here for the sake of presentability...   
 // = note: LC_ALL="C" PATH="/home/k/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin:/home/k/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin" VSLANG="1033" "cc" "-m64" "/tmp/rustcWMxOew/symbols.o" "/home/k/ME/Repos/embedded_tunnel/driver-development-book/driver_code/target/debug/deps/driver_code-4c11dfa3f10db3d0.f20457jvl65bh2w.rcgu.o" "-Wl,--as-needed" "-L" "/home/k/ME/Repos/embedded_tunnel/driver-development-book/driver_code/target/debug/deps" "-L" "/home/k/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/home/k/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-9686387289eaa322.rlib" "/home/k/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-632ae0f28c5e55ff.rlib" "/home/k/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-3166674eacfcf914.rlib" "-Wl,-Bdynamic" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-L" "/home/k/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/home/k/ME/Repos/embedded_tunnel/driver-development-book/driver_code/target/debug/deps/driver_code-4c11dfa3f10db3d0" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs"
  = note: /usr/bin/ld: /usr/lib/gcc/x86_64-linux-gnu/11/../../../x86_64-linux-gnu/Scrt1.o: in function `_start':
          (.text+0x1b): undefined reference to `main'
          /usr/bin/ld: (.text+0x21): undefined reference to `__libc_start_main'
          collect2: error: ld returned 1 exit status
          
  = note: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
  = note: use the `-l` flag to specify native libraries to link
  = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
```

This error occurs because the toolchain thinks that we are compiling for our host machine and therefore decides to use the default linker script targeting the host machine... which in this case happens to be a linux-mint machine with a x86_64 CPU.  
The reason this is a problem is because that linker script references start files that reference undefined symbols like `__libc_start_main` and `start`.  
To view the default linker script that gets used, you can follow [these steps](undone)


To fix this error, we implement one of the following solutions :
1. Specify a cargo-build for a triple target that has 'none' in its OS description. eg `riscv32i-unknown-none-elf`. This is the easier of the two solutions, and it is the most flexible.  
2. Supply a new linker script that defines our custom entry-point and section layout. If this method is used, the build process will still treat the host's triple-target as the compilation target.   

If the above 2 possible solutions made complete sense to you, and you were even able to implement them, just skim through the next few sub-chapters as a way to humor yourself.  

If they did not make sense, then you got some reading to do in the next immediate sub-chapters...  

Don't worry, we will get to a point where our bare-metal code will run without a hitch... but it's a long way to go.  
The next subchapters will be just theory...we'll fix the compiler error soon.


[overlay-explanation-video]: https://www.youtube.com/watch?v=lWVQsld8hMI
[the-c-runtime]: ../../misc/the_C_runtime.md  
[the-rust-runtime]: ../../misc/the_Rust_runtime.md  
<!-- undone : explain the C runtime in more detail -->
