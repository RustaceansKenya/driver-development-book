# Fixing the previous linker error

So far, our bare-metal program looks like this :  
```rust
// this is the main.rs file
#[no_std]
#[no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn defaultpanic_handler (_info: &PanicInfo) -> ! {
    // magic code
    loop { /* an endless loop that makes this function to be divergent */ }
}
```  

We get this error message when we run the 'cargo build' command.: 
```bash
error: linking with `cc` failed: exit status: 1
  |
  = note: LC_ALL="C" PATH="/home/k/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin:/home/k/.cargo bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin" VSLANG="1033" "cc" "-m64" "/tmp/rustcrOjhe9/symbols.o" "/home/k/ME/Repos/embedded_tunnel/driver-development-book/driver_code/target/debug/deps/driver_code-4c11dfa3f10db3d0.f20457jvl65bh2w.rcgu.o" "-Wl,--as-needed" "-L" "/home/k/ME/Repos/embedded_tunnel/driver-development-book/driver_code/target/debug/deps" "-L" "/home/k/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/home/k/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-9686387289eaa322.rlib" "/home/k/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-632ae0f28c5e55ff.rlib" "/home/k/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-3166674eacfcf914.rlib" "-Wl,-Bdynamic" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-L" "/home/k/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/home/k/ME/Repos/embedded_tunnel/driver-development-book/driver_code/target/debug/deps/driver_code-4c11dfa3f10db3d0" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs"
  = note: /usr/bin/ld: /usr/lib/gcc/x86_64-linux-gnu/11/../../../x86_64-linux-gnu/Scrt1.o: in function `_start':
          (.text+0x1b): undefined reference to `main'
          /usr/bin/ld: (.text+0x21): undefined reference to `__libc_start_main'
          collect2: error: ld returned 1 exit status
          
  = note: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
  = note: use the `-l` flag to specify native libraries to link
  = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)  

```  

Armed with the knowledge from the previous 2 sub-chapters, we can break-down the above error message into the following facts :  
1. The compilation process used the ld linker
2. The linker script used was meant for a `x86_64-unknown-linux-gnu` triple-target.  
3. The linker script used expects us to have a `main` function defined.  

All the above facts go against what we originally set out to do : 
1. To Build a file that does not depend on any OS. `x86_64-unknown-linux-gnu` object files rely on the existence of a Linux OS as an execution environment.  
2. We also just yeeted the default entry-point chain, so we cannot stick to a linker script that still requires us to provide a `main` function
 

To solve this, we change the compilation target for our project to something that does not specify an OS execution environment. For example : `riscv32i-unknown-none-elf`  or `x86_64-unknown-none`.  

We can do that by either passing on a compilation parameter : 
```bash
cargo build --target=riscv32i-unknown-none-elf
```

Or we can modify the .cargo/config.toml file :  
```toml
[build]
target = "riscv32imc-unknown-none-elf"


[target.riscv32imc-unknown-none-elf]
linker = "ld.lld"
```


And we are DONE!!!  
Now all we have to do is to load our compiled file to a bare-metal CPU... for example, the esp32c3.  
The esp32c3's target specification is `riscv32imc-unknown-none-elf`.  


# Disclaimer

Our code did not declare an explicit Entry point. The entry point was implicitly defined during the linking proccess.  
To confirm this, inspect your executable file using `readelf` cmd tool like this : 
```bash
readelf -h ./target/riscv32i-unknown-none-elf/debug/your_executable_file
```

You will get a response like the one below :  
```bash
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00 
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           RISC-V
  Version:                           0x1

  # LOOK HERE; the Entry Point has been assumed to be address Zero.
  Entry point address:               0x0  


  Start of program headers:          52 (bytes into file)
  Start of section headers:          5104 (bytes into file)
  Flags:                             0x0
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         4
  Size of section headers:           40 (bytes)
  Number of section headers:         12
  Section header string table index: 10
```

We will define our own entry point when it actually matters, for now we can ignore this.  



