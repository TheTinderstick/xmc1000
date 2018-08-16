#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LTCHOL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `LTOL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTOL0R {
    #[doc = "Passive"]
    VALUE1,
    #[doc = "Active"]
    VALUE2,
}
impl LTOL0R {
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
            LTOL0R::VALUE1 => false,
            LTOL0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LTOL0R {
        match value {
            false => LTOL0R::VALUE1,
            true => LTOL0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LTOL0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LTOL0R::VALUE2
    }
}
#[doc = "Possible values of the field `LTOL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTOL1R {
    #[doc = "Passive"]
    VALUE1,
    #[doc = "Active"]
    VALUE2,
}
impl LTOL1R {
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
            LTOL1R::VALUE1 => false,
            LTOL1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LTOL1R {
        match value {
            false => LTOL1R::VALUE1,
            true => LTOL1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LTOL1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LTOL1R::VALUE2
    }
}
#[doc = "Possible values of the field `LTOL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTOL2R {
    #[doc = "Passive"]
    VALUE1,
    #[doc = "Active"]
    VALUE2,
}
impl LTOL2R {
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
            LTOL2R::VALUE1 => false,
            LTOL2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LTOL2R {
        match value {
            false => LTOL2R::VALUE1,
            true => LTOL2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LTOL2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LTOL2R::VALUE2
    }
}
#[doc = "Possible values of the field `LTOL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTOL3R {
    #[doc = "Passive"]
    VALUE1,
    #[doc = "Active"]
    VALUE2,
}
impl LTOL3R {
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
            LTOL3R::VALUE1 => false,
            LTOL3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LTOL3R {
        match value {
            false => LTOL3R::VALUE1,
            true => LTOL3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LTOL3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LTOL3R::VALUE2
    }
}
#[doc = "Possible values of the field `LTOL4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTOL4R {
    #[doc = "Passive"]
    VALUE1,
    #[doc = "Active"]
    VALUE2,
}
impl LTOL4R {
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
            LTOL4R::VALUE1 => false,
            LTOL4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LTOL4R {
        match value {
            false => LTOL4R::VALUE1,
            true => LTOL4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LTOL4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LTOL4R::VALUE2
    }
}
#[doc = "Possible values of the field `LTOL5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTOL5R {
    #[doc = "Passive"]
    VALUE1,
    #[doc = "Active"]
    VALUE2,
}
impl LTOL5R {
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
            LTOL5R::VALUE1 => false,
            LTOL5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LTOL5R {
        match value {
            false => LTOL5R::VALUE1,
            true => LTOL5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LTOL5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LTOL5R::VALUE2
    }
}
#[doc = "Possible values of the field `LTOL6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTOL6R {
    #[doc = "Passive"]
    VALUE1,
    #[doc = "Active"]
    VALUE2,
}
impl LTOL6R {
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
            LTOL6R::VALUE1 => false,
            LTOL6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LTOL6R {
        match value {
            false => LTOL6R::VALUE1,
            true => LTOL6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LTOL6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LTOL6R::VALUE2
    }
}
#[doc = "Possible values of the field `LTOL7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTOL7R {
    #[doc = "Passive"]
    VALUE1,
    #[doc = "Active"]
    VALUE2,
}
impl LTOL7R {
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
            LTOL7R::VALUE1 => false,
            LTOL7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LTOL7R {
        match value {
            false => LTOL7R::VALUE1,
            true => LTOL7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LTOL7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LTOL7R::VALUE2
    }
}
#[doc = "Possible values of the field `LTOL8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTOL8R {
    #[doc = "Passive"]
    VALUE1,
    #[doc = "Active"]
    VALUE2,
}
impl LTOL8R {
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
            LTOL8R::VALUE1 => false,
            LTOL8R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LTOL8R {
        match value {
            false => LTOL8R::VALUE1,
            true => LTOL8R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LTOL8R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LTOL8R::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Last Trigger Channel Output"]
    #[inline]
    pub fn ltol0(&self) -> LTOL0R {
        LTOL0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Last Trigger Channel Output"]
    #[inline]
    pub fn ltol1(&self) -> LTOL1R {
        LTOL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Last Trigger Channel Output"]
    #[inline]
    pub fn ltol2(&self) -> LTOL2R {
        LTOL2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Last Trigger Channel Output"]
    #[inline]
    pub fn ltol3(&self) -> LTOL3R {
        LTOL3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Last Trigger Channel Output"]
    #[inline]
    pub fn ltol4(&self) -> LTOL4R {
        LTOL4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Last Trigger Channel Output"]
    #[inline]
    pub fn ltol5(&self) -> LTOL5R {
        LTOL5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Last Trigger Channel Output"]
    #[inline]
    pub fn ltol6(&self) -> LTOL6R {
        LTOL6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Last Trigger Channel Output"]
    #[inline]
    pub fn ltol7(&self) -> LTOL7R {
        LTOL7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Last Trigger Channel Output"]
    #[inline]
    pub fn ltol8(&self) -> LTOL8R {
        LTOL8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
