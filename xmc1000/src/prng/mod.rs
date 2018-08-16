#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PRNG Word Register"]
    pub word: WORD,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - PRNG Status Check Register"]
    pub chk: CHK,
    _reserved1: [u8; 6usize],
    #[doc = "0x0c - PRNG Control Register"]
    pub ctrl: CTRL,
}
#[doc = "PRNG Word Register"]
pub struct WORD {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PRNG Word Register"]
pub mod word;
#[doc = "PRNG Status Check Register"]
pub struct CHK {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PRNG Status Check Register"]
pub mod chk;
#[doc = "PRNG Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PRNG Control Register"]
pub mod ctrl;
