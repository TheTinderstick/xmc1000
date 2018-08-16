#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::VDESR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `VCLIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCLIPR {
    #[doc = "VCLIP is not active"]
    VALUE1,
    #[doc = "VCLIP is active"]
    VALUE2,
}
impl VCLIPR {
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
            VCLIPR::VALUE1 => false,
            VCLIPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VCLIPR {
        match value {
            false => VCLIPR::VALUE1,
            true => VCLIPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VCLIPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VCLIPR::VALUE2
    }
}
#[doc = "Possible values of the field `VDDPPW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDPPWR {
    #[doc = "VDDP is above pre-warning threshold"]
    VALUE1,
    #[doc = "VDDP is below pre-warningthreshold"]
    VALUE2,
}
impl VDDPPWR {
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
            VDDPPWR::VALUE1 => false,
            VDDPPWR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VDDPPWR {
        match value {
            false => VDDPPWR::VALUE1,
            true => VDDPPWR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VDDPPWR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VDDPPWR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - VCLIP Indication"]
    #[inline]
    pub fn vclip(&self) -> VCLIPR {
        VCLIPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - VDDPPW Indication"]
    #[inline]
    pub fn vddppw(&self) -> VDDPPWR {
        VDDPPWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
