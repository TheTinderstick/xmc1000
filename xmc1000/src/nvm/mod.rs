#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - NVM Status Register"]
    pub nvmstatus: NVMSTATUS,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - NVM Programming Control Register"]
    pub nvmprog: NVMPROG,
    _reserved1: [u8; 2usize],
    #[doc = "0x08 - NVM Configuration Register"]
    pub nvmconf: NVMCONF,
    _reserved2: [u8; 62usize],
    #[doc = "0x48 - Configuration 1 Register"]
    pub config1: CONFIG1,
}
#[doc = "NVM Status Register"]
pub struct NVMSTATUS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "NVM Status Register"]
pub mod nvmstatus;
#[doc = "NVM Programming Control Register"]
pub struct NVMPROG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "NVM Programming Control Register"]
pub mod nvmprog;
#[doc = "NVM Configuration Register"]
pub struct NVMCONF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "NVM Configuration Register"]
pub mod nvmconf;
#[doc = "Configuration 1 Register"]
pub struct CONFIG1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Configuration 1 Register"]
pub mod config1;
