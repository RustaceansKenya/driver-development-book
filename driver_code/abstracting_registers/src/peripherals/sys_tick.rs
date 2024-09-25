use core::ptr;


/// Systick is a peripheral based on the memory map below  
/// | Offset | Name        | Description                 | Width  |
/// |--------|-------------|-----------------------------|--------|
/// | 0x00   | SYST_CSR    | Control and Status Register | 32 bits|
/// | 0x04   | SYST_RVR    | Reload Value Register       | 32 bits|
/// | 0x08   | SYST_CVR    | Current Value Register      | 32 bits|
/// | 0x0C   | SYST_CALIB  | Calibration Value Register  | 32 bits|
/// 
/// 
/// The base register of Systick is at address 0xE000_E010 
#[repr(C)]
struct SysTick{
    csr: u32, // Control and Status Register 
    rvr: u32, // Reload Value Register
    cvr: u32, // Current Value Register
    calib: u32 // Calibration Value Register
}

impl SysTick{

    // instantiating a SysTick
    fn new() -> &'static mut SysTick{
        let sys_tick_ptr = 0xE000_E010 as *mut SysTick;  
        let sys_tick = unsafe {  sys_tick_ptr.as_mut()  }.unwrap();
        return sys_tick;
    }

    // read the current time from the Current Value Register (CVR)
    fn get_cvr_value(&self) -> u32 {
        unsafe { ptr::read_volatile(&(self.cvr)) }
    }

    // write data to the callibration value register
    fn set_calib_value(&mut self, value: u32){
        unsafe {    ptr::write_volatile(&mut self.calib, value);    }
    }



}


fn main() {
    
}
