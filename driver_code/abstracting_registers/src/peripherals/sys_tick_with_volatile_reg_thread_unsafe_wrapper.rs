use core::ptr;
use volatile_register::{RW, RO, WO};


/// Systick is a peripheral based on the memory map below  
/// | Offset | Name        | Description                 | Width  |        |
/// |--------|-------------|-----------------------------|--------|--------|
/// | 0x00   | SYST_CSR    | Control and Status Register | 32 bits|
/// | 0x04   | SYST_RVR    | Reload Value Register       | 32 bits|
/// | 0x08   | SYST_CVR    | Current Value Register      | 32 bits|
/// | 0x0C   | SYST_CALIB  | Calibration Value Register  | 32 bits|
/// 
/// 
/// The base register of Systick is at address 0xE000_E010 
#[repr(C)]
pub struct SysTick{
    csr: RW<u32>, // Control and Status Register 
    rvr: RW<u32>, // Reload Value Register
    cvr: RW<u32>, // Current Value Register
    calib: RW<u32> // Calibration Value Register
}

impl SysTick{

    // instantiating a SysTick
    pub fn new() -> &'static mut SysTick{
        let sys_tick_ptr = 0xE000_E010 as *mut SysTick;  
        let sys_tick = unsafe {  sys_tick_ptr.as_mut()  }.unwrap();
        return sys_tick;
    }

    // read the current time from the Current Value Register (CVR)
    pub fn get_cvr_value(&self) -> u32 {
        self.cvr.read()
    }

    // write data to the callibration value register
    pub fn set_calib_value(&mut self, value: u32){
        unsafe{self.calib.write(value);}
    }
    


}

