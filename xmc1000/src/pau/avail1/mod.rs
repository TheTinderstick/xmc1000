#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AVAIL1 {
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
    #[doc = "USIC0 Channel 0 is not available."]
    VALUE1,
    #[doc = "USIC0 Channel 0 is available."]
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
    #[doc = "USIC0 Channel 1 is not available."]
    VALUE1,
    #[doc = "USIC0 Channel 1 is available."]
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
#[doc = "Possible values of the field `AVAIL4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL4R {
    #[doc = "PRNG is not available."]
    VALUE1,
    #[doc = "PRNG is available."]
    VALUE2,
}
impl AVAIL4R {
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
            AVAIL4R::VALUE1 => false,
            AVAIL4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL4R {
        match value {
            false => AVAIL4R::VALUE1,
            true => AVAIL4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL4R::VALUE2
    }
}
#[doc = "Possible values of the field `AVAIL5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL5R {
    #[doc = "VADC0 Basic SFRs are not available."]
    VALUE1,
    #[doc = "VADC0 Basic SFRs are available."]
    VALUE2,
}
impl AVAIL5R {
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
            AVAIL5R::VALUE1 => false,
            AVAIL5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL5R {
        match value {
            false => AVAIL5R::VALUE1,
            true => AVAIL5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL5R::VALUE2
    }
}
#[doc = "Possible values of the field `AVAIL6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL6R {
    #[doc = "VADC0 Group 0 SFRs are not available."]
    VALUE1,
    #[doc = "VADC0 Group 0 SFRs are available."]
    VALUE2,
}
impl AVAIL6R {
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
            AVAIL6R::VALUE1 => false,
            AVAIL6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL6R {
        match value {
            false => AVAIL6R::VALUE1,
            true => AVAIL6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL6R::VALUE2
    }
}
#[doc = "Possible values of the field `AVAIL7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL7R {
    #[doc = "VADC0 Group 1 SFRs are not available."]
    VALUE1,
    #[doc = "VADC0 Group 1 SFRs are available."]
    VALUE2,
}
impl AVAIL7R {
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
            AVAIL7R::VALUE1 => false,
            AVAIL7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL7R {
        match value {
            false => AVAIL7R::VALUE1,
            true => AVAIL7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL7R::VALUE2
    }
}
#[doc = "Possible values of the field `AVAIL8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL8R {
    #[doc = "SHS0 is not available."]
    VALUE1,
    #[doc = "SHS0 is available."]
    VALUE2,
}
impl AVAIL8R {
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
            AVAIL8R::VALUE1 => false,
            AVAIL8R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL8R {
        match value {
            false => AVAIL8R::VALUE1,
            true => AVAIL8R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL8R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL8R::VALUE2
    }
}
#[doc = "Possible values of the field `AVAIL9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL9R {
    #[doc = "CC40 is not available."]
    VALUE1,
    #[doc = "CC40 is available."]
    VALUE2,
}
impl AVAIL9R {
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
            AVAIL9R::VALUE1 => false,
            AVAIL9R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL9R {
        match value {
            false => AVAIL9R::VALUE1,
            true => AVAIL9R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL9R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL9R::VALUE2
    }
}
#[doc = "Possible values of the field `AVAIL10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL10R {
    #[doc = "CC41 is not available."]
    VALUE1,
    #[doc = "CC41 is available."]
    VALUE2,
}
impl AVAIL10R {
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
            AVAIL10R::VALUE1 => false,
            AVAIL10R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL10R {
        match value {
            false => AVAIL10R::VALUE1,
            true => AVAIL10R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL10R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL10R::VALUE2
    }
}
#[doc = "Possible values of the field `AVAIL11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL11R {
    #[doc = "CC42 is not available."]
    VALUE1,
    #[doc = "CC42 is available."]
    VALUE2,
}
impl AVAIL11R {
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
            AVAIL11R::VALUE1 => false,
            AVAIL11R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL11R {
        match value {
            false => AVAIL11R::VALUE1,
            true => AVAIL11R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL11R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL11R::VALUE2
    }
}
#[doc = "Possible values of the field `AVAIL12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL12R {
    #[doc = "CC43 is not available."]
    VALUE1,
    #[doc = "CC43 is available."]
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - USIC0 Channel 0 Availability Flag"]
    #[inline]
    pub fn avail0(&self) -> AVAIL0R {
        AVAIL0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - USIC0 Channel 1 Availability Flag"]
    #[inline]
    pub fn avail1(&self) -> AVAIL1R {
        AVAIL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - PRNG Availability Flag"]
    #[inline]
    pub fn avail4(&self) -> AVAIL4R {
        AVAIL4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - VADC0 Basic SFRs Availability Flag"]
    #[inline]
    pub fn avail5(&self) -> AVAIL5R {
        AVAIL5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - VADC0 Group 0 SFRs Availability Flag"]
    #[inline]
    pub fn avail6(&self) -> AVAIL6R {
        AVAIL6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - VADC0 Group 1 SFRs Availability Flag"]
    #[inline]
    pub fn avail7(&self) -> AVAIL7R {
        AVAIL7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - SHS0 Availability Flag"]
    #[inline]
    pub fn avail8(&self) -> AVAIL8R {
        AVAIL8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - CC40 Availability Flag"]
    #[inline]
    pub fn avail9(&self) -> AVAIL9R {
        AVAIL9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - CC41 Availability Flag"]
    #[inline]
    pub fn avail10(&self) -> AVAIL10R {
        AVAIL10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - CC42 Availability Flag"]
    #[inline]
    pub fn avail11(&self) -> AVAIL11R {
        AVAIL11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - CC43 Availability Flag"]
    #[inline]
    pub fn avail12(&self) -> AVAIL12R {
        AVAIL12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
