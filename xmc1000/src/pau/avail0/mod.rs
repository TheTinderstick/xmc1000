#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AVAIL0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `AVAIL5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL5R {
    #[doc = "RAM block 1 is not available."]
    VALUE1,
    #[doc = "RAM block 1 is available."]
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
    #[doc = "RAM block 2 is not available."]
    VALUE1,
    #[doc = "RAM block 2 is available."]
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
    #[doc = "RAM block 3 is not available."]
    VALUE1,
    #[doc = "RAM block 3 is available."]
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
#[doc = "Possible values of the field `AVAIL20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL20R {
    #[doc = "MATH Global SFRs and Divider are not available."]
    VALUE1,
    #[doc = "MATH Global SFRs and Divider are available."]
    VALUE2,
}
impl AVAIL20R {
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
            AVAIL20R::VALUE1 => false,
            AVAIL20R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL20R {
        match value {
            false => AVAIL20R::VALUE1,
            true => AVAIL20R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL20R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL20R::VALUE2
    }
}
#[doc = "Possible values of the field `AVAIL21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL21R {
    #[doc = "MATH CORDIC is not available."]
    VALUE1,
    #[doc = "MATH CORDIC is available."]
    VALUE2,
}
impl AVAIL21R {
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
            AVAIL21R::VALUE1 => false,
            AVAIL21R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL21R {
        match value {
            false => AVAIL21R::VALUE1,
            true => AVAIL21R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL21R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL21R::VALUE2
    }
}
#[doc = "Possible values of the field `AVAIL22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL22R {
    #[doc = "Port 0 is not available."]
    VALUE1,
    #[doc = "Port 0 is available."]
    VALUE2,
}
impl AVAIL22R {
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
            AVAIL22R::VALUE1 => false,
            AVAIL22R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL22R {
        match value {
            false => AVAIL22R::VALUE1,
            true => AVAIL22R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL22R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL22R::VALUE2
    }
}
#[doc = "Possible values of the field `AVAIL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL23R {
    #[doc = "Port 1 is not available."]
    VALUE1,
    #[doc = "Port 1 is available."]
    VALUE2,
}
impl AVAIL23R {
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
            AVAIL23R::VALUE1 => false,
            AVAIL23R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL23R {
        match value {
            false => AVAIL23R::VALUE1,
            true => AVAIL23R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL23R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL23R::VALUE2
    }
}
#[doc = "Possible values of the field `AVAIL24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL24R {
    #[doc = "Port 2 is not available."]
    VALUE1,
    #[doc = "Port 2 is available."]
    VALUE2,
}
impl AVAIL24R {
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
            AVAIL24R::VALUE1 => false,
            AVAIL24R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVAIL24R {
        match value {
            false => AVAIL24R::VALUE1,
            true => AVAIL24R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL24R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL24R::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 5 - RAM Block 1 Availability Flag"]
    #[inline]
    pub fn avail5(&self) -> AVAIL5R {
        AVAIL5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - RAM Block 2 Availability Flag"]
    #[inline]
    pub fn avail6(&self) -> AVAIL6R {
        AVAIL6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - RAM Block 3 Availability Flag"]
    #[inline]
    pub fn avail7(&self) -> AVAIL7R {
        AVAIL7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - MATH Global SFRs and Divider Availability Flag"]
    #[inline]
    pub fn avail20(&self) -> AVAIL20R {
        AVAIL20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - MATH CORDIC Availability Flag"]
    #[inline]
    pub fn avail21(&self) -> AVAIL21R {
        AVAIL21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Port 0 Availability Flag"]
    #[inline]
    pub fn avail22(&self) -> AVAIL22R {
        AVAIL22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Port 1 Availability Flag"]
    #[inline]
    pub fn avail23(&self) -> AVAIL23R {
        AVAIL23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Port 0 Availability Flag"]
    #[inline]
    pub fn avail24(&self) -> AVAIL24R {
        AVAIL24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
