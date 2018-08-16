#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Global Control Register"]
    pub glbcon: GLBCON,
    #[doc = "0x08 - Module Identification Register"]
    pub id: ID,
    #[doc = "0x0c - Event Interrupt Enable Register"]
    pub evier: EVIER,
    #[doc = "0x10 - Event Flag Register"]
    pub evfr: EVFR,
    #[doc = "0x14 - Event Flag Set Register"]
    pub evfsr: EVFSR,
    #[doc = "0x18 - Event Flag Clear Register"]
    pub evfcr: EVFCR,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - Dividend Register"]
    pub dvd: DVD,
    #[doc = "0x24 - Divisor Register"]
    pub dvs: DVS,
    #[doc = "0x28 - Quotient Register"]
    pub quot: QUOT,
    #[doc = "0x2c - Remainder Register"]
    pub rmd: RMD,
    #[doc = "0x30 - Divider Status Register"]
    pub divst: DIVST,
    #[doc = "0x34 - Divider Control Register"]
    pub divcon: DIVCON,
    _reserved2: [u8; 8usize],
    #[doc = "0x40 - CORDIC Status and Data Control Register"]
    pub statc: STATC,
    #[doc = "0x44 - CORDIC Control Register"]
    pub con: CON,
    #[doc = "0x48 - CORDIC X Data Register"]
    pub cordx: CORDX,
    #[doc = "0x4c - CORDIC Y Data Register"]
    pub cordy: CORDY,
    #[doc = "0x50 - CORDIC Z Data Register"]
    pub cordz: CORDZ,
    #[doc = "0x54 - CORDIC X Result Register"]
    pub corrx: CORRX,
    #[doc = "0x58 - CORDIC Y Result Register"]
    pub corry: CORRY,
    #[doc = "0x5c - CORDIC Z Result Register"]
    pub corrz: CORRZ,
}
#[doc = "Global Control Register"]
pub struct GLBCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Control Register"]
pub mod glbcon;
#[doc = "Module Identification Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "Event Interrupt Enable Register"]
pub struct EVIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Interrupt Enable Register"]
pub mod evier;
#[doc = "Event Flag Register"]
pub struct EVFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Flag Register"]
pub mod evfr;
#[doc = "Event Flag Set Register"]
pub struct EVFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Flag Set Register"]
pub mod evfsr;
#[doc = "Event Flag Clear Register"]
pub struct EVFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Flag Clear Register"]
pub mod evfcr;
#[doc = "Dividend Register"]
pub struct DVD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dividend Register"]
pub mod dvd;
#[doc = "Divisor Register"]
pub struct DVS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Divisor Register"]
pub mod dvs;
#[doc = "Quotient Register"]
pub struct QUOT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Quotient Register"]
pub mod quot;
#[doc = "Remainder Register"]
pub struct RMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Remainder Register"]
pub mod rmd;
#[doc = "Divider Status Register"]
pub struct DIVST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Divider Status Register"]
pub mod divst;
#[doc = "Divider Control Register"]
pub struct DIVCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Divider Control Register"]
pub mod divcon;
#[doc = "CORDIC Status and Data Control Register"]
pub struct STATC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CORDIC Status and Data Control Register"]
pub mod statc;
#[doc = "CORDIC Control Register"]
pub struct CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CORDIC Control Register"]
pub mod con;
#[doc = "CORDIC X Data Register"]
pub struct CORDX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CORDIC X Data Register"]
pub mod cordx;
#[doc = "CORDIC Y Data Register"]
pub struct CORDY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CORDIC Y Data Register"]
pub mod cordy;
#[doc = "CORDIC Z Data Register"]
pub struct CORDZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CORDIC Z Data Register"]
pub mod cordz;
#[doc = "CORDIC X Result Register"]
pub struct CORRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CORDIC X Result Register"]
pub mod corrx;
#[doc = "CORDIC Y Result Register"]
pub struct CORRY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CORDIC Y Result Register"]
pub mod corry;
#[doc = "CORDIC Z Result Register"]
pub struct CORRZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CORDIC Z Result Register"]
pub mod corrz;
