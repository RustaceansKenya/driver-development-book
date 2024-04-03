# Practicals - part 3  

Well, that was a long break.  
Last time in [practicals 2][practicals-2], we got the following compilation error before we had to learn about cross-compilation and linking.  
```bash
error: linking with `cc` failed: exit status: 1
  |
  # some lines have been hidden here for the sake of presentability...   
  = note: /usr/bin/ld: /usr/lib/gcc/x86_64-linux-gnu/11/../../../x86_64-linux-gnu/Scrt1.o: in function `_start':
          (.text+0x1b): undefined reference to `main'
          /usr/bin/ld: (.text+0x21): undefined reference to `__libc_start_main'
          collect2: error: ld returned 1 exit status
          
  = note: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
  = note: use the `-l` flag to specify native libraries to link
  = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)

```
### Analyzing the error
Using the `cross-compilation` and `linking` knowledge, we can dissect the above compilation error.  
```bash
# ...
error: linking with `cc` failed: exit status: 1
# ...
```
`cc` stands for `C Compiler`. It looks like some linking error occured while the C compiler was compiling some files in our codebase. Why the hell is Rust calling a C compiler? Which files need to be compiled using the C compiler?  

Well, if we scroll to some lower sections of the error, we see the `crt1.o` file mentioned. This is an object file that usually gets used as part of the C-runtime. (it must have been written in C, this must be the reason why our compilation process had to summon the C-compiler) - mystery 1 solved.  

Mystery two: Why are C runtime files getting involed in our code-base even after we had added the `#![no_main]` attribute?  
The C runtime files got involved because the Rust compiler still thinks that we are compiling for the host's triple-target. The reason why it still thinks that is because we used the command `cargo build` instead of `cargo build --target=<a_different_target>`  

Compiling for the host's triple-target means that the linker will by default use a pre-defined linker-script that had been made specifically for the host. In my case, the linker-script had information relating to the C runtime and that is how the C-runtime files got involved in the drama.  

To fix this error, we have to stop the usage of that linker-script that has C-runtime affiliations.  
We can do either of the following :  

**Solution 1.** Provide our very own linker-script that does not have affiliations to the C-runtime.  

**Solution 2.** Instruct the linker to stop including C-runtime file symbols to our object files. 

**Solution 3.** We can stop compiling for any target that has a faulty linker-script and instead, only compile for targets that have linker-scripts that do not reference the C-runtime. All triple-targets that have Operating systems specified are more likely to call the C-runtime. All triple-targets that do not have the operating-system specified are less likely to call the C runtime. In short, triple-targets that use the `std` library are out of our radar.  


### Solution 1.  
Solution 1 is about writing our own linker-script as a solution since a manual linker-script usually overides the default auto-generated script.  
I tried it, it didn't work. If anyone cracks this, kindly update this page on github (undone)  

### Solution 2.  
Solution 2 is achieved by running the following command. 
```bash 
cargo rustc -- -C link-arg=-nostartfiles # learn more about these commands by reading the Cargo and Rustc books
```  

### Solution 3.  
Solution 3 can be implemnted by running the command below :  
```bash
cargo build --target=riscv32i-unknown-none-elf
# You can replace `riscv32i-unknown-none-elf` with a target that you have already installed
# The target here should have the value 'none' in place of the Operating system
```

















[practicals-2]: ./pracs_2.md