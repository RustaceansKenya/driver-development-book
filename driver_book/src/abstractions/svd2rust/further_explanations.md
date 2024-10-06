# svd2rust further explanations


This is a cheap summary from the svd2rust docs.  



From the docs :  



### Generating an ISA-specific PAC crate 

`svd2rust` can generate PAC crates that are either ISA-specific OR ISA-agnostic.  
Currently supported ISAs are: Riscv, Cortex-m, MSP430, Xtensa and LX6.  

**What is an ISA-specific PAC crate?**  
This is a PAC crate that...  (undone)

To generate a PAC crate for a particular supported micro-controller, you can run one of the following commands. (undone)
```bash

``` 

### Generating an ISA-agnostic PAC crate
To generate a PAC crate for a custom micro-controller, you can ... (undone)

### Atomics

(undone)
- flag : --atomics (Generate atomic register modification API)
- flag : --atomics-feature <FEATURE> (add feature gating for atomic register modification API)

### Enumerated field values
(undone)

### Logging and debugging
(undone)
- flag:  impl-defmt <FEATURE>
- log <log_level> Choose which messages to log (overrides RUST_LOG) [possible values: off, error, warn, info, debug, trace]
- flag: impl-debug
- flag: impl-debug-feature <FEATURE>
  
### Documenting
- flag: --html-url <URL>
- arranging files  

### Combining configs
- flag: --config <TOML_FILE>  

### File Organization
- generic.rs
- build.rs, build script that places device.x somewhere the linker can find
- device.x, linker script that weakly aliases all the interrupt handlers to the default exception handler (DefaultHandler)
- lib.rs
- group
- rustfmt and form

### Runtime feature  
What the runtime feature does: 
1. It Imports the error handlers defined in files found in the rustc search path and includes them as part of the crate's namespace. It does this through an extern import like so...
```rust
#[cfg(feature = "rt")]
extern "C" {
    fn WIFI_MAC();
    fn WIFI_MAC_NMI();
    fn WIFI_PWR();
    fn WIFI_BB();
    fn BT_MAC();
    fn BT_BB();
    fn BT_BB_NMI();
    fn RWBT();
    fn RWBLE();
    fn RWBT_NMI();
    fn RWBLE_NMI();
    // other error handler functions have been snipped ....
}
```  
The names of these event handlers have beed declared in the `device.x` file.  
If you look at the `device.x` file, you will notice that the Interrupt_symbols have been weakly defined using the `PROVIDE` linker keyword. The symbols have been given the value ``DefaultHandler`
2. After importing those error handlers into the namespace, it encloses them in a static public vector-array (function array) for the sole purpose of making it easy for the linker to locate the Interrupt symbols.  
This is achieved through the same old trick of demangling symbols and making them globally accessible like this...
```rust
#[repr(C)]
pub union Vector {
    pub _handler: unsafe extern "C" fn(),
    pub _reserved: usize,
}

#[cfg(feature = "rt")]
#[doc(hidden)]
#[no_mangle]
pub static __EXTERNAL_INTERRUPTS: [Vector; 62] = [
    Vector { _handler: WIFI_MAC },
    Vector { _handler: WIFI_MAC_NMI },
    Vector { _handler: WIFI_PWR },
    Vector { _handler: WIFI_BB },
    Vector { _handler: BT_MAC },
    Vector { _handler: BT_BB },
    Vector { _handler: BT_BB_NMI },
    Vector { _handler: RWBT },
    // ...insert other vectors here
]
```



Taking Peripherals needs to be atomic. Some boards do not support atomics, so the best way is to use software-defined critical sections.  
The svd2rust documentation states that the take method used to get an instance of the device peripherals needs a critical-section implementation provided.  


The take function looks something like this :  
```rust
undone
```

### Abstraction aims
1. Reduce the amount of unsafe blocks. Limit them to be as close to the registers as much as possible. Reduce them in as much as you can
2. Abstract the true nature of the registers : Read-only, Write-only, Modify capabilities
3. Hide the register structures such that NOT Any piece of code anywhere in your program could access the hardware through these register structures
4. Ensure Volatile reads and writes.  
5. Handle concurrency gracefully using critical sections

- take by singletons
- repr peripheral with register-block - Name each regsiter
- register is a cell with volatile capabilities
- Reading requires a Read-proxy that can only access readable register sections
- You can access a bit OR bits OR enumerated value
- bit and Bits can be read or written using masks that have been enumerated.
- For unsafe arts, you can read or write to register sections with non-enumerated masks or bits. BUT they must be of the required bit-width.  
- The Write method takes in closures and returns a writer proxy. This enables chaining of writes in one command. (kinda like the builder pattern)
- The write method can write bit, bit and enumerated value.  
- modify method performs a single read-modify-write operation that involves one read (LDR) to the register using a read proxy, modifying the value and then a single write (STR) of the modified value to the register using a write proxy. 
- The reset_function...
  - resets the register under question
  - returns the reset value for you to inspect if you need to
  - 


Generally, as will be seen going forward, PAC code takes the following form:
```bash
[Peripheral Handle].[Peripheral Register Name].[Operation]
```  

PACs provide type-safe access to peripheral registers through API that allows manipulation of individual bits.


### Enumerated values  


If your SVD uses the <enumeratedValues> feature, then the API will be extended to provide even more type safety.  

In the context of SVD2Rust, the <enumeratedValues> feature in an SVD (System View Description) file is used to define named enumerations for specific fields within a register. These named enumerations provide additional type safety and clarity when working with the generated Rust API.  

To determine if your SVD file utilizes the <enumeratedValues> feature, you'll need to inspect the SVD file itself. This typically involves opening the SVD file in a text editor and searching for <enumeratedValues> tags within <registers> or <peripherals> sections.

Here's a simplified example of what such a section might look like in an SVD file:  

```xml
<registers>
    <register>
        <name>CTRL</name>
        <fields>
            <field>
                <name>MODE</name>
                <description>Operating mode</description>
                <bitRange>0-1</bitRange>
                <enumeratedValues>
                    <enumeratedValue>
                        <name>MODE1</name>
                        <description>Mode 1</description>
                        <value>0b00</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>MODE2</name>
                        <description>Mode 2</description>
                        <value>0b01</value>
                    </enumeratedValue>
                    <!-- More enumerated values here -->
                </enumeratedValues>
            </field>
            <!-- More fields here -->
        </fields>
    </register>
    <!-- More registers here -->
</registers>

```  


In this example, the <enumeratedValues> section defines named values (MODE1, MODE2, etc.) for the MODE field within the CTRL register. If your SVD file contains such sections, then the <enumeratedValues> feature is enabled, and you'll benefit from the additional type safety mentioned in the SVD2Rust documentation.  

The PAC library produced might contain code that looks like this :  
```rust
// Generated by svd2rust

// Module for peripherals
pub mod peripherals {
    // Register block for peripheral
    pub mod ctrl {
        // Register for the peripheral
        pub struct RegisterBlock {
            // Control register
            pub ctrl: RW<u32>,
        }

        // Register traits
        pub mod ctrl {
            // Read-write register
            pub struct RW<u32>;
        }

        impl RegisterBlock {
            // Method to create new instance
            pub fn new() -> RegisterBlock {
                RegisterBlock {
                    ctrl: RW::<u32>::new(0x4000_0000),
                }
            }
        }

        // Field enums
        pub mod ctrl {
            // Enum for field MODE
            pub enum MODE {
                MODE1 = 0b00,
                MODE2 = 0b01,
                // More variants for other enumerated values
            }
        }
    }
}
```  
MODE is an enum generated for the MODE field within the ctrl register. The variants of this enum (MODE1, MODE2, etc.) correspond to the enumerated values defined in the SVD file.  


In your driver code, you might use the enums like so ....

```rust
// Import the generated code
use peripherals::ctrl::{MODE, RegisterBlock};

fn main() {
    // Create a new instance of the register block
    let mut regs = RegisterBlock::new();

    // Set the MODE field to MODE2
    regs.ctrl.modify(|_, w| w.mode().variant(MODE::MODE2));

    // Read the MODE field
    let mode = regs.ctrl.read().mode().variant();

    // Match on the mode to perform actions based on its value
    match mode {
        MODE::MODE1 => println!("MODE1 selected"),
        MODE::MODE2 => println!("MODE2 selected"),
        _ => println!("Unknown mode"),
    }
}
```  


### Interrupts  

In SVD (System View Description) files, device interrupts are typically described within the <interrupt> section. This section provides information about the interrupts supported by the device, including their names, numbers, descriptions, and any associated priorities or properties.  

Here's a simplified example of how interrupts might be described in an SVD file:  
```xml
<device>
    <!-- Other device information -->

    <!-- Interrupts section -->
    <interrupts>
        <interrupt>
            <name>IRQ0</name>
            <description>Interrupt 0</description>
            <value>0</value>
        </interrupt>
        <interrupt>
            <name>IRQ1</name>
            <description>Interrupt 1</description>
            <value>1</value>
        </interrupt>
        <!-- More interrupts here -->
    </interrupts>

    <!-- Other device information -->
</device>

```


And from the above svd, the svd2rust program might generate a PAC module that looks something like this :  

```rust
// Generated by svd2rust

// Module for interrupts
pub mod interrupts {
    // Enum for device interrupts
    pub enum Interrupt {
        IRQ0,
        IRQ1,
        // More interrupts here
    }
}
```  

You can then use this enum in your Rust code to handle interrupts in a type-safe manner. For example:  
```rust
// Import the generated code
use interrupts::Interrupt;

fn main() {
    // Handle an interrupt
    let interrupt = Interrupt::IRQ0;

    match interrupt {
        Interrupt::IRQ0 => println!("Handling IRQ0"),
        Interrupt::IRQ1 => println!("Handling IRQ1"),
        _ => println!("Unknown interrupt"),
    }
}
```  

This interupt enums can be used with the microcontroller crates to enable/disable interrupts. For example, 
```rust
use cortex_m::peripheral::Peripherals;
use stm32f30x::Interrupt;

let p = Peripherals::take().unwrap();
let mut nvic = p.NVIC;

nvic.enable(Interrupt::TIM2);
nvic.enable(Interrupt::TIM3);
```


### The RT feature

The RT feature enables the inclusion of the `runtime-dependency` during compilations. The runtime dependencie may be crates like `cortex-m`, `riscv-rt`.  

The svd files contain additional info on interrupts that the board supports. So the svd2rust generates PAC code that adds this extra info onto the default vector-table that the rt-crate produced. It also provides a macro and an attribute to help you mark new interrupt handlers  

#### Why does the PAC crate depend on a runtime crate?  
If the rt Cargo feature of the svd2rust generated crate is enabled, the crate will populate the part of the vector table that contains the interrupt vectors and provide an interrupt! macro (non Cortex-M/MSP430 targets) or interrupt attribute (Cortex-M or MSP430) that can be used to register interrupt handlers.  



[espressif_svd_file_samples]: https://github.com/espressif/svd/tree/main/svd  
[esp32c3_svd_file]: https://github.com/espressif/svd/tree/main/svd