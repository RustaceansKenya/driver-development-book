#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[panic_handler]
fn default_panic_handler(_info: &PanicInfo) -> !{
    loop {    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // println!("Hello, world!");
    loop {
        let _x = 2 + 3;
    }
}



