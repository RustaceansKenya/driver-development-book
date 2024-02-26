## Setting Things Up

Under this chapter, we intend to answer the following 3 questions :
1. What are we setting up?
2. Why are we setting up those things?
3. How are we seting up those things?


### What are we setting up?  
We are setting up the following components : 
1. A development toolchain
2. A RISCV virtual environment 
3. A no-std Rust file.


#### The development Toolchain
A toolchain is a group of software tools that typically get used together...a chain of tools...  
In OS Development, the name toolchain usually refers to the combination of the compiler, linker, debugger and a bunch of programs that help in inspecting files. This toolchain gets used to convert source code into a format that can run on an execution *environment*.

An execution environment is a place where a software program can run. It provides the necessary resources, like the operating system and libraries, that the program needs to function. Examples of execution enviroments include: Bare metal, Browsers, Virtual Machines, Operating systems and Containers.

The toolchain in our case will consist of the following tools :
1. The Rust Nightly Compiler with a riscv64gc-unknown-none-elf backend
2. linker : Rust-lld
3. Binutils 

To our luck, we do not have to install all these elements seperately. There exists compact toolchains :
1. LLVM Riscv toolchain (ignore for now)
2. The GNU Riscv Toolchain (ignore for now)
3. The Rust Toolchain

#### Why we need the toolchain
We will have two kinds of source code files in our project : Rust source files and RISCV-Assembly files. Both of these types of files need to be turned into object files. Afterwards, those object files need to get linked together into a single executable file.  
To do all this, we need a compiler and a linker that can do cross-procesing.  

We can go about this process of creating a single executable file in two ways:  
##### Method 1
We can compile the Rust files seperately from the Assembly files.  
Meaning that we will use do the following actions in order : 
- Use a stand-alone assembler to assemble the RISCV assembly files and turn them into object code.  
- Compile the RUST files into object code using a RUST_compiler.  
- Combine the all the resultant object files from the above 2 steps using a linker to form a single executable. 

##### Method 2
We can embed the assembly code into the Rust source code.  

That way, we only need one compilation, we will only need to compile the asm_embedded Rust files. This method seems more of 'plug and play'.  

The disadvantage of this method is that we will always have to re-compile every file each time we change anything in any source file. But this is not really a problem. Modern compilers are Fast. Using method one will save up a few nano_seconds. A few nanoseconds is cheap price to pay.  

Method 2  is a more user friendly method. Trading off negligible compile time over a user-friendliness in building and tweaking configurations is by far a very good choice.  

Moreover, the rust compiler comes with its own inbuilt LLVM linker, rust-lld. That means that once we hit compile, we get the executable file output. One click, and all the build process runs inbuilt; from compiling rust files, to compiling assembly files, to creating a riscv-compliant executable file.

No more [Makefiles nightmares](https://makefiletutorial.com/), no more. This is very big news.  
For this reason, we will use Method 2.


***references***
- [The LLD official Page](https://lld.llvm.org/)
- [Linking in Rust](https://nnethercote.github.io/perf-book/compile-times.html)

