#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PDISC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PDIS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS0R {
    #[doc = "Pad P1.x is enabled."]
    VALUE1,
    #[doc = "Pad P1.x is disabled."]
    VALUE2,
}
impl PDIS0R {
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
            PDIS0R::VALUE1 => false,
            PDIS0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS0R {
        match value {
            false => PDIS0R::VALUE1,
            true => PDIS0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS0R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS1R {
    #[doc = "Pad P1.x is enabled."]
    VALUE1,
    #[doc = "Pad P1.x is disabled."]
    VALUE2,
}
impl PDIS1R {
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
            PDIS1R::VALUE1 => false,
            PDIS1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS1R {
        match value {
            false => PDIS1R::VALUE1,
            true => PDIS1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS1R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS2R {
    #[doc = "Pad P1.x is enabled."]
    VALUE1,
    #[doc = "Pad P1.x is disabled."]
    VALUE2,
}
impl PDIS2R {
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
            PDIS2R::VALUE1 => false,
            PDIS2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS2R {
        match value {
            false => PDIS2R::VALUE1,
            true => PDIS2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS2R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS3R {
    #[doc = "Pad P1.x is enabled."]
    VALUE1,
    #[doc = "Pad P1.x is disabled."]
    VALUE2,
}
impl PDIS3R {
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
            PDIS3R::VALUE1 => false,
            PDIS3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS3R {
        match value {
            false => PDIS3R::VALUE1,
            true => PDIS3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS3R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS4R {
    #[doc = "Pad P1.x is enabled."]
    VALUE1,
    #[doc = "Pad P1.x is disabled."]
    VALUE2,
}
impl PDIS4R {
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
            PDIS4R::VALUE1 => false,
            PDIS4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS4R {
        match value {
            false => PDIS4R::VALUE1,
            true => PDIS4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS4R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS5R {
    #[doc = "Pad P1.x is enabled."]
    VALUE1,
    #[doc = "Pad P1.x is disabled."]
    VALUE2,
}
impl PDIS5R {
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
            PDIS5R::VALUE1 => false,
            PDIS5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS5R {
        match value {
            false => PDIS5R::VALUE1,
            true => PDIS5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS5R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS6R {
    #[doc = "Pad P1.x is enabled."]
    VALUE1,
    #[doc = "Pad P1.x is disabled."]
    VALUE2,
}
impl PDIS6R {
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
            PDIS6R::VALUE1 => false,
            PDIS6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS6R {
        match value {
            false => PDIS6R::VALUE1,
            true => PDIS6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS6R::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Pad Disable for Port 1 Pin 0"]
    #[inline]
    pub fn pdis0(&self) -> PDIS0R {
        PDIS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pad Disable for Port 1 Pin 1"]
    #[inline]
    pub fn pdis1(&self) -> PDIS1R {
        PDIS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Pad Disable for Port 1 Pin 2"]
    #[inline]
    pub fn pdis2(&self) -> PDIS2R {
        PDIS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Pad Disable for Port 1 Pin 3"]
    #[inline]
    pub fn pdis3(&self) -> PDIS3R {
        PDIS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Pad Disable for Port 1 Pin 4"]
    #[inline]
    pub fn pdis4(&self) -> PDIS4R {
        PDIS4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Pad Disable for Port 1 Pin 5"]
    #[inline]
    pub fn pdis5(&self) -> PDIS5R {
        PDIS5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Pad Disable for Port 1 Pin 6"]
    #[inline]
    pub fn pdis6(&self) -> PDIS6R {
        PDIS6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
