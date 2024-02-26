# Setting up the linker
References : 
- [Guide to Linker Scripting](http://bravegnu.org/gnu-eprog/lds.html)

As earlier mentioned, the Rust compiler comes with an inbuilt linker. 
Each target comes with its own configured linker  

So by default we do not need a linker script. But as for our case, we need a linker script.  

**Why?**	

Reason 1:	
The default compiler does not know the name of your entry_point function. Normally, the linker deals with rust crates that depend on the std libraries, and the entry_point of these crates is "_start"-->"start"-->"main" by default. In our case the linker has no clue. We need to tell it using a linker script.	

Reason 2:
Here is the thing, the elf file has many sections. the global_data section, the heap, the stack, the bss, the text section...	

To write kernel code, you need to know the memory addresses of different elf sections.	
For example....	 
When programming how the kernel heap will get allocated and deallocated, you will need to know the exact memory address of the heap and you will reference it in your allocating function using a pointer. Or when you want the bootloader to point to the text section of the kernel, you will need to give the bootloader the exact memory address of the start of the text section.	 
Point is, to write kernel code, you need to know the memory addresses of the different sections.  

The linker script lets you tell the linker the exact memory addresses where you want it to place the different elf sections. This way, you can make the linker point to the same memory addresses used in your code. You can be assured that the pointers in your code are pointing to some place that you KNOW.  
And the good thing is that the linker lets you label this known memory points using variables.	

If you let the linker decide the memory addresses,you would have to constantly change your code to point to the addresses the linker chose for you. And the linker is not that deterministic. Today it places the heap here, tomorrow there.  

There is a problem with this explanation, the linker only deals with virtual memory. Not Physical memory. It is the job of the BIOS to load the kernel in physical addresses that mao to the virtual addresses.  

Reason 3:	
You may want to make sure the different elf sections are aligned to a certain multiple. For example, if you plan to divide the heap into 4KB blocks, you may prefer to make the heap_start memory address a multiple of 4096

End of reasons...	


So how do we write a Linker Script? And to which linker are we scripting for?

####  Which linker are we scripting for?
The rust gives you an option to choose whichever linker you want to use.  
Rust uses the LLVM Linker by default. So we are currently scripting for the LLVM Linker.   
You may want to use other linkers based on your usecase. For example the LLVM linker is known for its advanced optimizations. The gold linker is optimized for elf files only, so it is lightweight and faster than the GNU linker. Meaning that you will not prefer the gold linker when creating non_elf files.  

To know which linker you are currently using, you can enter the command below :
```bash
rustc --version --verbose
```

You get a result like this :
	rustc 1.70.0-nightly (f63ccaf25 2023-03-06)
    binary: rustc
    commit-hash: f63ccaf25f74151a5d8ce057904cd944074b01d2
    commit-date: 2023-03-06
    host: x86_64-unknown-linux-gnu
    release: 1.70.0-nightly
    LLVM version: 15.0.7

From the above result, you can see That **LLVM linker is used** and specifically version 15.0.7

But each target uses a particular linker flavour, what if you want more information about your current host target? What if you want information about another non_host target ? Use the following command :  
```bash
rustc +nightly -Z unstable-options --target=wasm32-unknown-unknown --print target-spec-json   // for the nightly compiler
OR
rustc  -Z unstable-options --target=riscv64gc-unknown-none-elf --print target-spec-json     //for the stable compiler

```  



You can optionaly specify your linker choice in the build manifest file (configuration file) - cargo.toml as follows :
```toml
[target.'cfg(target_os = "linux")'.llvm]
linker = "/usr/bin/ld.gold"                   //this specifies the path to the gold linker
```
But this is hard work, we are not taking that path. The less configurations we do, the more portable our code, the less headaches we get.

#### How do we write a Linker Script?
Before we explain how to write the linker script, we should answer the question : "Why write the liner script?"  
The linker functions include :
	- Resolving External symbols
	- Section Merging
	- Section Placement

We are writing the linker script so that we can instruct the linker on how it will do the section merging and section placement.  

**Section merging** is the process of combining similar elf sections from different files: For example if A.o and B.o were to be linked together to form C.o then the linker will merge the .text section in both A and B ie.  A.text_section + B.text_section = C.text_section  

**Section placement** is the process of specifying the virtual address of the different sections within the elf file. For example you may place the text section at 0x00 or 0x800... you name it. By default the linker places the different segments in adjacent to each other... but if you do this section placement process manually, you can set paddings between segments or jumble things up.  

You can follow this tutorial [here](http://bravegnu.org/gnu-eprog/lds.html) :

- Tell the linker which architecture you are targeting
- You define the entry address of the elf file
- Define all the memory that we have : RAM and ROM or just one of them 






Here is the Linker script example :
```bash
/*
  define the architecture that the linker understands.  
  for any RISC-V target (64-bit riscv is the name of the architectut or 32-bit).

  We will further refine this by using -mabi=lp64 and -march=rv64gc
*/
OUTPUT_ARCH( "riscv" )

/*
We're setting our entry point to a symbol
called _start which is inside of boot.S. This
essentially stores the address of _start as the
"entry point", or where CPU instructions should start
executing.

In the rest of this script, we are going to place _start
right at the beginning of 0x8000_0000 because this is where
the virtual machine and many RISC-V boards will start executing.
*/
ENTRY( _start )

/*
The MEMORY section will explain that we have "ram" that contains
a section that is 'w' (writeable), 'x' (executable), and 'a' (allocatable).
We use '!' to invert 'r' (read-only) and 'i' (initialized). We don't want
our memory to be read-only, and we're stating that it is NOT initialized
at the beginning.

The ORIGIN is the memory address 0x8000_0000. If we look at the virt
spec or the specification for the RISC-V HiFive Unleashed, this is the
starting memory address for our code.

Side note: There might be other boot ROMs at different addresses, but
their job is to get to this point.

Finally LENGTH = 128M tells the linker that we have 128 megabyte of RAM.
The linker will double check this to make sure everything can fit.

The HiFive Unleashed has a lot more RAM than this, but for the virtual 
machine, I went with 128M since I think that's enough RAM for now.

We can provide other pieces of memory, such as QSPI, or ROM, but we're
telling the linker script here that we have one pool of RAM.
*/
MEMORY
{
  ram   (wxa!ri) : ORIGIN = 0x80000000, LENGTH = 128M
}

/*
PHDRS is short for "program headers", which we specify three here:
text - CPU instructions (executable sections)
data - Global, initialized variables
bss  - Global, uninitialized variables (all will be set to 0 by boot.S)

The command PT_LOAD tells the linker that these sections will be loaded
from the file into memory.

We can actually stuff all of these into a single program header, but by
splitting it up into three, we can actually use the other PT_* commands
such as PT_DYNAMIC, PT_INTERP, PT_NULL to tell the linker where to find
additional information.

However, for our purposes, every section will be loaded from the program
headers.
*/
PHDRS
{
  text PT_LOAD;   
  data PT_LOAD;
  bss PT_LOAD;
}

/*
We are now going to organize the memory based on which
section it is in. In assembly, we can change the section
with the ".section" directive. However, in C++ and Rust,
CPU instructions go into text, global constants go into
rodata, global initialized variables go into data, and
global uninitialized variables go into bss.
*/
SECTIONS
{
  /*
    The first part of our RAM layout will be the text section.
	Since our CPU instructions are here, and our memory starts at
	0x8000_0000, we need our entry point to line up here.
  */
  .text : {
	  /* In the GNU Linker Script Language, the PROVIDE keyword instructs the linker to declare a new symbol and assign it a value 

	    PROVIDE allows me to create a symbol called _text_start so
		I know where the text section starts in the operating system.
		This should not move, but it is here for convenience.
		The period '.' tells the linker to set _text_start to the
		CURRENT location ('.' = current memory location). This current
		memory location moves as we add things.
	  */

    PROVIDE(_text_start = .);
	/*
	  We are going to layout all text sections here, starting with 
	  .text.init. 
	  The asterisk in front of the parentheses means to match
	  the .text.init section of ANY object file. Otherwise, we can specify
	  which object file should contain the .text.init section, for example,
	  boot.o(.text.init) would specifically put the .text.init section of
	  our bootloader here.

	  Because we might want to change the name of our files, we'll leave it
	  with a *.

	  Inside the parentheses is the name of the section. I created my own
	  called .text.init to make 100% sure that the _start is put right at the
	  beginning. The linker will lay this out in the order it receives it:

	  .text.init first
	  all .text sections next
	  any .text.* sections last

	  .text.* means to match anything after .text. If we didn't already specify
	  .text.init, this would've matched here. The assembler and linker can place
	  things in "special" text sections, so we match any we might come across here.
	*/
    *(.text.init) *(.text .text.*)

	/*
	  Again, with PROVIDE, we're providing a readable symbol called _text_end, which is
	  set to the memory address AFTER .text.init, .text, and .text.*'s have been added.
	*/
    PROVIDE(_text_end = .);
	/*
	  The portion after the right brace is in an odd format. However, this is telling the
	  linker what memory portion to put it in. We labeled our RAM, ram, with the constraints
	  that it is writeable, allocatable, and executable. The linker will make sure with this
	  that we can do all of those things.

	  >ram - This just tells the linker script to put this entire section (.text) into the
	         ram region of memory. To my knowledge, the '>' does not mean "greater than". Instead,
			 it is a symbol to let the linker know we want to put this in ram.

	  AT>ram - This sets the LMA (load memory address) region to the same thing.this linker script, we're loading
			   everything into its physical location. We'll l LMA is the final
	           translation of a VMA (virtual memory address). With et the kernel copy and sort out the 
			   virtual memory. That's why >ram and AT>ram are continually the same thing.

	  :text  - This tells the linker script to put this into the :text program header. We've only
	           defined three: text, data, and bss. In this case, we're telling the linker script
			   to go into the text section.
	*/
  } >ram AT>ram :text
   /*
     The global pointer allows the linker to position global variables and constants into
	 independent positions relative to the gp (global pointer) register. The globals start
	 after the text sections and are only relevant to the rodata, data, and bss sections.
   */
   PROVIDE(_global_pointer = .);
   /*
     Most compilers create a rodata (read only data) section for global constants. However,
	 we're going to place ours in the text section. We can actually put this in :data, but
	 since the .text section is read-only, we can place it there.

	 NOTE: This doesn't actually do anything, yet. The actual "protection" cannot be done
	 at link time. Instead, when we program the memory management unit (MMU), we will be
	 able to choose which bits (R=read, W=write, X=execute) we want each memory segment
	 to be able to do.
   */
  .rodata : {
    PROVIDE(_rodata_start = .);
    *(.rodata .rodata.*)
    PROVIDE(_rodata_end = .);
	/*
	   Again, we're placing the rodata section in the memory segment "ram" and we're putting
	   it in the :text program header. We don't have one for rodata anyway.
	*/
  } >ram AT>ram :text

  .data : {
	/*
	   . = ALIGN(4096) tells the linker to align the current memory location (which is
	   0x8000_0000 + text section + rodata section) to 4096 bytes. This is because our paging
	   system's resolution is 4,096 bytes or 4 KiB.

	   As a result, the current memory address is rounded off to the next nearest address that has a value that is a multiple of 4096
	*/
    . = ALIGN(4096);
    PROVIDE(_data_start = .);
	/*
	   sdata and data are essentially the same thing. However, compilers usually use the
	   sdata sections for shorter, quicker loading sections. So, usually critical data
	   is loaded there. However, we're loading all of this in one fell swoop.
	   So, we're looking to put all of the following sections under the umbrella .data:
	   .sdata
	   .sdata.[anything]
	   .data
	   .data.[anything]

	   ...in that order.
	*/
    *(.sdata .sdata.*) *(.data .data.*)
    PROVIDE(_data_end = .);
  } >ram AT>ram :data

  .bss : {
    PROVIDE(_bss_start = .);
    *(.sbss .sbss.*) *(.bss .bss.*)
    PROVIDE(_bss_end = .);
  } >ram AT>ram :bss

  /*
     The following will be helpful when we allocate the kernel stack (_stack) and
	 determine where the heap begins and ends (_heap_start and _heap_start + _heap_size)/
	 When we do memory allocation, we can use these symbols.

	 We use the symbols instead of hard-coding an address because this is a floating target.
	 Floating target means that the address space layout keeps on changing, do it becomes hard to hardcode physical adresses.
	 The heap size is not known at compile time
	 As we add code, the heap moves farther down the memory and gets shorter.

	 _memory_start will be set to 0x8000_0000 here. We use ORIGIN(ram) so that it will take
	 whatever we set the origin of ram to. Otherwise, we'd have to change it more than once
	 if we ever stray away from 0x8000_0000 as our entry point.
  */
  PROVIDE(_memory_start = ORIGIN(ram));
  /*
     Our kernel stack starts at the end of the bss segment (_bss_end). However, we're allocating
	 0x80000 bytes (524 KiB) to our kernel stack. This should be PLENTY of space. The reason
	 we add the memory is because the stack grows from higher memory to lower memory (bottom to top).
	 Therefore we set the stack at the very bottom of its allocated slot.
	 When we go to allocate from the stack, we'll subtract the number of bytes we need.
  */
  PROVIDE(_stack = _bss_end + 0x80000);
  PROVIDE(_memory_end = ORIGIN(ram) + LENGTH(ram));

  /* 
     Finally, our heap starts right after the kernel stack. This heap will be used mainly
	 to dole out memory for user-space applications. However, in some circumstances, it will
	 be used for kernel memory as well.

	 We don't align here because we let the kernel determine how it wants to do this.
  */
  PROVIDE(_heap_start = _stack);
  PROVIDE(_heap_size = _memory_end - _stack);
}

```

Our Linker script is ready !!!
