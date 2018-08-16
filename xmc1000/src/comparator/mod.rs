#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Out Of Range Comparator Control Register"]
    pub orcctrl: ORCCTRL,
    _reserved0: [u8; 2904usize],
    #[doc = "0xb5c - Analog Comparator 0 Control Register"]
    pub anacmp0: ANACMP0,
    _reserved1: [u8; 2usize],
    #[doc = "0xb60 - Analog Comparator 1 Control Register"]
    pub anacmp1: ANACMP1,
    _reserved2: [u8; 2usize],
    #[doc = "0xb64 - Analog Comparator 2 Control Register"]
    pub anacmp2: ANACMP2,
}
#[doc = "Out Of Range Comparator Control Register"]
pub struct ORCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Out Of Range Comparator Control Register"]
pub mod orcctrl;
#[doc = "Analog Comparator 0 Control Register"]
pub struct ANACMP0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Analog Comparator 0 Control Register"]
pub mod anacmp0;
#[doc = "Analog Comparator 1 Control Register"]
pub struct ANACMP1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Analog Comparator 1 Control Register"]
pub mod anacmp1;
#[doc = "Analog Comparator 2 Control Register"]
pub struct ANACMP2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Analog Comparator 2 Control Register"]
pub mod anacmp2;
