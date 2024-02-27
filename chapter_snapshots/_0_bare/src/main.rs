#![no_std]  // DIsable no-std and insted use lib-core only
#![no_main] // Disable the default entry-point chain that usually treats 'main' as a third entry point

use core::panic::PanicInfo;

// Definition of a custom panic handler
#[panic_handler]
fn my_custom_function( panic_info: &PanicInfo)-> !{
    // println!("message : {}", panic_info.message())
    // println!("location : {}", panic_info.location())
    loop {
        // create an infinite loop to make this Function to be divergent
    }
}


// Our entry point into Rust after we have finished booting
#[no_mangle]
pub extern "C" fn rust_entry() {
  // some magic
}