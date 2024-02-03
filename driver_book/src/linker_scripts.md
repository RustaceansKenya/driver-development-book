# Linker scripts

You can read about linker scripts here :
1. [this page](https://users.informatik.haw-hamburg.de/~krabat/FH-Labor/gnupro/5_GNUPro_Utilities/c_Using_LD/ldLinker_scripts.html#PROVIDE_command)
2. and [this other page](https://sourceware.org/binutils/docs/ld/Scripts.html)


examples :
- GNU Binutils Linker (ld)
- The LLVM Linker (lld) - preferred because it is a predecessor of ld. Similar to ld in terms of commands and scripting landuage. faster. Modular. Documented.

But I'll stick with ld then move on to lld later on. 


To check the default linker for your target... check the target-spec-json :
```bash
rustc -Z unstable-options --target riscv32i-unknown-none-elf --print target-spec-json
```

It is not guaranteed that the default linker will always be used, so check the cargo.toml file for assurance


### Why do we need a linker-script
A linker script helps organize the memory layout of the object file(s).  

### Why do we need to write linker script each time, can't we have a default?  
There is a default script that comes with the linker. You can see it using the 'ld --verbose' command.  
But this linker is suited for the pc-host.  
So if we are compiling an object file for a different host, we need to define a linker script for the new target.   


You may supply your own linker script by using the ` -T ' command line option. for example :  
```bash
ld -T
```

You can also add link linker-scripts as input files, but they wont erplace the default linker-script.   



### Common Inspections

Tools : objdump, nm

- ld -h file  // view all the sections, and their neat matadata
- ld -t file  // view the symbol table

- nm - list symbols from object files
- Run 'man nm'
-  nm -C ./target/debug/riscv_metal  // view the symbol table while things are demangled


