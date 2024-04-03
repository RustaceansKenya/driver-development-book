#![no_std]
#![no_main]  // here is the new line. We have added the no_main macro attribute

use core::panic::PanicInfo;

#[panic_handler]
fn default_panic_handler(_info: &PanicInfo) -> !{
    loop { /* magic goes here */ }
}
