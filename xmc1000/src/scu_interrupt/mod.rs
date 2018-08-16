#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCU Raw Service Request Status"]
    pub srraw: SRRAW,
    #[doc = "0x04 - SCU Service Request Mask"]
    pub srmsk: SRMSK,
    #[doc = "0x08 - SCU Service Request Clear"]
    pub srclr: SRCLR,
    #[doc = "0x0c - SCU Service Request Set"]
    pub srset: SRSET,
}
#[doc = "SCU Raw Service Request Status"]
pub struct SRRAW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCU Raw Service Request Status"]
pub mod srraw;
#[doc = "SCU Service Request Mask"]
pub struct SRMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCU Service Request Mask"]
pub mod srmsk;
#[doc = "SCU Service Request Clear"]
pub struct SRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCU Service Request Clear"]
pub mod srclr;
#[doc = "SCU Service Request Set"]
pub struct SRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCU Service Request Set"]
pub mod srset;
