# pracs

we are going to write the linker script for an esp32c3.  
So get your datasheet out and plug in that board to your pc.  





#### Random questions you need to think through  
- How does a debugger flash memory?  can you explain it step by step
- Why is it called a ROM if a debugger can write to it?
- Is the ESP32 ROM empty? 
- If it is empty, what does the cpu fetch?  
- If it not empty, what is there? really...
- What is the difference between loadable address and reocatable address?  
- Suppose the debugger manages to flash to the device, does it flash to both the RAM and ROM
- If the loadable address for a section is not specified, it is assumed to be the same as the relocatable address. The loadable address == ROM address, the relocatable_address == RAM. what if the ROM address space is smaller than the RAM address space? What happens? Does the debugger throw an error? Or does it just flash the RAM only?
- wtf is an ABI?
- what is the interface between the CPU and the software?
- what correlation is there between the isa and abi?
- Who reads the elf? the cpu? the os/ a program loader engrained in the firmware? which firmware ... arent we the ones writing that firmware?
- What is the crt0? where do we inject it in our code? what does it do? And dont say 'prepare the environment'
- describe the entire journey frm writing hello world to flashing to running to turning off to restarting the board
- read the elf abi
- Why do we need to produce an elf file? Is an elf file just a way of grouping sections in a standard way?  
- What is the difference between a memory segment and memory region?
- If a debugger is powerful enough to flash a machine, how do people protect their machines from tampering?
- What is the use of the overlay command in a linker script? How does it get implemented on the physical machine? What implements it? When is it a good idea to use the overlay command?
- What is the relation between program headers, sections and memory regions?
- Does the riscv cpu care that you've used an elf? or a specific riscv ABI version?
- When is the PROVIDE keyword necessary?

### more
- apart from mapping sections, what other operations can the linker do?