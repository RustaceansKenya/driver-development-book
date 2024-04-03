# How to build runtimes

- Provides the following attributes for the no-std programer : 
  - #[entry] 
  - #[exception]
  - #[pre_init] to run code before static variables are initialized
  - #[interrupt], which allows you to define interrupt handlers. However, since which interrupts are available depends on the microcontroller in use, this attribute should be re-exported and used from a device crate