#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Dimming Level Shadow"]
    pub dls: DLS,
    #[doc = "0x04 - Dimming Level"]
    pub dl: DL,
    #[doc = "0x08 - Dimming Transition Time"]
    pub dtt: DTT,
}
#[doc = "Dimming Level Shadow"]
pub struct DLS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dimming Level Shadow"]
pub mod dls;
#[doc = "Dimming Level"]
pub struct DL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dimming Level"]
pub mod dl;
#[doc = "Dimming Transition Time"]
pub struct DTT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dimming Transition Time"]
pub mod dtt;
