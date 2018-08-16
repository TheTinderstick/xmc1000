#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Control"]
    pub globcon: GLOBCON,
    #[doc = "0x04 - Global Clock"]
    pub globclk: GLOBCLK,
    #[doc = "0x08 - Module Identification"]
    pub id: ID,
    #[doc = "0x0c - Channel Enable"]
    pub chen: CHEN,
    #[doc = "0x10 - Channel Output Control"]
    pub chocon: CHOCON,
    #[doc = "0x14 - Channel Trigger"]
    pub chtrig: CHTRIG,
    #[doc = "0x18 - Channel Shadow Transfer"]
    pub chstrcon: CHSTRCON,
    #[doc = "0x1c - Last Trigger Channel Output Level"]
    pub ltchol: LTCHOL,
    #[doc = "0x20 - Dimming Engine Enable"]
    pub deen: DEEN,
    #[doc = "0x24 - Dimming Shadow Transfer"]
    pub destrcon: DESTRCON,
    #[doc = "0x28 - Global Dimming Level"]
    pub globdim: GLOBDIM,
    #[doc = "0x2c - Event Interrupt Enable"]
    pub evier: EVIER,
    #[doc = "0x30 - Event Flag"]
    pub evfr: EVFR,
    #[doc = "0x34 - Event Flag Set"]
    pub evfsr: EVFSR,
    #[doc = "0x38 - Event Flag Clear"]
    pub evfcr: EVFCR,
}
#[doc = "Global Control"]
pub struct GLOBCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Control"]
pub mod globcon;
#[doc = "Global Clock"]
pub struct GLOBCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Clock"]
pub mod globclk;
#[doc = "Module Identification"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Identification"]
pub mod id;
#[doc = "Channel Enable"]
pub struct CHEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Enable"]
pub mod chen;
#[doc = "Channel Output Control"]
pub struct CHOCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Output Control"]
pub mod chocon;
#[doc = "Channel Trigger"]
pub struct CHTRIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Trigger"]
pub mod chtrig;
#[doc = "Channel Shadow Transfer"]
pub struct CHSTRCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Shadow Transfer"]
pub mod chstrcon;
#[doc = "Last Trigger Channel Output Level"]
pub struct LTCHOL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Last Trigger Channel Output Level"]
pub mod ltchol;
#[doc = "Dimming Engine Enable"]
pub struct DEEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dimming Engine Enable"]
pub mod deen;
#[doc = "Dimming Shadow Transfer"]
pub struct DESTRCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dimming Shadow Transfer"]
pub mod destrcon;
#[doc = "Global Dimming Level"]
pub struct GLOBDIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Dimming Level"]
pub mod globdim;
#[doc = "Event Interrupt Enable"]
pub struct EVIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Interrupt Enable"]
pub mod evier;
#[doc = "Event Flag"]
pub struct EVFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Flag"]
pub mod evfr;
#[doc = "Event Flag Set"]
pub struct EVFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Flag Set"]
pub mod evfsr;
#[doc = "Event Flag Clear"]
pub struct EVFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Flag Clear"]
pub mod evfcr;
