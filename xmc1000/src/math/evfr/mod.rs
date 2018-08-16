#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EVFR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `DIVEOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVEOCR {
    #[doc = "Divider end of calculation event has not been detected."]
    VALUE1,
    #[doc = "Divider end of calculation event has been detected."]
    VALUE2,
}
impl DIVEOCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DIVEOCR::VALUE1 => false,
            DIVEOCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIVEOCR {
        match value {
            false => DIVEOCR::VALUE1,
            true => DIVEOCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIVEOCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIVEOCR::VALUE2
    }
}
#[doc = "Possible values of the field `DIVERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVERRR {
    #[doc = "Divider error event has not been detected"]
    VALUE1,
    #[doc = "Divider error event has been detected"]
    VALUE2,
}
impl DIVERRR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DIVERRR::VALUE1 => false,
            DIVERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIVERRR {
        match value {
            false => DIVERRR::VALUE1,
            true => DIVERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIVERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIVERRR::VALUE2
    }
}
#[doc = "Possible values of the field `CDEOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDEOCR {
    #[doc = "CORDIC end of calculation event has not been detected."]
    VALUE1,
    #[doc = "CORDIC end of calculation event has been detected."]
    VALUE2,
}
impl CDEOCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CDEOCR::VALUE1 => false,
            CDEOCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CDEOCR {
        match value {
            false => CDEOCR::VALUE1,
            true => CDEOCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CDEOCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CDEOCR::VALUE2
    }
}
#[doc = "Possible values of the field `CDERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDERRR {
    #[doc = "CORDIC error event has not been detected"]
    VALUE1,
    #[doc = "CORDIC error event has been detected"]
    VALUE2,
}
impl CDERRR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CDERRR::VALUE1 => false,
            CDERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CDERRR {
        match value {
            false => CDERRR::VALUE1,
            true => CDERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CDERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CDERRR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Divider End of Calculation Event Flag"]
    #[inline]
    pub fn diveoc(&self) -> DIVEOCR {
        DIVEOCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Divider Error Event Flag"]
    #[inline]
    pub fn diverr(&self) -> DIVERRR {
        DIVERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - CORDIC End of Calculation Event Flag"]
    #[inline]
    pub fn cdeoc(&self) -> CDEOCR {
        CDEOCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - CORDIC Error Event Flag"]
    #[inline]
    pub fn cderr(&self) -> CDERRR {
        CDERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
