#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 36usize],
    #[doc = "0x24 - Temperature Sensor Control Register"]
    pub anatsectrl: ANATSECTRL,
    _reserved1: [u8; 10usize],
    #[doc = "0x30 - Temperature Sensor High Temperature Interrupt Register"]
    pub anatseih: ANATSEIH,
    _reserved2: [u8; 2usize],
    #[doc = "0x34 - Temperature Sensor Low Temperature Interrupt Register"]
    pub anatseil: ANATSEIL,
    _reserved3: [u8; 10usize],
    #[doc = "0x40 - Temperature Sensor Counter2 Monitor Register"]
    pub anatsemon: ANATSEMON,
    _reserved4: [u8; 14usize],
    #[doc = "0x50 - Voltage Detector Control Register"]
    pub anavdel: ANAVDEL,
    _reserved5: [u8; 26usize],
    #[doc = "0x6c - DCO1 Offset Register"]
    pub anaoffset: ANAOFFSET,
}
#[doc = "Temperature Sensor Control Register"]
pub struct ANATSECTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Temperature Sensor Control Register"]
pub mod anatsectrl;
#[doc = "Temperature Sensor High Temperature Interrupt Register"]
pub struct ANATSEIH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Temperature Sensor High Temperature Interrupt Register"]
pub mod anatseih;
#[doc = "Temperature Sensor Low Temperature Interrupt Register"]
pub struct ANATSEIL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Temperature Sensor Low Temperature Interrupt Register"]
pub mod anatseil;
#[doc = "Temperature Sensor Counter2 Monitor Register"]
pub struct ANATSEMON {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Temperature Sensor Counter2 Monitor Register"]
pub mod anatsemon;
#[doc = "Voltage Detector Control Register"]
pub struct ANAVDEL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Voltage Detector Control Register"]
pub mod anavdel;
#[doc = "DCO1 Offset Register"]
pub struct ANAOFFSET {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DCO1 Offset Register"]
pub mod anaoffset;
