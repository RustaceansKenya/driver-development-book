# Why is my program not flashing?

>Does your program use a correct linker script? Does the script specify a memory layout that conflicts with the Esp32c3 memory layout? File too big for certain memory sections? Have you assigned sections to memory regions that were flagged as unreachable?  

To test that out use the linker sript and src_code found in the fixup folder.  


Did you specify the right debug protocol? JTAG vs SWD?  

