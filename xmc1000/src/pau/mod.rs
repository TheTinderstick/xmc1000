#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 64usize],
    #[doc = "0x40 - Peripheral Availability Register 0"]
    pub avail0: AVAIL0,
    #[doc = "0x44 - Peripheral Availability Register 1"]
    pub avail1: AVAIL1,
    #[doc = "0x48 - Peripheral Availability Register 2"]
    pub avail2: AVAIL2,
    _reserved1: [u8; 52usize],
    #[doc = "0x80 - Peripheral Privilege Access Register 0"]
    pub privdis0: PRIVDIS0,
    #[doc = "0x84 - Peripheral Privilege Access Register 1"]
    pub privdis1: PRIVDIS1,
    #[doc = "0x88 - Peripheral Privilege Access Register 2"]
    pub privdis2: PRIVDIS2,
    _reserved2: [u8; 884usize],
    #[doc = "0x400 - ROM Size Register"]
    pub romsize: ROMSIZE,
    #[doc = "0x404 - Flash Size Register"]
    pub flsize: FLSIZE,
    _reserved3: [u8; 8usize],
    #[doc = "0x410 - RAM0 Size Register"]
    pub ram0size: RAM0SIZE,
}
#[doc = "Peripheral Availability Register 0"]
pub struct AVAIL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Availability Register 0"]
pub mod avail0;
#[doc = "Peripheral Availability Register 1"]
pub struct AVAIL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Availability Register 1"]
pub mod avail1;
#[doc = "Peripheral Availability Register 2"]
pub struct AVAIL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Availability Register 2"]
pub mod avail2;
#[doc = "Peripheral Privilege Access Register 0"]
pub struct PRIVDIS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Privilege Access Register 0"]
pub mod privdis0;
#[doc = "Peripheral Privilege Access Register 1"]
pub struct PRIVDIS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Privilege Access Register 1"]
pub mod privdis1;
#[doc = "Peripheral Privilege Access Register 2"]
pub struct PRIVDIS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Privilege Access Register 2"]
pub mod privdis2;
#[doc = "ROM Size Register"]
pub struct ROMSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ROM Size Register"]
pub mod romsize;
#[doc = "Flash Size Register"]
pub struct FLSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Size Register"]
pub mod flsize;
#[doc = "RAM0 Size Register"]
pub struct RAM0SIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RAM0 Size Register"]
pub mod ram0size;
