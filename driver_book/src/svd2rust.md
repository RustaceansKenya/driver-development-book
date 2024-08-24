# svd2rust

Once you read the datasheet, and understand the memory mappings, pin-layout and whatever else you wanted to get straight, you begin to safely abstract the board.  


### SVD files
An svd file is a file that describes the peripherals of a board using xml. So you could say that an svd file is a board abstracted as an xml template.  
SVD is the abbreviation for : System View Description.  


The svd file outlines :
 - The boards metadata eg boardname, board version, feature description, vendor name
 - Major component info : eg CPU_capabilities, Endianness, address_width, added cpu_extensions... 
 - all list of all the peripherals
   - the registers of each peripheral
   - the functions of each register
   - the memory address of each register
   - the read/write access of each register



You can find sample svd files [here][espressif_svd_file_samples], they are from the espressif organization.  
Here is the esp32C3 svd file that we will be using : [ESP32_C3 svd file][esp32c3_svd_file]  


Here is a snippet of a sample svd file : 
```xml
<?xml version="1.0" encoding="UTF-8"?>
<device schemaVersion="1.1" xmlns:xs="http://www.w3.org/2001/XMLSchema-instance" xs:noNamespaceSchemaLocation="CMSIS-SVD_Schema_1_1.xsd">
  <vendor>ESPRESSIF SYSTEMS (SHANGHAI) CO., LTD.</vendor>
  <vendorID>ESPRESSIF</vendorID>
  <name>ESP32-C3</name>
  <series>ESP32 C-Series</series>
  <version>17</version>
  <description>32-bit RISC-V MCU &amp; 2.4 GHz Wi-Fi &amp; Bluetooth 5 (LE)</description>

  <!-- snip snip snipped some lines -->

  <cpu>
    <name>RV32IMC</name>
    <revision>r0p0</revision>
    <endian>little</endian>
    <mpuPresent>false</mpuPresent>
    <fpuPresent>false</fpuPresent>
    <nvicPrioBits>0</nvicPrioBits>
    <vendorSystickConfig>false</vendorSystickConfig>
  </cpu>

  <!-- snip snip snipped some lines -->

   <peripherals>
   
   <!-- here is 1/32 peripherals -->
    <peripheral>
        <name>UART0</name>
      <description>UART (Universal Asynchronous Receiver-Transmitter) Controller 0</description>
      <groupName>UART</groupName>
      <baseAddress>0x60000000</baseAddress>
      <addressBlock>
        <offset>0x0</offset>
        <size>0x84</size>
        <usage>registers</usage>
      </addressBlock>
      <interrupt>
        <name>UART0</name>
        <value>21</value>
      </interrupt>
      <registers>
        <register>
          <name>FIFO</name>
          <description>FIFO data register</description>
          <addressOffset>0x0</addressOffset>
          <size>0x20</size>
          <fields>
            <field>
              <name>RXFIFO_RD_BYTE</name>
              <description>UART 0 accesses FIFO via this register.</description>
              <bitOffset>0</bitOffset>
              <bitWidth>8</bitWidth>
              <access>read-write</access>
            </field>
          </fields>
        </register>

        <!-- more registers -->
        <register>
        </register>
        </register>
        <!-- more registers -->
    <peripheral>

    <!-- snipped out the other 31 peripherals -->
   <peripherals>
```


### svd2Rust

This is a tool that takes in svd files and outputs Rust code that reflects the contents of the svd file.  

### Why use svd2rust instead of doing the abstraction manually?

Before we discuss whether you should do it manually or not. Let's settle out some facts first.  
A full-fledged board has many components. The datasheet reference is like >700 pages. These components are dependent on each other.  
You get some form of info overload. How can you create complete abstractions if you do not fully understand the board and how they are interdependent on each other? Enter headaches and suicidal thoughts.  

If you look at the esp32c3.svd file, you realize it is >35000 lines. But at-least the svd file provides a complete abstraction from the >700 page datasheet.  

#### When to do it manually
1. When you fully understand all the details about a peripheral
2. When you also fuly understand all the direct components that the target peripheral depends on.  
3. When you can comfortably abstract the peripheral and its dependents, in a safe way: critical sections, atomics and all that vodoo when accessing registers.  
4. When you do not need to abstract the whole board.  

#### When to do it automatically
1. When you dont mind abstracting all the peripherals
2. When you want a library to automatically implement the access-safety methods of accessing registers. You don't have to implement atomic vodoo on your own.
3. When you want to use a standard way of abstracting the board. Your whole team uses the same template. Everyone speaks the same language, everyone becomes happy.



### svd2rust

To understand svd2rust, let's :
1. read its docs
2. experiment with it a little
3. Do our abstractions manually without depending on svd2rust
4. Go back to svd2rust while fully appreciating all the manual work it does for us 



From the docs :  

"svd2rust supports Cortex-M, MSP430, RISCV and Xtensa LX6 microcontrollers." this means that ...

Taking Peripherals needs to be atomic. Some boards do not support atomics, so the best way is to use software-defined critical sections.  
The svd2rust documentation states that the take method used to get an instance of the device peripherals needs a critical-section implementation provided.  


The take function looks something like this :  
```rust

```

### Abstraction aims
1. Reduce the amount of unsafe blocks. Limit them to be as close to the registers as much as possible. Reduce them in as much as you can
2. Abstract the true nature of the registers : Read-only, Write-only, Modify capabilities
3. Hide the register structures such that NOT Any piece of code anywhere in your program could access the hardware through these register structures
4. Ensure Volatile reads and writes.  
5. Handle concurrency gracefully using critical sections

- take by singletonsrustc-link-search
- repr peripheral with register-block - Name each regsiter
- register is a cell with volatile capabilities
- Reading requires a Read-proxy that can only access readable register sections
- You can access a bit OR bits OR enumerated value
- bit and Bits can be read or written using masks that have been enumerated.
- For unsafe arts, you can read or write to register sections with non-enumerated masks or bits. BUT they must be of the required bit-width.  
- The Write method takes in closures and returns a writer proxy. This enables chaining of writes in one command. (kinda like the builder pattern)
- The write method can write bit, bitrustc-link-searchs and enumerated value.  
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

The svd files contain additional info on interrupts that the board supports. So the svd2rust generates PAC code that adds this extra info onto the default vector-table that the rt-crate produced. It also provides a macro and an attibute to help you mark new interrupt handlers  

#### Why does the PAC crate depend on a runtime crate?  
If the rt Cargo feature of the svd2rust generated crate is enabled, the crate will populate the part of the vector table that contains the interrupt vectors and provide an interrupt! macro (non Cortex-M/MSP430 targets) or interrupt attribute (Cortex-M or MSP430) that can be used to register interrupt handlers.  



[espressif_svd_file_samples]: https://github.com/espressif/svd/tree/main/svd  
[esp32c3_svd_file]: https://github.com/espressif/svd/tree/main/svd

