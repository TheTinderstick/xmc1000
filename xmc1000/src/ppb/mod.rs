#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - SysTick Control and Status Register"]
    pub syst_csr: SYST_CSR,
    #[doc = "0x14 - SysTick Reload Value Register"]
    pub syst_rvr: SYST_RVR,
    #[doc = "0x18 - SysTick Current Value Register"]
    pub syst_cvr: SYST_CVR,
    #[doc = "0x1c - SysTick Calibration Value Register"]
    pub syst_calib: SYST_CALIB,
    _reserved1: [u8; 224usize],
    #[doc = "0x100 - Interrupt Set-enable Register"]
    pub nvic_iser: NVIC_ISER,
    _reserved2: [u8; 124usize],
    #[doc = "0x180 - IInterrupt Clear-enable Register"]
    pub nvic_icer: NVIC_ICER,
    _reserved3: [u8; 124usize],
    #[doc = "0x200 - Interrupt Set-pending Register"]
    pub nvic_ispr: NVIC_ISPR,
    _reserved4: [u8; 124usize],
    #[doc = "0x280 - Interrupt Clear-pending Register"]
    pub nvic_icpr: NVIC_ICPR,
    _reserved5: [u8; 380usize],
    #[doc = "0x400 - Interrupt Priority Register 0"]
    pub nvic_ipr0: NVIC_IPR0,
    #[doc = "0x404 - Interrupt Priority Register 1"]
    pub nvic_ipr1: NVIC_IPR1,
    #[doc = "0x408 - Interrupt Priority Register 2"]
    pub nvic_ipr2: NVIC_IPR2,
    #[doc = "0x40c - Interrupt Priority Register 3"]
    pub nvic_ipr3: NVIC_IPR3,
    #[doc = "0x410 - Interrupt Priority Register 4"]
    pub nvic_ipr4: NVIC_IPR4,
    #[doc = "0x414 - Interrupt Priority Register 5"]
    pub nvic_ipr5: NVIC_IPR5,
    #[doc = "0x418 - Interrupt Priority Register 6"]
    pub nvic_ipr6: NVIC_IPR6,
    #[doc = "0x41c - Interrupt Priority Register 7"]
    pub nvic_ipr7: NVIC_IPR7,
    _reserved6: [u8; 2272usize],
    #[doc = "0xd00 - CPUID Base Register"]
    pub cpuid: CPUID,
    #[doc = "0xd04 - Interrupt Control and State Register"]
    pub icsr: ICSR,
    _reserved7: [u8; 4usize],
    #[doc = "0xd0c - Application Interrupt and Reset Control Register"]
    pub aircr: AIRCR,
    #[doc = "0xd10 - System Control Register"]
    pub scr: SCR,
    #[doc = "0xd14 - Configuration and Control Register"]
    pub ccr: CCR,
    _reserved8: [u8; 4usize],
    #[doc = "0xd1c - System Handler Priority Register 2"]
    pub shpr2: SHPR2,
    #[doc = "0xd20 - System Handler Priority Register 3"]
    pub shpr3: SHPR3,
    #[doc = "0xd24 - System Handler Control and State Register"]
    pub shcsr: SHCSR,
}
#[doc = "SysTick Control and Status Register"]
pub struct SYST_CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick Control and Status Register"]
pub mod syst_csr;
#[doc = "SysTick Reload Value Register"]
pub struct SYST_RVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick Reload Value Register"]
pub mod syst_rvr;
#[doc = "SysTick Current Value Register"]
pub struct SYST_CVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick Current Value Register"]
pub mod syst_cvr;
#[doc = "SysTick Calibration Value Register"]
pub struct SYST_CALIB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick Calibration Value Register"]
pub mod syst_calib;
#[doc = "Interrupt Set-enable Register"]
pub struct NVIC_ISER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Set-enable Register"]
pub mod nvic_iser;
#[doc = "IInterrupt Clear-enable Register"]
pub struct NVIC_ICER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IInterrupt Clear-enable Register"]
pub mod nvic_icer;
#[doc = "Interrupt Set-pending Register"]
pub struct NVIC_ISPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Set-pending Register"]
pub mod nvic_ispr;
#[doc = "Interrupt Clear-pending Register"]
pub struct NVIC_ICPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear-pending Register"]
pub mod nvic_icpr;
#[doc = "Interrupt Priority Register 0"]
pub struct NVIC_IPR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 0"]
pub mod nvic_ipr0;
#[doc = "Interrupt Priority Register 1"]
pub struct NVIC_IPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 1"]
pub mod nvic_ipr1;
#[doc = "Interrupt Priority Register 2"]
pub struct NVIC_IPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 2"]
pub mod nvic_ipr2;
#[doc = "Interrupt Priority Register 3"]
pub struct NVIC_IPR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 3"]
pub mod nvic_ipr3;
#[doc = "Interrupt Priority Register 4"]
pub struct NVIC_IPR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 4"]
pub mod nvic_ipr4;
#[doc = "Interrupt Priority Register 5"]
pub struct NVIC_IPR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 5"]
pub mod nvic_ipr5;
#[doc = "Interrupt Priority Register 6"]
pub struct NVIC_IPR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 6"]
pub mod nvic_ipr6;
#[doc = "Interrupt Priority Register 7"]
pub struct NVIC_IPR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register 7"]
pub mod nvic_ipr7;
#[doc = "CPUID Base Register"]
pub struct CPUID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPUID Base Register"]
pub mod cpuid;
#[doc = "Interrupt Control and State Register"]
pub struct ICSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Control and State Register"]
pub mod icsr;
#[doc = "Application Interrupt and Reset Control Register"]
pub struct AIRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Application Interrupt and Reset Control Register"]
pub mod aircr;
#[doc = "System Control Register"]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Control Register"]
pub mod scr;
#[doc = "Configuration and Control Register"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration and Control Register"]
pub mod ccr;
#[doc = "System Handler Priority Register 2"]
pub struct SHPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handler Priority Register 2"]
pub mod shpr2;
#[doc = "System Handler Priority Register 3"]
pub struct SHPR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handler Priority Register 3"]
pub mod shpr3;
#[doc = "System Handler Control and State Register"]
pub struct SHCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handler Control and State Register"]
pub mod shcsr;
