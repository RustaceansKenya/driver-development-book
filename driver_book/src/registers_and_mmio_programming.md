# Registers and MMIO programming

You know your hardware by reading the relevant parts of your datasheet.  
Your hardware is like a library, the datasheet is its documentation.  

## MMIO Programming  
But before we talk about the datasheets, let't talk about MMIO-programming.  
[demo, undone]  
MMIO involves using memory addresses as an interface to communicate with and control hardware devices.  

Memory-Mapped I/O refers to the technique of accessing hardware registers and controlling peripherals using memory addresses. Instead of specialized instructions, developers interact with hardware by reading from and writing to specific memory addresses, treating hardware like memory-mapped regions.  

Hardware Registers: Peripherals and hardware components are often controlled by registers, each associated with a specific functionality (e.g., configuration, data transmission, status).

So to control hardware, you read and write to the respective registers... or memory regions.  


## The Volatile key word

Quick detour :

Facts :
1. Your code does not always get executed procedurally. 
2. Some lines from your code get ignored or cut out by the compiler or CPU. 

 
The compiler optimizes the order of instructions, it even makes assumptions : 
eg 
```rust
  let register_1 = 1;
  // insert other instructions here
  let register_1 = 1;
```

can be optimized to 
```rust
  let register_1 = 1;
  // insert other instructions here
  //   let register_1 = 1; // gets truncated
```


The CPU also optimizes and changes the order of those instruction even further.  
If you add parallelism to the matter... it just makes it impossible to be sure that your instructions get executed in a specific order. 

These optimizations are bad if the changes to the register truly matter.  

### Enter the volatile key-word... the superhero  

The volatile keyword makes reads and writes to be atomic and un-reordered(if that'ts a word).   

It is the embodiement of :
"Hey, compiler and CPU, no optimizations should affect the order of my reads and writes. I cannot deal with surprises please"  
useul when the order and timing of reads and writes are critical, and the compiler should not make any assumptions about the potential side effects of these operations.  

example in rust : 
```rust
// Import necessary modules
use core::ptr;

fn main() {
    // Define a mutable pointer to a memory-mapped address
    let mut mmio_ptr = 0x4000_0000 as *mut u32;

    unsafe {
        // Read from a volatile memory-mapped address
        let value = ptr::read_volatile(mmio_ptr);
        println!("Read value: {}", value);

        // Write to a volatile memory-mapped address
        let new_value = value + 1;
        ptr::write_volatile(mmio_ptr, new_value);
        println!("Written new value: {}", new_value);
    }
}
```


Summary :  
1. You control hardware by reading and writing to its registers.   
2. All reads and writes to the registers have to be done using the volatile keyword.


Question : Why do you think Volatile reads and writes are not the default methods. why are unpredictable reads and writes the default methods?


