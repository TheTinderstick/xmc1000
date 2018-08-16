#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RRASS {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `ASSRR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSRR0R {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR0R {
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
            ASSRR0R::VALUE1 => false,
            ASSRR0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSRR0R {
        match value {
            false => ASSRR0R::VALUE1,
            true => ASSRR0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSRR0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSRR0R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSRR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSRR1R {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR1R {
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
            ASSRR1R::VALUE1 => false,
            ASSRR1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSRR1R {
        match value {
            false => ASSRR1R::VALUE1,
            true => ASSRR1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSRR1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSRR1R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSRR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSRR2R {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR2R {
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
            ASSRR2R::VALUE1 => false,
            ASSRR2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSRR2R {
        match value {
            false => ASSRR2R::VALUE1,
            true => ASSRR2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSRR2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSRR2R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSRR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSRR3R {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR3R {
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
            ASSRR3R::VALUE1 => false,
            ASSRR3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSRR3R {
        match value {
            false => ASSRR3R::VALUE1,
            true => ASSRR3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSRR3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSRR3R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSRR4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSRR4R {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR4R {
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
            ASSRR4R::VALUE1 => false,
            ASSRR4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSRR4R {
        match value {
            false => ASSRR4R::VALUE1,
            true => ASSRR4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSRR4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSRR4R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSRR5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSRR5R {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR5R {
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
            ASSRR5R::VALUE1 => false,
            ASSRR5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSRR5R {
        match value {
            false => ASSRR5R::VALUE1,
            true => ASSRR5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSRR5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSRR5R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSRR6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSRR6R {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR6R {
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
            ASSRR6R::VALUE1 => false,
            ASSRR6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSRR6R {
        match value {
            false => ASSRR6R::VALUE1,
            true => ASSRR6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSRR6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSRR6R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSRR7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSRR7R {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR7R {
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
            ASSRR7R::VALUE1 => false,
            ASSRR7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSRR7R {
        match value {
            false => ASSRR7R::VALUE1,
            true => ASSRR7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSRR7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSRR7R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSRR8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSRR8R {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR8R {
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
            ASSRR8R::VALUE1 => false,
            ASSRR8R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSRR8R {
        match value {
            false => ASSRR8R::VALUE1,
            true => ASSRR8R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSRR8R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSRR8R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSRR9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSRR9R {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR9R {
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
            ASSRR9R::VALUE1 => false,
            ASSRR9R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSRR9R {
        match value {
            false => ASSRR9R::VALUE1,
            true => ASSRR9R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSRR9R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSRR9R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSRR10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSRR10R {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR10R {
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
            ASSRR10R::VALUE1 => false,
            ASSRR10R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSRR10R {
        match value {
            false => ASSRR10R::VALUE1,
            true => ASSRR10R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSRR10R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSRR10R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSRR11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSRR11R {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR11R {
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
            ASSRR11R::VALUE1 => false,
            ASSRR11R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSRR11R {
        match value {
            false => ASSRR11R::VALUE1,
            true => ASSRR11R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSRR11R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSRR11R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSRR12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSRR12R {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR12R {
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
            ASSRR12R::VALUE1 => false,
            ASSRR12R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSRR12R {
        match value {
            false => ASSRR12R::VALUE1,
            true => ASSRR12R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSRR12R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSRR12R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSRR13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSRR13R {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR13R {
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
            ASSRR13R::VALUE1 => false,
            ASSRR13R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSRR13R {
        match value {
            false => ASSRR13R::VALUE1,
            true => ASSRR13R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSRR13R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSRR13R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSRR14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSRR14R {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR14R {
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
            ASSRR14R::VALUE1 => false,
            ASSRR14R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSRR14R {
        match value {
            false => ASSRR14R::VALUE1,
            true => ASSRR14R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSRR14R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSRR14R::VALUE2
    }
}
#[doc = "Possible values of the field `ASSRR15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSRR15R {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR15R {
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
            ASSRR15R::VALUE1 => false,
            ASSRR15R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASSRR15R {
        match value {
            false => ASSRR15R::VALUE1,
            true => ASSRR15R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASSRR15R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASSRR15R::VALUE2
    }
}
#[doc = "Values that can be written to the field `ASSRR0`"]
pub enum ASSRR0W {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSRR0W::VALUE1 => false,
            ASSRR0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSRR0W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSRR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSRR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result register y can also be written by the background source"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSRR0W::VALUE1)
    }
    #[doc = "Result register y can only be written by sources within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSRR0W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASSRR1`"]
pub enum ASSRR1W {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSRR1W::VALUE1 => false,
            ASSRR1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSRR1W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSRR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSRR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result register y can also be written by the background source"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSRR1W::VALUE1)
    }
    #[doc = "Result register y can only be written by sources within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSRR1W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASSRR2`"]
pub enum ASSRR2W {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSRR2W::VALUE1 => false,
            ASSRR2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSRR2W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSRR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSRR2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result register y can also be written by the background source"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSRR2W::VALUE1)
    }
    #[doc = "Result register y can only be written by sources within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSRR2W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASSRR3`"]
pub enum ASSRR3W {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSRR3W::VALUE1 => false,
            ASSRR3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSRR3W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSRR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSRR3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result register y can also be written by the background source"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSRR3W::VALUE1)
    }
    #[doc = "Result register y can only be written by sources within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSRR3W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASSRR4`"]
pub enum ASSRR4W {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSRR4W::VALUE1 => false,
            ASSRR4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSRR4W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSRR4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSRR4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result register y can also be written by the background source"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSRR4W::VALUE1)
    }
    #[doc = "Result register y can only be written by sources within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSRR4W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASSRR5`"]
pub enum ASSRR5W {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSRR5W::VALUE1 => false,
            ASSRR5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSRR5W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSRR5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSRR5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result register y can also be written by the background source"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSRR5W::VALUE1)
    }
    #[doc = "Result register y can only be written by sources within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSRR5W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASSRR6`"]
pub enum ASSRR6W {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSRR6W::VALUE1 => false,
            ASSRR6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSRR6W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSRR6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSRR6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result register y can also be written by the background source"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSRR6W::VALUE1)
    }
    #[doc = "Result register y can only be written by sources within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSRR6W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASSRR7`"]
pub enum ASSRR7W {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSRR7W::VALUE1 => false,
            ASSRR7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSRR7W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSRR7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSRR7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result register y can also be written by the background source"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSRR7W::VALUE1)
    }
    #[doc = "Result register y can only be written by sources within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSRR7W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASSRR8`"]
pub enum ASSRR8W {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSRR8W::VALUE1 => false,
            ASSRR8W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSRR8W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSRR8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSRR8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result register y can also be written by the background source"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSRR8W::VALUE1)
    }
    #[doc = "Result register y can only be written by sources within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSRR8W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASSRR9`"]
pub enum ASSRR9W {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSRR9W::VALUE1 => false,
            ASSRR9W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSRR9W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSRR9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSRR9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result register y can also be written by the background source"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSRR9W::VALUE1)
    }
    #[doc = "Result register y can only be written by sources within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSRR9W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASSRR10`"]
pub enum ASSRR10W {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSRR10W::VALUE1 => false,
            ASSRR10W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSRR10W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSRR10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSRR10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result register y can also be written by the background source"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSRR10W::VALUE1)
    }
    #[doc = "Result register y can only be written by sources within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSRR10W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASSRR11`"]
pub enum ASSRR11W {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSRR11W::VALUE1 => false,
            ASSRR11W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSRR11W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSRR11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSRR11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result register y can also be written by the background source"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSRR11W::VALUE1)
    }
    #[doc = "Result register y can only be written by sources within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSRR11W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASSRR12`"]
pub enum ASSRR12W {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSRR12W::VALUE1 => false,
            ASSRR12W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSRR12W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSRR12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSRR12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result register y can also be written by the background source"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSRR12W::VALUE1)
    }
    #[doc = "Result register y can only be written by sources within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSRR12W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASSRR13`"]
pub enum ASSRR13W {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSRR13W::VALUE1 => false,
            ASSRR13W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSRR13W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSRR13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSRR13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result register y can also be written by the background source"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSRR13W::VALUE1)
    }
    #[doc = "Result register y can only be written by sources within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSRR13W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASSRR14`"]
pub enum ASSRR14W {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSRR14W::VALUE1 => false,
            ASSRR14W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSRR14W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSRR14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSRR14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result register y can also be written by the background source"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSRR14W::VALUE1)
    }
    #[doc = "Result register y can only be written by sources within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSRR14W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASSRR15`"]
pub enum ASSRR15W {
    #[doc = "Result register y can also be written by the background source"]
    VALUE1,
    #[doc = "Result register y can only be written by sources within group x"]
    VALUE2,
}
impl ASSRR15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASSRR15W::VALUE1 => false,
            ASSRR15W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASSRR15W<'a> {
    w: &'a mut W,
}
impl<'a> _ASSRR15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASSRR15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result register y can also be written by the background source"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSRR15W::VALUE1)
    }
    #[doc = "Result register y can only be written by sources within group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSRR15W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Assignment for Result Register 0"]
    #[inline]
    pub fn assrr0(&self) -> ASSRR0R {
        ASSRR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Assignment for Result Register 1"]
    #[inline]
    pub fn assrr1(&self) -> ASSRR1R {
        ASSRR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Assignment for Result Register 2"]
    #[inline]
    pub fn assrr2(&self) -> ASSRR2R {
        ASSRR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Assignment for Result Register 3"]
    #[inline]
    pub fn assrr3(&self) -> ASSRR3R {
        ASSRR3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Assignment for Result Register 4"]
    #[inline]
    pub fn assrr4(&self) -> ASSRR4R {
        ASSRR4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Assignment for Result Register 5"]
    #[inline]
    pub fn assrr5(&self) -> ASSRR5R {
        ASSRR5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Assignment for Result Register 6"]
    #[inline]
    pub fn assrr6(&self) -> ASSRR6R {
        ASSRR6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Assignment for Result Register 7"]
    #[inline]
    pub fn assrr7(&self) -> ASSRR7R {
        ASSRR7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Assignment for Result Register 8"]
    #[inline]
    pub fn assrr8(&self) -> ASSRR8R {
        ASSRR8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Assignment for Result Register 9"]
    #[inline]
    pub fn assrr9(&self) -> ASSRR9R {
        ASSRR9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Assignment for Result Register 10"]
    #[inline]
    pub fn assrr10(&self) -> ASSRR10R {
        ASSRR10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Assignment for Result Register 11"]
    #[inline]
    pub fn assrr11(&self) -> ASSRR11R {
        ASSRR11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Assignment for Result Register 12"]
    #[inline]
    pub fn assrr12(&self) -> ASSRR12R {
        ASSRR12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Assignment for Result Register 13"]
    #[inline]
    pub fn assrr13(&self) -> ASSRR13R {
        ASSRR13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Assignment for Result Register 14"]
    #[inline]
    pub fn assrr14(&self) -> ASSRR14R {
        ASSRR14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Assignment for Result Register 15"]
    #[inline]
    pub fn assrr15(&self) -> ASSRR15R {
        ASSRR15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Assignment for Result Register 0"]
    #[inline]
    pub fn assrr0(&mut self) -> _ASSRR0W {
        _ASSRR0W { w: self }
    }
    #[doc = "Bit 1 - Assignment for Result Register 1"]
    #[inline]
    pub fn assrr1(&mut self) -> _ASSRR1W {
        _ASSRR1W { w: self }
    }
    #[doc = "Bit 2 - Assignment for Result Register 2"]
    #[inline]
    pub fn assrr2(&mut self) -> _ASSRR2W {
        _ASSRR2W { w: self }
    }
    #[doc = "Bit 3 - Assignment for Result Register 3"]
    #[inline]
    pub fn assrr3(&mut self) -> _ASSRR3W {
        _ASSRR3W { w: self }
    }
    #[doc = "Bit 4 - Assignment for Result Register 4"]
    #[inline]
    pub fn assrr4(&mut self) -> _ASSRR4W {
        _ASSRR4W { w: self }
    }
    #[doc = "Bit 5 - Assignment for Result Register 5"]
    #[inline]
    pub fn assrr5(&mut self) -> _ASSRR5W {
        _ASSRR5W { w: self }
    }
    #[doc = "Bit 6 - Assignment for Result Register 6"]
    #[inline]
    pub fn assrr6(&mut self) -> _ASSRR6W {
        _ASSRR6W { w: self }
    }
    #[doc = "Bit 7 - Assignment for Result Register 7"]
    #[inline]
    pub fn assrr7(&mut self) -> _ASSRR7W {
        _ASSRR7W { w: self }
    }
    #[doc = "Bit 8 - Assignment for Result Register 8"]
    #[inline]
    pub fn assrr8(&mut self) -> _ASSRR8W {
        _ASSRR8W { w: self }
    }
    #[doc = "Bit 9 - Assignment for Result Register 9"]
    #[inline]
    pub fn assrr9(&mut self) -> _ASSRR9W {
        _ASSRR9W { w: self }
    }
    #[doc = "Bit 10 - Assignment for Result Register 10"]
    #[inline]
    pub fn assrr10(&mut self) -> _ASSRR10W {
        _ASSRR10W { w: self }
    }
    #[doc = "Bit 11 - Assignment for Result Register 11"]
    #[inline]
    pub fn assrr11(&mut self) -> _ASSRR11W {
        _ASSRR11W { w: self }
    }
    #[doc = "Bit 12 - Assignment for Result Register 12"]
    #[inline]
    pub fn assrr12(&mut self) -> _ASSRR12W {
        _ASSRR12W { w: self }
    }
    #[doc = "Bit 13 - Assignment for Result Register 13"]
    #[inline]
    pub fn assrr13(&mut self) -> _ASSRR13W {
        _ASSRR13W { w: self }
    }
    #[doc = "Bit 14 - Assignment for Result Register 14"]
    #[inline]
    pub fn assrr14(&mut self) -> _ASSRR14W {
        _ASSRR14W { w: self }
    }
    #[doc = "Bit 15 - Assignment for Result Register 15"]
    #[inline]
    pub fn assrr15(&mut self) -> _ASSRR15W {
        _ASSRR15W { w: self }
    }
}
