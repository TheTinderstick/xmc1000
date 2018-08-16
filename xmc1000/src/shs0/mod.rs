#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Module Identification Register"]
    pub id: ID,
    _reserved1: [u8; 52usize],
    #[doc = "0x40 - SHS Configuration Register"]
    pub shscfg: SHSCFG,
    #[doc = "0x44 - Stepper Configuration Register"]
    pub stepcfg: STEPCFG,
    _reserved2: [u8; 8usize],
    #[doc = "0x50 - Loop Control Register"]
    pub loop_: LOOP,
    _reserved3: [u8; 44usize],
    #[doc = "0x80 - Timing Configuration Register 0"]
    pub timcfg0: TIMCFG0,
    #[doc = "0x84 - Timing Configuration Register 1"]
    pub timcfg1: TIMCFG1,
    _reserved4: [u8; 52usize],
    #[doc = "0xbc - Calibration Control Register"]
    pub calctr: CALCTR,
    #[doc = "0xc0 - Gain Calibration Control Register 0"]
    pub calgc0: CALGC0,
    #[doc = "0xc4 - Gain Calibration Control Register 1"]
    pub calgc1: CALGC1,
    _reserved5: [u8; 24usize],
    #[doc = "0xe0 - Offset Calibration Control Register 0"]
    pub caloc0: CALOC0,
    #[doc = "0xe4 - Offset Calibration Control Register 1"]
    pub caloc1: CALOC1,
    _reserved6: [u8; 152usize],
    #[doc = "0x180 - Gain Control Register 00"]
    pub gnctr00: GNCTR00,
    _reserved7: [u8; 12usize],
    #[doc = "0x190 - Gain Control Register 10"]
    pub gnctr10: GNCTR10,
}
#[doc = "Module Identification Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "SHS Configuration Register"]
pub struct SHSCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SHS Configuration Register"]
pub mod shscfg;
#[doc = "Stepper Configuration Register"]
pub struct STEPCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stepper Configuration Register"]
pub mod stepcfg;
#[doc = "Timing Configuration Register 0"]
pub struct TIMCFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timing Configuration Register 0"]
pub mod timcfg0;
#[doc = "Timing Configuration Register 1"]
pub struct TIMCFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timing Configuration Register 1"]
pub mod timcfg1;
#[doc = "Calibration Control Register"]
pub struct CALCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration Control Register"]
pub mod calctr;
#[doc = "Gain Calibration Control Register 0"]
pub struct CALGC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Gain Calibration Control Register 0"]
pub mod calgc0;
#[doc = "Gain Calibration Control Register 1"]
pub struct CALGC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Gain Calibration Control Register 1"]
pub mod calgc1;
#[doc = "Offset Calibration Control Register 0"]
pub struct CALOC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Offset Calibration Control Register 0"]
pub mod caloc0;
#[doc = "Offset Calibration Control Register 1"]
pub struct CALOC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Offset Calibration Control Register 1"]
pub mod caloc1;
#[doc = "Gain Control Register 00"]
pub struct GNCTR00 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Gain Control Register 00"]
pub mod gnctr00;
#[doc = "Gain Control Register 10"]
pub struct GNCTR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Gain Control Register 10"]
pub mod gnctr10;
#[doc = "Loop Control Register"]
pub struct LOOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Loop Control Register"]
pub mod loop_;
