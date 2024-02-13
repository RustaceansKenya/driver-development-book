# Linking... and Linker scripts

**Stand** :  

Linking is a VERY fundamental topic.  

It is best to learn it **slowly** and **in full** from the [docs](#note-worthy-docs). It will be worth it; You will save yourself hundreds of hours in the future if you make the first read intentional. Be patient with yourself, restrain from skimming through the docs if it's your first time.  

For this reason, this book will not spoil or water-down the purity of the [linking docs](#note-worthy-docs).  

This book will however :
  - assume that you have read [the docs](#note-worthy-docs).
  - Demonstrate how to fix the no-std linking error encountered in the previous chapter
  - Demonstrate how to build a full linker script for the Esp32c3 board.



### Note-worthy docs 
1. Start with this 3-minute video demonstrating the role of the linker from a high level.  
2. Then move to this [doc][gentle-first-docs]. It is gentle, covers the basics and its short.  
3. And finally finish it with this [more detailed docs][more-detailed-second-docs]. The two most important pages there are on [memory description][memory-description] and [memory abstraction][memory-abstraction].


[linker-video]: https://www.youtube.com/watch?v=cJDRShqtTbk
[gentle-first-docs]: https://users.informatik.haw-hamburg.de/~krabat/FH-Labor/gnupro/5_GNUPro_Utilities/c_Using_LD/ldLinker_scripts.html
[more-detailed-second-docs]: https://sourceware.org/binutils/docs/ld/Scripts.html
[memory-description]: https://sourceware.org/binutils/docs/ld/MEMORY.html
[memory-abstraction]: https://sourceware.org/binutils/docs/ld/REGION_005fALIAS.html





<!-- 
undone -- remove this junk, pick whatever you think is important
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
The linker always uses a linker script. If you do not supply one yourself, the linker will use a default script that is compiled into the linker executable.  

There is a default script that comes with the linker. You can see it using the 'ld --verbose' command.  
But this link-script is suited for the pc-host.  
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

 -->
