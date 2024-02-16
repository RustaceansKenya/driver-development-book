#![no_std]
#![no_main]


use core::panic::PanicInfo;

#[panic_handler]
fn default_panic_handler(_info: &PanicInfo) -> ! {

    // some magic
    loop {
        // endless loop to make the function divergent
    }
}

