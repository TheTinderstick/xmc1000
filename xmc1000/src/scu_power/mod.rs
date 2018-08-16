#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Voltage Detector Status Register"]
    pub vdesr: VDESR,
}
#[doc = "Voltage Detector Status Register"]
pub struct VDESR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Voltage Detector Status Register"]
pub mod vdesr;
