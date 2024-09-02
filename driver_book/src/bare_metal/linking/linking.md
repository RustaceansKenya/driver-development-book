# Linkers and Linking


Linking is a VERY fundamental topic.  

It is best to learn it **slowly** and **in full** from the [docs](#note-worthy-docs) listed below.     
For this reason, this book will not spoil or water-down the purity of the [linking docs](#note-worthy-docs).  


### Note-worthy docs 
1. Start with this [3-minute video][linker-video] demonstrating the role of the linker from a high level.  
2. Then move to this [doc][gentle-first-docs]. It is gentle, covers the basics and its short.  
3. And finally finish it with this [more detailed docs][more-detailed-second-docs]. The two most important pages there are on [memory description][memory-description] and [memory abstraction][memory-abstraction].

## linkers in Rust
There are many linkers in existence. However the two dominant linkers are :  
1. The [LD linker][ld-linker-page] (also called the GNU linker)
2. The [LLD linker][lld-linker-page] (also called the LLVM linker)  

The Rust toolchain is built using the LLVM toolchain, so it uses the LLVM linker by default. You can however configure it to use the GNU linker after some tweaks to the cargo configuration file.  

<br><br>
The GNU linker and the LLVM linker have two subtle and important differences have been listed below.  

### 1. Automatic linker-script generation.  
The GNU linker ALWAYS requires a manually-defined linker script to function while the LLD (the LLVM linker) doesn't always require a manually-defined linker script to function.   

In many cases, LLD can automatically generate linker scripts internally based on the specified triple-target, format, and other parameters specified before & during the linking process. This means that LLD can handle the linking process without requiring an explicit linker script provided by the user.

However, LLD does provide a way for users to specify custom linker scripts if needed. Users can pass a custom linker script to LLD using command-line options or configuration files, similar to how it's done with LD. This gives users flexibility in defining the linking behavior and organizing the output binary according to their specific requirements.  

### 2. Cross linking and the existence of flavours
The GNU linker is compact and straight-foward. There is only one GNU linker. If you want to compile something into an elf, you supply the linker with an elf-generating linker script. If you need a wasm binary file, you supply it with a corresponding linker script.  
This may seem simple at first, but writing a correct linker script is usually not an easy task. To solve this problem, the LLVM linker implemented the concept of ***linker-flavours***.  

The LLVM linker is not a monolith, it is made up of different specialized linkers that are typically called flavours. The flavours produce object files for specific targets ONLY.  

For example, Let's say you want to produce a unix elf file, instead of writing a complex & erronous linker script, you could use the `LD.LLD linker flavour` and it will automatically generate an internal unix-elf-focussed script for you. This is what makes LLD a cross-linker by default.  

There are currently 4 mainstream LLVM-linker flavours : 
1. **LD.LLD (unix)** : specializes in generating object files and executables for Unix-like operating systems, such as Linux and FreeBSD. It supports formats like ELF (Executable and Linkable Format) and handles symbol resolution, linking libraries, and generating debug information specific to Unix environments.  

2. **ld64.lld (macOS)** : specializes in producing object files and executables for macOS and other Apple platforms. It supports the Mach-O (Mach Object) file format used on macOS. 

3. **lld-link (Windows)** : specializes in generating object files and executables for Windows-based systems. It supports the PE (Portable Executable) file format used on Windows, handles symbol resolution, and integrates with Windows-specific tools and libraries for linking applications and generating executables compatible with the Windows environment.  

4. **wasm-ld (WebAssembly)** : This flavour is a work in progress. It specializes in producing WebAssembly (Wasm) modules and executables that follow wasm specifications.  

## Implications of those two subtle differences

1. It becomes easy to make the Rust toolchain to be able to compile and link for new targets. The `rustup target add` command literally [does this][target-add-description]. 

2. Declaring linker scripts becomes optional.  

To view the defult lld flavour of a supported target, run the following command :  
```bash
# Replace `riscv32i-unknown-none-elf` with a target of your liking
rustc -Z unstable-options --target riscv32i-unknown-none-elf --print target-spec-json
```  
Feedback : 
```bash
{
  "arch": "riscv32",
  "atomic-cas": false,
  "cpu": "generic-rv32",
  "crt-objects-fallback": "false",
  "data-layout": "e-m:e-p:32:32-i64:64-n32-S128",
  "eh-frame-header": false,
  "emit-debug-gdb-scripts": false,
  "features": "+forced-atomics",
  "is-builtin": true,
  "linker": "rust-lld",  # HERE is the linker name... it could have been something like ld
  "linker-flavor": "gnu-lld",  # HERE is the linker Flavour
  "llvm-target": "riscv32",
  "max-atomic-width": 32,
  "panic-strategy": "abort",
  "relocation-model": "static",
  "target-pointer-width": "32"
}
```


[lld-linker-page]: https://lld.llvm.org/
[ld-linker-page]: https://ftp.gnu.org/old-gnu/Manuals/ld-2.9.1/html_mono/ld.html
[linker-video]: https://www.youtube.com/watch?v=cJDRShqtTbk
[gentle-first-docs]: https://users.informatik.haw-hamburg.de/~krabat/FH-Labor/gnupro/5_GNUPro_Utilities/c_Using_LD/ldLinker_scripts.html
[more-detailed-second-docs]: https://sourceware.org/binutils/docs/ld/Scripts.html
[memory-description]: https://sourceware.org/binutils/docs/ld/MEMORY.html
[memory-abstraction]: https://sourceware.org/binutils/docs/ld/REGION_005fALIAS.html
[target-add-description]: ../../misc/rustup_target_add.md