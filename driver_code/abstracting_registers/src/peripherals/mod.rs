mod sys_tick_with_volatile_reg;

use core::mem;
use sys_tick_with_volatile_reg::SysTick;


pub struct Peripherals{
    system_timer: Option<SysTick>,
}  


impl Peripherals{
    pub fn new_uninitialized() -> Peripherals{
        Peripherals { system_timer: None }
    }       

    pub fn initialize(&mut self){
        let system_timer_instance = SysTick::new(0x200000);
        let ret_value = mem::replace(&mut self.system_timer, Some(system_timer_instance));
        if 
    } 

    pub fn get_system_timer() -> SysTick{
        unimplemented!()
    }
}