#![no_std]
#![no_main]


use core::panic::PanicInfo;
use riscv_rt::entry;
// use esp_println::println;

#[panic_handler]
fn default_panic_handler(_info: &PanicInfo) -> ! {

    // some magic
    loop {
        // endless loop to make the function divergent
    }
}


#[entry]
fn main() -> ! {
    // let peripherals = Peripherals::take();
    // let system = peripherals.SYSTEM.split();

    // let clocks = ClockControl::max(system.clock_control).freeze();
    // let mut delay = Delay::new(&clocks);

    // println!("Hello world!");
    // loop {
    //     println!("Loop...");
    //     delay.delay_ms(500u32);
    // }

    loop {
        
    }
}

