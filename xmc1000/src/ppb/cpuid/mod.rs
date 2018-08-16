#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CPUID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `Revision`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REVISIONR {
    #[doc = "Patch 0"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REVISIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REVISIONR::VALUE1 => 0,
            REVISIONR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REVISIONR {
        match value {
            0 => REVISIONR::VALUE1,
            i => REVISIONR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REVISIONR::VALUE1
    }
}
#[doc = "Possible values of the field `PartNo`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARTNOR {
    #[doc = "Cortex-M0"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl PARTNOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            PARTNOR::VALUE1 => 3104,
            PARTNOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> PARTNOR {
        match value {
            3104 => PARTNOR::VALUE1,
            i => PARTNOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PARTNOR::VALUE1
    }
}
#[doc = "Possible values of the field `Architecture`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARCHITECTURER {
    #[doc = "ARMv6-M"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ARCHITECTURER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ARCHITECTURER::VALUE1 => 12,
            ARCHITECTURER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ARCHITECTURER {
        match value {
            12 => ARCHITECTURER::VALUE1,
            i => ARCHITECTURER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ARCHITECTURER::VALUE1
    }
}
#[doc = "Possible values of the field `Variant`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VARIANTR {
    #[doc = "Revision 0"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VARIANTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VARIANTR::VALUE1 => 0,
            VARIANTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VARIANTR {
        match value {
            0 => VARIANTR::VALUE1,
            i => VARIANTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VARIANTR::VALUE1
    }
}
#[doc = "Possible values of the field `Implementer`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMPLEMENTERR {
    #[doc = "ARM"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IMPLEMENTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IMPLEMENTERR::VALUE1 => 65,
            IMPLEMENTERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IMPLEMENTERR {
        match value {
            65 => IMPLEMENTERR::VALUE1,
            i => IMPLEMENTERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IMPLEMENTERR::VALUE1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Revision Number"]
    #[inline]
    pub fn revision(&self) -> REVISIONR {
        REVISIONR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:15 - Part Number of the Processor"]
    #[inline]
    pub fn part_no(&self) -> PARTNOR {
        PARTNOR::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:19 - Architecture"]
    #[inline]
    pub fn architecture(&self) -> ARCHITECTURER {
        ARCHITECTURER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Variant Number"]
    #[inline]
    pub fn variant(&self) -> VARIANTR {
        VARIANTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:31 - Implementer Code"]
    #[inline]
    pub fn implementer(&self) -> IMPLEMENTERR {
        IMPLEMENTERR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
