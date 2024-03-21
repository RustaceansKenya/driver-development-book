- exception attribute with re-entrancy guarantees
- zero-cost abstractions?


### What should our rules be when dealing with peripherl registers?

How can we reliably interact with these peripherals?

- Always use volatile methods to read or write to peripheral memory, as it can change at any time
- In software, we should be able to share any number of read-only accesses to these peripherals
- If some software should have read-write access to a peripheral, it should hold the only reference to that peripheral

We cannot use global mutable variables because they expose our peripheral to every software. We just want one software to have the mutable reference to a single peripheral. Every other software should have read_references.  

There should ONLY BE ONE mutable instance of a peripheral.  

That is why we use the Singleton. 

### Pointer manipulations
Be afraid of implicit copies. Just use ONE value assignation and multiple references when dealing with pointers that point to ONE physical memory address.  
  
As you can see below, x and y have different addresses. 
```rust
    let x = 10;
    let y = x;

    println!("Address of x = {}", (&x as *const i32) as usize); // Address of x = 140735333354256
    println!("Address of y = {}", (&y as *const i32) as usize); // Address of y = 140735333354260
```  


ONE ASSIGNATION, the rest should be references.  
TWO ASIGNATIONS result in two different raw addresse.... 