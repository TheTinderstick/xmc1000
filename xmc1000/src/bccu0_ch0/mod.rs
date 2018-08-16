#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Intensit0 Shadow"]
    pub ints: INTS,
    #[doc = "0x04 - Channel Intensit0"]
    pub int: INT,
    #[doc = "0x08 - Channel Configuration"]
    pub chconfig: CHCONFIG,
    #[doc = "0x0c - Packer Compare"]
    pub pkcmp: PKCMP,
    #[doc = "0x10 - Packer Counter"]
    pub pkcntr: PKCNTR,
}
#[doc = "Channel Intensit0 Shadow"]
pub struct INTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Intensit0 Shadow"]
pub mod ints;
#[doc = "Channel Intensit0"]
pub struct INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Intensit0"]
pub mod int;
#[doc = "Channel Configuration"]
pub struct CHCONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Configuration"]
pub mod chconfig;
#[doc = "Packer Compare"]
pub struct PKCMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Packer Compare"]
pub mod pkcmp;
#[doc = "Packer Counter"]
pub struct PKCNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Packer Counter"]
pub mod pkcntr;
