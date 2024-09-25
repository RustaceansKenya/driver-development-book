# Abstracting a Peripheral  

We will build a Peripheral Access Layer(PAC) above two different peripherals: A system timer and the UART.  

We've chosen the System timer because it is simple - It has only 4 registers and has few functionalities. We will use the timer as practice ground before moving on to the UART which has more registers and functionalities.  

Here is the source-code for both the timer and UART PACs
1. System Timer Code
2. UART code


Go read this chapter from [the Embedded Rust Book](https://docs.rust-embedded.org/book/peripherals/a-first-attempt.html). The SysTick example is comparatively simple.  

Here we go....