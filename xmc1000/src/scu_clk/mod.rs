#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Control Register"]
    pub clkcr: CLKCR,
    #[doc = "0x04 - Power Save Control Register"]
    pub pwrsvcr: PWRSVCR,
    #[doc = "0x08 - Peripheral 0 Clock Gating Status"]
    pub cgatstat0: CGATSTAT0,
    #[doc = "0x0c - Peripheral 0 Clock Gating Set"]
    pub cgatset0: CGATSET0,
    #[doc = "0x10 - Peripheral 0 Clock Gating Clear"]
    pub cgatclr0: CGATCLR0,
    #[doc = "0x14 - Oscillator Control and Status Register"]
    pub osccsr: OSCCSR,
}
#[doc = "Clock Control Register"]
pub struct CLKCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Control Register"]
pub mod clkcr;
#[doc = "Power Save Control Register"]
pub struct PWRSVCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Save Control Register"]
pub mod pwrsvcr;
#[doc = "Peripheral 0 Clock Gating Status"]
pub struct CGATSTAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral 0 Clock Gating Status"]
pub mod cgatstat0;
#[doc = "Peripheral 0 Clock Gating Set"]
pub struct CGATSET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral 0 Clock Gating Set"]
pub mod cgatset0;
#[doc = "Peripheral 0 Clock Gating Clear"]
pub struct CGATCLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral 0 Clock Gating Clear"]
pub mod cgatclr0;
#[doc = "Oscillator Control and Status Register"]
pub struct OSCCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oscillator Control and Status Register"]
pub mod osccsr;
