1. Consume the pointer pointing to a peripheral. Make sure ONLY one &mut T points to that peripheral. No other mutable reference should be used to reference that peripheral.  

```rust
/// Systick is a peripheral based on the memory map below  
/// | Offset | Name        | Description                 | Width  |
/// |--------|-------------|-----------------------------|--------|
/// | 0x00   | SYST_CSR    | Control and Status Register | 32 bits|
/// | 0x04   | SYST_RVR    | Reload Value Register       | 32 bits|
/// | 0x08   | SYST_CVR    | Current Value Register      | 32 bits|
/// | 0x0C   | SYST_CALIB  | Calibration Value Register  | 32 bits|
/// 
/// 
/// The base register of Systick is at address 0xE000_E010 
#[repr(C)]
struct SysTick{
    csr: u32, 
    rvr: u32,
    cvr: u32,
    calib: u32
}


fn main(){

    // Bad Example : the pointer to peripheral is not deleted
    {
        let sys_tick_instance_ptr = 0x200000 as *mut SysTick;
        let sys_tick_ref = unsafe {  sys_tick_instance_ptr as &mut SysTick  };
        *sys_tick_instance_ptr = 67; // we want to avoid untracked mutations like this line.  
                                     // It is better to stick to using references over pointers.  
                                     // Avoiding pointers has the downside that offset-calculations will be unavailable... But...
                                     // But you can solve that by creating better struct-abstractions over the offset-memory-region
    }
    // end of bad example 


    // Good example  : Raw pointer is immediately deleated
    {
        let sys_tick_instance_ptr = 0x200000 as *mut SysTick;
        let sys_tick_ref = unsafe {sys_tick_instance_ptr.as_mut()}; // as_mut() consumes the pointer as a value
        *sys_tick_instance_ptr = 67; // this will throw a compilation error. You are restricted to just using the sys_tick_ref.
    }
    // end of good example 
}

```


2. Make sure the data behind pointers are Non-Copy. If they are Copy-able, make sure you NEVER implicitly try to take ownership. This is better explained with the code below.  

```rust
    // Ownership primer loading....

    // this is a custom i32 that does not implement Copy trait. It will become relevant in a few lines to come
    struct NoCopyi32{
        data: i32
    }

    // every time you run a `let` statement, a new address in the stack must be instatiated. 
    // eg : 2 different addresses will be displayed in the 2 printlns! below.  
    let x = 10;  
    println!("address of x: {}", ptr::addr_of_mut!());
    let x = 20; 
    println!("new address of x: {}", ptr::addr_of_mut!());  

    // Let's look at another situation where both x and y hold values that implement the Copy trait
    let x = 30; // new stack address gets named `x`, 30 gets stored under that address
    let y = x;  // new stack adddress gets named `y`, 
                // rust-runtime copies the value in `x` and pastes it in `y`.
                // Rust does not Invalidate `x` because `x` and `y` are holding values without Copy trait
    println!("{}", x) // compiler does not complain about this line.
    println!("address of y: {} is different from address of x: {}. Both x & y are valid", ptr::addr_of!(y), ptr::addr_of!(x)); // NOTE this. 
    // the above line will be called `LINE_A`    

    // Let's look at another situation where both x and y hold values that DON'T implement the Copy trait
    let x = NoCopyi32{data: 40}; // new stack address gets named `x`, 40 gets stored under that address
    let x_addr = ptr::addr_of!(x); 

    let y = x;  // new stack adddress gets named `y`, 
                // rust-runtime copies the value in `x` and pastes it in `y`.
                // Rust INVALIDATES `x` because `x` and `y` are holding values with Copy trait
    print!("{} \n", x) // compiler COMPLAINS about this line.  

    println!("address of y: {} is different from address of x: {}. But x is no-longer valid", ptr::addr_of!(y), x_addr); // NOTE this. 
    // the above line will be called `LINE_B`                                                                                         


    // NOW to pointers!!!
    // Peripherals have definite addresses. To manipulate a peripheral, you must manipulate a specific address ONLY.  
    // 

    // Let us look at  situation where Copyable values make us deviate from our goal of ONLY affecting a SPECIFIC address.
    let reg_address = 0x20000; 
    let reg_ptr_1 = reg_address as *mut i32; // i32 implements Copy trait

    let mut reg_value = unsafe { *reg_ptr_1 } // This line tries to take ownership of value behind pointer
        // the let statement creates a new address (eg 0x80000) in the stack and calls that address `reg_value`
        // the rust-runtime copies the value contained in the address `0x20000` and pastes it in the address of `reg_value`
        // Rust does not invalidate reg_ptr_1
    
    // so when you run the line below, you are not modifying `0x20000`, you are modifying the new address(0x80000).  
    // in-short, because of the implicit copy, you are no longer modifying the peripheral address
    reg_value = 10; 
    println!("address of reg_value: {} is different from address of peripheral: {}.", ptr::addr_of!(reg_value), );



    // Possible solutions:  
        // 1. avoid using Copyable values behind pointers AND use references to modify register value(Best solution IMO)
        // 2. If you can't help but use Copyable values eg usize, then never try to take ownership and instead use only one mutable reference to access the registers


```





3. 