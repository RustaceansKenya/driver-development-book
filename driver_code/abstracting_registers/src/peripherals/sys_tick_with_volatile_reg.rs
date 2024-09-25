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
struct SysTickMap{
    csr: RW<u32>, // Control and Status Register 
    rvr: RW<u32>, // Reload Value Register
    cvr: RW<u32>, // Current Value Register
    calib: RW<u32> // Calibration Value Register
}

impl SysTickMap{

    // instantiating a SysTickMap
    fn new(base_address: usize) -> &'static mut SysTickMap{
        let sys_tick_ptr = base_address as *mut SysTickMap;  
        let sys_tick = unsafe {  sys_tick_ptr.as_mut()  }.unwrap();
        return sys_tick;
    }

    // read the current time from the Current Value Register (CVR)
    fn get_cvr_value(&self) -> u32 {
        self.cvr.read()
    }

    // write data to the callibration value register
    fn set_calib_value(&mut self, value: u32){
        unsafe{self.calib.write(value);}
    }
    


}


pub struct SysTick{
    sys_tick_map: &'static mut SysTickMap
}

impl SysTick{
    pub fn new(base_address: usize) -> SysTick{
        let sys_tick_map_ref = SysTickMap::new(0xE000_E010);
        SysTick { sys_tick_map: sys_tick_map_ref }
    }   


    // read the current time from the Current Value Register (CVR)
    fn get_cvr_value(&self) -> u32 {
        self.sys_tick_map.get_cvr_value()
    }

    // write data to the callibration value register
    fn set_calib_value(&mut self, value: u32){
        self.sys_tick_map.set_calib_value(value)
    }
    


}

