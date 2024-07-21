#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn my_panic_handler (panic_info_instance: &PanicInfo) -> !{
    loop {
        // endless loop
    }
}



fn main() {
    // println!("Hello, world!");
}
