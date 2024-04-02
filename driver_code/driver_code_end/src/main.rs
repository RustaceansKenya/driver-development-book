#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[panic_handler]
fn default_panic_handler(_info: &PanicInfo) -> !{
    loop {    }
}


