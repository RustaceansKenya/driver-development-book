use core::ptr;

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

// instantiating a SysTick
fn instantiate_sys_tick() -> &'static mut SysTick{
    let sys_tick_ptr = 0xE000_E010 as *mut SysTick;  
    let sys_tick = unsafe {  sys_tick_ptr.as_mut()  };
    return sys_tick;
}

#[derive(Debug)]

struct Point<X,Y>(X, Y);


fn main() {
    let mut value = 10;

    // Create a mutable reference
    let reference: &mut i32 = &mut value;

    // Cast the reference to a raw pointer
    let raw_pointer: *mut i32 = reference as *mut i32;

    // Use the reference again (which is now invalid after the cast)
    *reference = 50; // Undefined behavior! The reference and pointer accesses are interleaved.

    // Access through raw pointer
    unsafe {
        *raw_pointer = 42;
        println!("Value through raw pointer: {}", *raw_pointer);
    }

    println!("Value after raw pointer access: {}", value);
}

fn compute(input: &u32, output: &mut u32) {
    // keep `*input` in a register
    let cached_input = *input;
    if cached_input > 10 {
        // If the original, > 10 would imply:
        //
        // *output = 1
        // *output *= 2
        //
        // which we can just simplify into:
        *output = 2;
    } else if cached_input > 5 {
        *output *= 2;
    }
}       