# Step one (continuation)

Now we need to use the info gathered from the datasheet to decide on the layout of the object file. 

## Observations and decisions

1. All the grey-shaded areas shown in the Address mapping structure are unusable. This was seen in the address mapping structure. So our linker script should not load any section to those memory regions.
2. The 400KiB SRAM is enough, let us leave the ROM for more permanent core functions that we will write in the future. For now, the SRAM is enough. We will touch on the RTC Memory when we need to preserve data while the CPU is in sleep mode... we don't need such complexity at the moment.  
3. We will store .data and .bss section the SRAM1 section because that section can be accessed using the data-bus.  
4. We will store .text section in the SRAM2 memory region because that section can be accessed by the instruction-bus   
   


