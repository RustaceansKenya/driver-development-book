# Abstractions  

Now we have understood the registers of the UART device. It's time to build software above it.   


You can read these 2 sources on abstraction layers above the hardware:  
1. Embedded Rust Book - [Mapped Registers Chapter](https://docs.rust-embedded.org/book/start/registers.html)
2. Embedded Rust Book - [Portability Chapter](https://docs.rust-embedded.org/book/portability/)


As with other chapters in this book, I will just go on ahead and summarize the above docs in an inaccurate manner.  
Spot the lies.  

Roughly speaking, here are 5 layers of hardware abstraction arranged from most-low-level to most-high-level.  

| Abstraction Layer Name  | level     |
|-------------------------|-----------|
| Microarchitecture crate | (level 1) |
| Peripheral Access Crate | (level 1) |
| Hardware Access Layer   | (level 2) |
| Board Access Layer      | (level 3) |
| Runtime Layers          | (level 4) |
|                         |           |

<br><br>
Here is a pic extracted from the Embedded Rust Book :  
<figure>
  <img src="img/abstration_crates.png" alt="Abstraction crates over Hardware">
  <figcaption>Abstraction crates over Hardware</figcaption>
</figure>


## MicroArchitechture crates (MACs)
This crate abstracts away the processor Architecture itself. It's API provides things like : 
- rust-wrapped assembly commands 
- provides a framework to manage interrupts (as prescribed by the processor)
- provides a way to abstract critical sections as implemented by the Architecture.  

Examples include of MAC craretes include : [cortex-m](https://crates.io/crates/cortex-m) and [riscv](https://crates.io/crates/riscv)

## Peripheral Access Crates (PACs)
These crates abstract away the registers of the physical devices. They abstract away peripherals that come attached to the processor. Eg the UART, I2C and USB modules.   

In other words.. this kind of crate is a thin wrapper over the various memory-wrapper registers defined for your particular part-number of micro-controller you are using. 

Some peripherals are sometimes considered to be a part of the micro-architecture instead of independent peripherals eg the system-timer in cortex boards. As a result, the crate above the system-timer becomes part of the MAC instead of the PAC. The line between MACs and PACs is vague... you could say that the MAC is a PAC above the Micro-processor!    

Examples include:   [tm4c123x](https://crates.io/crates/tm4c123x), [stm32f30x](https://crates.io/crates/stm32f30x) 

## Hardware Access Layer
The hardware access layer is an API that tries to abstract away the combination of both PACs and MACs. It summarizes its API into a group of traits that represent functions generically found in most processors.   

We will look at the embedded-hal in a future chapter (undone) 

## Board Access Layer
This board builds upon the Hardware Access Layer by configuring the API exposed by the HAL to suit a specific board. We will also look at this in a future chapter. (undone)

## Runtime Layers
The Runtime layer is more of a hypervisor... or even a kernel. It manages things like concurrent use of the hardware below. It takes care of booting (or even safe booting), flashing-safety, logging, loading, debugging & updating of firmware, ...  

To put it inaccurately, it is a small minimal kernel over your board.  So really, in reality we cannot say what its definite functions are - it is up to the runtime library creator to figure that out.  


We will try to build all these layers as we go.  