#![no_std]

use core::panic::PanicInfo;


#[panic_handler]
// you can name this function any name...it does not matter. eg the_coolest_name_in_the_world
// The function takes in a reference to the panic Info. 
// Kid, go read the docs in core::panic module. It's short & sweet. You will revisit it a couple of times though  
fn default_panic_handler(_info: &PanicInfo) -> !{
    loop {  
        // function does nothing for now, but this is where you write your magic //
        // This is where you typically call an exception handler, or call code that logs the error or panic messages before aborting the program
        // The function never returns, this is an endless loop... The panic_handler is a divergent function
      }
}


fn main(){
    // println!("Hello world!!");
}