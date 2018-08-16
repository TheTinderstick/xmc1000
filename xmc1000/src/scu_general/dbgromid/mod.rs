#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DBGROMID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct MANUFIDR {
    bits: u16,
}
impl MANUFIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PARTNOR {
    bits: u16,
}
impl PARTNOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VERSIONR {
    bits: u8,
}
impl VERSIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 1:11 - Manufactory Identity"]
    #[inline]
    pub fn manufid(&self) -> MANUFIDR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MANUFIDR { bits }
    }
    #[doc = "Bits 12:27 - Part Number"]
    #[inline]
    pub fn partno(&self) -> PARTNOR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PARTNOR { bits }
    }
    #[doc = "Bits 28:31 - Product version"]
    #[inline]
    pub fn version(&self) -> VERSIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VERSIONR { bits }
    }
}
