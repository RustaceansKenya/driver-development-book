# Course Outline

If you are attending the driver classes, then this is a rough outline of what we'll be covering.   
We might take 14 days or less...or more...there are no strict limits.  

Here is the link to the class recordings : not_yet_posted


## Beginnings

- [x] Setup environments (wokwi, rust_toolchains, other software(gdb, lldb, probe-rs, esp-rs...))
- [x] intro or something, I don't know (driver, MMIO, pre-requisites)
- [x] no_std stuff
   - cross-compilation, linking, testing, probing, debugging.  
- [x] Uart theory  

## On a NS16550A UART chipset on Qemu 
- [ ] Uart implementation on NS16550A UART chipset on a Qemu device (with naive abstractions)
  - [ ] write linker
  - [ ] write bootloader



## Standard Abstractions
- [ ] Learning Standard Abstractions (PAC, HAL, common safety constructs{typestate_programming, singletons...})
- [ ] RE-building the UART for NS16550A UART chipset with improved abstractions



## Porting driver to the esp32c3 board
- [ ] Understanding the esp32c3 board
  - [ ] datasheet { memory mappings, tooling around flashing,linking,monitoring, logging, debugging }
  - [ ] Understanding the UART implementation of the esp32c3

- [ ] Porting our DIY driver to the esp32c3 board (wokwi, physical)


- [ ] Improve the book, Improve the Code
  