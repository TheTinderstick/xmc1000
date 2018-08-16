#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AVAIL2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `AVAIL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL0R {
    #[doc = "CC80 and CCU80 kernel SFRs are not available."]
    VALUE1,
    #[doc = "CC80 and CCU80 kernel SFRs are available."]
    VALUE2,
}
impl AVAIL0R {
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
            AVAIL0R::VALUE1 => false,
            AVAIL0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL0R {
        match value {
            false => AVAIL0R::VALUE1,
            true => AVAIL0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL0R::VALUE2
    }
}
#[doc = "Possible values of the field `AVAIL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL1R {
    #[doc = "CC81 is not available."]
    VALUE1,
    #[doc = "CC81 is available."]
    VALUE2,
}
impl AVAIL1R {
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
            AVAIL1R::VALUE1 => false,
            AVAIL1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL1R {
        match value {
            false => AVAIL1R::VALUE1,
            true => AVAIL1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL1R::VALUE2
    }
}
#[doc = "Possible values of the field `AVAIL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL2R {
    #[doc = "CC82 is not available."]
    VALUE1,
    #[doc = "CC82 is available."]
    VALUE2,
}
impl AVAIL2R {
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
            AVAIL2R::VALUE1 => false,
            AVAIL2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL2R {
        match value {
            false => AVAIL2R::VALUE1,
            true => AVAIL2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL2R::VALUE2
    }
}
#[doc = "Possible values of the field `AVAIL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL3R {
    #[doc = "CC83 is not available."]
    VALUE1,
    #[doc = "CC83 is available."]
    VALUE2,
}
impl AVAIL3R {
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
            AVAIL3R::VALUE1 => false,
            AVAIL3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL3R {
        match value {
            false => AVAIL3R::VALUE1,
            true => AVAIL3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL3R::VALUE2
    }
}
#[doc = "Possible values of the field `AVAIL12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL12R {
    #[doc = "POSIF0 is not available."]
    VALUE1,
    #[doc = "POSIF0 is available."]
    VALUE2,
}
impl AVAIL12R {
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
            AVAIL12R::VALUE1 => false,
            AVAIL12R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL12R {
        match value {
            false => AVAIL12R::VALUE1,
            true => AVAIL12R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL12R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL12R::VALUE2
    }
}
#[doc = "Possible values of the field `AVAIL15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL15R {
    #[doc = "BCCU0 is not available."]
    VALUE1,
    #[doc = "BCCU0 is available."]
    VALUE2,
}
impl AVAIL15R {
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
            AVAIL15R::VALUE1 => false,
            AVAIL15R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL15R {
        match value {
            false => AVAIL15R::VALUE1,
            true => AVAIL15R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL15R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL15R::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - CC80 and CCU80 kernel SFRs Availability Flag"]
    #[inline]
    pub fn avail0(&self) -> AVAIL0R {
        AVAIL0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - CC81 Availability Flag"]
    #[inline]
    pub fn avail1(&self) -> AVAIL1R {
        AVAIL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - CC82 Availability Flag"]
    #[inline]
    pub fn avail2(&self) -> AVAIL2R {
        AVAIL2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - CC83 Availability Flag"]
    #[inline]
    pub fn avail3(&self) -> AVAIL3R {
        AVAIL3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - POSIF0 Availability Flag"]
    #[inline]
    pub fn avail12(&self) -> AVAIL12R {
        AVAIL12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - BCCU0 Availability Flag"]
    #[inline]
    pub fn avail15(&self) -> AVAIL15R {
        AVAIL15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
