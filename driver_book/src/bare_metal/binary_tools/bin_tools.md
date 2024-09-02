# Binary File Inspection in Rust

We will be dealing with binary files in our development, so it's better to get comfortable with them.  
For an in depth understanding, just read the first 2 chapters of the book recommended at the bottom of the page.  

Oh well, here goes the shallow explanation for those who won't read the book...   

## What's a binary file?  
A binary file is a file filled with zeroes and ones. End of story.  
The 0s and 1s might be machine code or any other data that can be encoded to binary.  
That's it.  

## Types of binary files
Any file can contain a bunch of 0's and 1's but there are some that contain meaningful combinations of 0's and 1's.  
There're many kinds of binary files but we'll just cover binary files that are relevant to us.  

1. An **executable binary** is a binary file that gets produced by a compilation toolchain, this file is fully executable and self-contained. (eg main.out on linux)  

2. A **Relocatable object file** is a specific type of binary file produced by a compiler or assembler. It contains machine code, but it is not a complete executable program. Instead, it represents a single translation unit (e.g, a compiled source file) that needs to be linked with other object files and libraries to create a final executable. (eg lib.o). It is called relocatable because it contains symbols that references memory addresses that are NOT FINAL.  

3. A **dynamic binary file** is a shared library produced by a compilation toolchain too. It is fully compiled and linked, but it is designed to be loaded at runtime rather than being fully self-contained. (e.g., .so on Linux, .dll on Windows)  

## Binary File Formats
A bunch of 0's and 1's can be meaningless unless they are structured, parsable and maybe executable.  

Binary file formats define the structure and encoding of data within a binary file. These formats specify how sequences of bits and bytes are organized to represent different types of data such as executable code, images, or multimedia.  

Common binary file formats include **"Executable and Linkable Format (ELF)"** for unix-like systems and "**Portable Executable (PE)**" for Windows programs, and various proprietary formats for firmware and custom applications. Understanding these formats is key to interpreting and manipulating binary data effectively.  

You can read about Elf file format from specifications, for example, here are some [ELF reference docs from the Linux Foundation][linux-foundation-elf-references].  
Forget about Portable Executable (PE), we wont need it. You can read about it if you wanna.  

## Why would anyone want to inspect binary files?  
People inspect and edit binary files for the same reasons why people inspect and edit source code files -- To improve a program's performance in terms of memory and time, to break it, to crack it, to secure it, to reverse engineer it, to have fun, to show off.  

You can even get a Phd in Binary analysis because it's an entire software field on its own.  

You may want to inspect binary files for whatever reason you may have... but in our case, we inspect them for the following reasons...

## Why a firmware dev might want to inspect/edit binary files
1. Firmware code uses a lot of MMIO programming that references registers and ROM specific addresses. Being able to inspect the binary helps in verifying and editing specific memory addresses.
2. Sometimes you'll deal with memory constrained devices and you'd want to strip the binary, monitor, edit and discard certain binary sections just to reduce binary file size.  
3. Being that you may deal with custom formats of binary, you may have to deal with custom program loaders. Having a good grip over binary file manipulation would be ideal. For example, in a memory constrained device, you may want to create a new way for dynamically overlaying specific segments of the program at runtime.  
4. Sometimes you'll be dealing with firmware whose source code is unavailable... reverse engineering the binary file would be the game. Proprietary hardware & firmware is so common, this may leave you dealing with black-boxes during debugging.   
5. SECURITY. SECURITY. Mf SECURITY. Security is a messy job, you have to protect your firmware from binary-exploiters, so you have to be a good exploiter too. A never-ending tiresome game. On the other hand, you can decide to just ignore security because in real life, firmware devs respect each other's sweat. They understand how much the other dev went through, if they find a bug, they just holla the responsible dev... and the world becomes a better place, just flowers everywhere.      


## Tools for the job

### Hex Editors
Hex editors are programs that can be used to inspect and edit binary files. They can deal with both static and running binary files.  
Let me repeat that, there are hex editors that can view and edit running programs. So imagine all the magic you can do with this.  
You can read about them [here][hex-editor-wikipedia-link].  

If you want to get started on hex editors, have a look at [ImHex][imhex-homepage], it is 
- opensource
- has a nice GUI
- has a good online support(libraries)
- Has many functionalities and is extensible
- has good documentation
- has an inbuilt Pattern Analysis Language that was inspired by C++ & Rust
- crossplatform 

If you want something lightweight you can go with either : 
- Bless (has a simple GUI) 
- Hexyl (it's a CLI made in Rust)

### CMD tools
There are other standard and battle-tested cmd tools that have existed since the 80's.  
These are the real deals.  
Most of them come pre-installed with your linux box. Here's an incomplete list :  

- **Readelf**: Displays information about ELF files, such as section headers, segments and even does some disassembly. You can read its man pages from your command line
- **Objdump**: Provides detailed information about the contents of object files, including disassembly of the code sections.  
- LLVM tools : 
  - **llvm-nm** : used in analyzing symbols and symbol tables in a binary file.
  - **llvm-objcopy** : Copies and translates object files, allowing you to extract, manipulate, or strip specific sections of an object or binary file. It's often used to create stripped-down versions of binaries or firmware images.
  - **llvm-mca** : llvm machine code performance analyzer
  - **objdump** : inspects and disassembles binary files
  - **readobj** : Displays low-level information about object files and executable files, such as headers, sections, and symbols. (similar to readelf except that it's more universal to other known binary file formats)
  - **llvm-size** : analyze the size of different elements in the binary eg sections. 
  - **llvm-strip** : Removes symbols and debugging information from binary files, reducing their size
- GNU tools (look them up, they are mostly similar to LLVM tools)

The above info is shallow, you can access the manuals of each of the tools above and mess around with them.  


## Rust and LLVM-tools
Rust easily integrates with the  LLVM-binary tools

**What do we mean by integration?**  
Parsing and analyzing Binary files can be a headache when building binary tools like llvm-readobj because each target architecture has its own unique intricacies in its binary files even when they use standard file formats such as ELF.  

Rust binaries make it *worse*/*better* by introducing new memory layouts, new symbol-mangling techniques, additional debugging symbols and additional linking symbols just to name a few parsing headaches. (I named a few because my knnowledge ends there, Help! 😂).  

So in short, the resultant rust-made elf files are not really standard elf files, they contain additional rust-specific info. Normal Elf tools like `llvm-readobj` have the ability to parse these rust-made files, but they miss out on the rust-specific analysis.  

For this reason, the Rust community provides modded versions of the LLVM-tools in form of a toolchain componet called `llvm-tools-preview`. This component contains a bunch of modded llvm-tools that can completely parse and inspect both normal and rust-made elf files.  

The word "`preview`" in the name "`llvm-tools-preview`" is important because it indicates that the component is currently not stable and is under active development. You can view the development progress through this [tracking issue][tracking-issue].  

You can add the `llvm-tools-preview` components to your Nightly toolchain by running this command :  
```bash
rustup component add llvm-tools-preview
```

### **Cargo integration**  
To avoid leaving your cargo environment when programming, you can integrate llvm-tools-preview with cargo by running the following command :  
```bash
cargo install cargo-binutils
```

That's it! Now you can churn out commands like these: 
```bash
cargo readobj --all 
cargo objdump --bin hello-world -- -d
cargo objdump --bin hello-world -- --segment
# ... read the cargo-binutils doc ...
```





>This page has over-simplified things. Binary file inspection is a fundamental skill that is best learnt with practice and deep reading.  


>There is this book called : [Binary Analysis for Beginners, build your own linux tools by David Andriesse][binary-analysis-book]. It may not be cutting-edge but it gets you acquainted with the fundamentals in a practical manner.  



[hex-editor-wikipedia-link]: https://en.wikipedia.org/wiki/Hex_editor 
[imhex-homepage]: https://imhex.werwolv.net/  
[linux-foundation-elf-references]: https://refspecs.linuxfoundation.org/  
[tracking-issue]: https://github.com/rust-lang/rust/issues/85658  
[binary-analysis-book]: https://www.amazon.com/Practical-Binary-Analysis-Instrumentation-Disassembly/dp/1593279124  




