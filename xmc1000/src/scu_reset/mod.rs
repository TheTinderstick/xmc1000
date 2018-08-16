#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RCU Reset Status"]
    pub rststat: RSTSTAT,
    #[doc = "0x04 - RCU Reset Set Register"]
    pub rstset: RSTSET,
    #[doc = "0x08 - RCU Reset Clear Register"]
    pub rstclr: RSTCLR,
    #[doc = "0x0c - RCU Reset Control Register"]
    pub rstcon: RSTCON,
}
#[doc = "RCU Reset Status"]
pub struct RSTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCU Reset Status"]
pub mod rststat;
#[doc = "RCU Reset Set Register"]
pub struct RSTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCU Reset Set Register"]
pub mod rstset;
#[doc = "RCU Reset Clear Register"]
pub struct RSTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCU Reset Clear Register"]
pub mod rstclr;
#[doc = "RCU Reset Control Register"]
pub struct RSTCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCU Reset Control Register"]
pub mod rstcon;
