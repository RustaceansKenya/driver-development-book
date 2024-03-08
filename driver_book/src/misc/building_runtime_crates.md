Reference crates : 
- cortex-m-rt
- riscv-rt


- combiation of startupcode + runtime code


### Functions of startup code 

- Sets up the memory layout of the program. In particular, it populates the vector table so the device can boot correctly, and properly dispatch exceptions and interrupts.

- Initializing static variables before the program entry point.

- Enabling the FPU before the program entry point if the target is thumbv7em-none-eabihf

- Provides the following attributes for the no-std programer : 
  - #[entry] 
  - #[exception]
  - #[pre_init] to run code before static variables are initialized
  - #[interrupt], which allows you to define interrupt handlers. However, since which interrupts are available depends on the microcontroller in use, this attribute should be re-exported and used from a device crate