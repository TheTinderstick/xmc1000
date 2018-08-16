#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Debug System ROM ID Register"]
    pub dbgromid: DBGROMID,
    #[doc = "0x04 - Chip ID Register"]
    pub idchip: IDCHIP,
    #[doc = "0x08 - SCU Module ID Register"]
    pub id: ID,
    _reserved0: [u8; 8usize],
    #[doc = "0x14 - SSW Register 0"]
    pub ssw0: SSW0,
    _reserved1: [u8; 12usize],
    #[doc = "0x24 - Password Register"]
    pub passwd: PASSWD,
    _reserved2: [u8; 8usize],
    #[doc = "0x30 - CCU Control Register"]
    pub ccucon: CCUCON,
    _reserved3: [u8; 20usize],
    #[doc = "0x48 - Mirror Update Status Register"]
    pub mirrsts: MIRRSTS,
    _reserved4: [u8; 8usize],
    #[doc = "0x54 - Parity Memory Test Select Register"]
    pub pmtsr: PMTSR,
}
#[doc = "Debug System ROM ID Register"]
pub struct DBGROMID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug System ROM ID Register"]
pub mod dbgromid;
#[doc = "Chip ID Register"]
pub struct IDCHIP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip ID Register"]
pub mod idchip;
#[doc = "SCU Module ID Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCU Module ID Register"]
pub mod id;
#[doc = "SSW Register 0"]
pub struct SSW0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSW Register 0"]
pub mod ssw0;
#[doc = "Password Register"]
pub struct PASSWD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Password Register"]
pub mod passwd;
#[doc = "CCU Control Register"]
pub struct CCUCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCU Control Register"]
pub mod ccucon;
#[doc = "Mirror Update Status Register"]
pub struct MIRRSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mirror Update Status Register"]
pub mod mirrsts;
#[doc = "Parity Memory Test Select Register"]
pub struct PMTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Memory Test Select Register"]
pub mod pmtsr;
