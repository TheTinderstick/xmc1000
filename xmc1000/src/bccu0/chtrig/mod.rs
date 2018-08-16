#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHTRIG {
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
#[doc = "Possible values of the field `ET0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ET0R {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET0R {
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
            ET0R::VALUE1 => false,
            ET0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ET0R {
        match value {
            false => ET0R::VALUE1,
            true => ET0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ET0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ET0R::VALUE2
    }
}
#[doc = "Possible values of the field `ET1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ET1R {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET1R {
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
            ET1R::VALUE1 => false,
            ET1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ET1R {
        match value {
            false => ET1R::VALUE1,
            true => ET1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ET1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ET1R::VALUE2
    }
}
#[doc = "Possible values of the field `ET2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ET2R {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET2R {
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
            ET2R::VALUE1 => false,
            ET2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ET2R {
        match value {
            false => ET2R::VALUE1,
            true => ET2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ET2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ET2R::VALUE2
    }
}
#[doc = "Possible values of the field `ET3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ET3R {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET3R {
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
            ET3R::VALUE1 => false,
            ET3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ET3R {
        match value {
            false => ET3R::VALUE1,
            true => ET3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ET3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ET3R::VALUE2
    }
}
#[doc = "Possible values of the field `ET4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ET4R {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET4R {
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
            ET4R::VALUE1 => false,
            ET4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ET4R {
        match value {
            false => ET4R::VALUE1,
            true => ET4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ET4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ET4R::VALUE2
    }
}
#[doc = "Possible values of the field `ET5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ET5R {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET5R {
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
            ET5R::VALUE1 => false,
            ET5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ET5R {
        match value {
            false => ET5R::VALUE1,
            true => ET5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ET5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ET5R::VALUE2
    }
}
#[doc = "Possible values of the field `ET6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ET6R {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET6R {
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
            ET6R::VALUE1 => false,
            ET6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ET6R {
        match value {
            false => ET6R::VALUE1,
            true => ET6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ET6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ET6R::VALUE2
    }
}
#[doc = "Possible values of the field `ET7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ET7R {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET7R {
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
            ET7R::VALUE1 => false,
            ET7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ET7R {
        match value {
            false => ET7R::VALUE1,
            true => ET7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ET7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ET7R::VALUE2
    }
}
#[doc = "Possible values of the field `ET8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ET8R {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET8R {
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
            ET8R::VALUE1 => false,
            ET8R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ET8R {
        match value {
            false => ET8R::VALUE1,
            true => ET8R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ET8R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ET8R::VALUE2
    }
}
#[doc = "Possible values of the field `TOS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS0R {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS0R {
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
            TOS0R::VALUE1 => false,
            TOS0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOS0R {
        match value {
            false => TOS0R::VALUE1,
            true => TOS0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TOS0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TOS0R::VALUE2
    }
}
#[doc = "Possible values of the field `TOS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS1R {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS1R {
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
            TOS1R::VALUE1 => false,
            TOS1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOS1R {
        match value {
            false => TOS1R::VALUE1,
            true => TOS1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TOS1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TOS1R::VALUE2
    }
}
#[doc = "Possible values of the field `TOS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS2R {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS2R {
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
            TOS2R::VALUE1 => false,
            TOS2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOS2R {
        match value {
            false => TOS2R::VALUE1,
            true => TOS2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TOS2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TOS2R::VALUE2
    }
}
#[doc = "Possible values of the field `TOS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS3R {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS3R {
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
            TOS3R::VALUE1 => false,
            TOS3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOS3R {
        match value {
            false => TOS3R::VALUE1,
            true => TOS3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TOS3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TOS3R::VALUE2
    }
}
#[doc = "Possible values of the field `TOS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS4R {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS4R {
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
            TOS4R::VALUE1 => false,
            TOS4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOS4R {
        match value {
            false => TOS4R::VALUE1,
            true => TOS4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TOS4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TOS4R::VALUE2
    }
}
#[doc = "Possible values of the field `TOS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS5R {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS5R {
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
            TOS5R::VALUE1 => false,
            TOS5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOS5R {
        match value {
            false => TOS5R::VALUE1,
            true => TOS5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TOS5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TOS5R::VALUE2
    }
}
#[doc = "Possible values of the field `TOS6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS6R {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS6R {
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
            TOS6R::VALUE1 => false,
            TOS6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOS6R {
        match value {
            false => TOS6R::VALUE1,
            true => TOS6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TOS6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TOS6R::VALUE2
    }
}
#[doc = "Possible values of the field `TOS7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS7R {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS7R {
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
            TOS7R::VALUE1 => false,
            TOS7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOS7R {
        match value {
            false => TOS7R::VALUE1,
            true => TOS7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TOS7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TOS7R::VALUE2
    }
}
#[doc = "Possible values of the field `TOS8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS8R {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS8R {
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
            TOS8R::VALUE1 => false,
            TOS8R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOS8R {
        match value {
            false => TOS8R::VALUE1,
            true => TOS8R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TOS8R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TOS8R::VALUE2
    }
}
#[doc = "Values that can be written to the field `ET0`"]
pub enum ET0W {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ET0W::VALUE1 => false,
            ET0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ET0W<'a> {
    w: &'a mut W,
}
impl<'a> _ET0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ET0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel trigger is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ET0W::VALUE1)
    }
    #[doc = "Channel trigger is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ET0W::VALUE2)
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
#[doc = "Values that can be written to the field `ET1`"]
pub enum ET1W {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ET1W::VALUE1 => false,
            ET1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ET1W<'a> {
    w: &'a mut W,
}
impl<'a> _ET1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ET1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel trigger is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ET1W::VALUE1)
    }
    #[doc = "Channel trigger is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ET1W::VALUE2)
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
#[doc = "Values that can be written to the field `ET2`"]
pub enum ET2W {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ET2W::VALUE1 => false,
            ET2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ET2W<'a> {
    w: &'a mut W,
}
impl<'a> _ET2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ET2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel trigger is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ET2W::VALUE1)
    }
    #[doc = "Channel trigger is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ET2W::VALUE2)
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
#[doc = "Values that can be written to the field `ET3`"]
pub enum ET3W {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ET3W::VALUE1 => false,
            ET3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ET3W<'a> {
    w: &'a mut W,
}
impl<'a> _ET3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ET3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel trigger is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ET3W::VALUE1)
    }
    #[doc = "Channel trigger is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ET3W::VALUE2)
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
#[doc = "Values that can be written to the field `ET4`"]
pub enum ET4W {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ET4W::VALUE1 => false,
            ET4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ET4W<'a> {
    w: &'a mut W,
}
impl<'a> _ET4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ET4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel trigger is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ET4W::VALUE1)
    }
    #[doc = "Channel trigger is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ET4W::VALUE2)
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
#[doc = "Values that can be written to the field `ET5`"]
pub enum ET5W {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ET5W::VALUE1 => false,
            ET5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ET5W<'a> {
    w: &'a mut W,
}
impl<'a> _ET5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ET5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel trigger is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ET5W::VALUE1)
    }
    #[doc = "Channel trigger is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ET5W::VALUE2)
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
#[doc = "Values that can be written to the field `ET6`"]
pub enum ET6W {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ET6W::VALUE1 => false,
            ET6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ET6W<'a> {
    w: &'a mut W,
}
impl<'a> _ET6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ET6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel trigger is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ET6W::VALUE1)
    }
    #[doc = "Channel trigger is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ET6W::VALUE2)
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
#[doc = "Values that can be written to the field `ET7`"]
pub enum ET7W {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ET7W::VALUE1 => false,
            ET7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ET7W<'a> {
    w: &'a mut W,
}
impl<'a> _ET7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ET7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel trigger is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ET7W::VALUE1)
    }
    #[doc = "Channel trigger is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ET7W::VALUE2)
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
#[doc = "Values that can be written to the field `ET8`"]
pub enum ET8W {
    #[doc = "Channel trigger is disabled"]
    VALUE1,
    #[doc = "Channel trigger is enabled"]
    VALUE2,
}
impl ET8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ET8W::VALUE1 => false,
            ET8W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ET8W<'a> {
    w: &'a mut W,
}
impl<'a> _ET8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ET8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel trigger is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ET8W::VALUE1)
    }
    #[doc = "Channel trigger is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ET8W::VALUE2)
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
#[doc = "Values that can be written to the field `TOS0`"]
pub enum TOS0W {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOS0W::VALUE1 => false,
            TOS0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOS0W<'a> {
    w: &'a mut W,
}
impl<'a> _TOS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TOS0W::VALUE1)
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TOS0W::VALUE2)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TOS1`"]
pub enum TOS1W {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOS1W::VALUE1 => false,
            TOS1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOS1W<'a> {
    w: &'a mut W,
}
impl<'a> _TOS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TOS1W::VALUE1)
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TOS1W::VALUE2)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TOS2`"]
pub enum TOS2W {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOS2W::VALUE1 => false,
            TOS2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOS2W<'a> {
    w: &'a mut W,
}
impl<'a> _TOS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TOS2W::VALUE1)
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TOS2W::VALUE2)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TOS3`"]
pub enum TOS3W {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOS3W::VALUE1 => false,
            TOS3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOS3W<'a> {
    w: &'a mut W,
}
impl<'a> _TOS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOS3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TOS3W::VALUE1)
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TOS3W::VALUE2)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TOS4`"]
pub enum TOS4W {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOS4W::VALUE1 => false,
            TOS4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOS4W<'a> {
    w: &'a mut W,
}
impl<'a> _TOS4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOS4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TOS4W::VALUE1)
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TOS4W::VALUE2)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TOS5`"]
pub enum TOS5W {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOS5W::VALUE1 => false,
            TOS5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOS5W<'a> {
    w: &'a mut W,
}
impl<'a> _TOS5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOS5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TOS5W::VALUE1)
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TOS5W::VALUE2)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TOS6`"]
pub enum TOS6W {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOS6W::VALUE1 => false,
            TOS6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOS6W<'a> {
    w: &'a mut W,
}
impl<'a> _TOS6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOS6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TOS6W::VALUE1)
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TOS6W::VALUE2)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TOS7`"]
pub enum TOS7W {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOS7W::VALUE1 => false,
            TOS7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOS7W<'a> {
    w: &'a mut W,
}
impl<'a> _TOS7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOS7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TOS7W::VALUE1)
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TOS7W::VALUE2)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TOS8`"]
pub enum TOS8W {
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    VALUE1,
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    VALUE2,
}
impl TOS8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOS8W::VALUE1 => false,
            TOS8W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOS8W<'a> {
    w: &'a mut W,
}
impl<'a> _TOS8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOS8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TOS8W::VALUE1)
    }
    #[doc = "The channel trigger pulse will appear on BCCU_TRIGOUT1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TOS8W::VALUE2)
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
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Channel 0 Trigger Enable"]
    #[inline]
    pub fn et0(&self) -> ET0R {
        ET0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel 1 Trigger Enable"]
    #[inline]
    pub fn et1(&self) -> ET1R {
        ET1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 2 Trigger Enable"]
    #[inline]
    pub fn et2(&self) -> ET2R {
        ET2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 3 Trigger Enable"]
    #[inline]
    pub fn et3(&self) -> ET3R {
        ET3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel 4 Trigger Enable"]
    #[inline]
    pub fn et4(&self) -> ET4R {
        ET4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel 5 Trigger Enable"]
    #[inline]
    pub fn et5(&self) -> ET5R {
        ET5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channel 6 Trigger Enable"]
    #[inline]
    pub fn et6(&self) -> ET6R {
        ET6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel 7 Trigger Enable"]
    #[inline]
    pub fn et7(&self) -> ET7R {
        ET7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Channel 8 Trigger Enable"]
    #[inline]
    pub fn et8(&self) -> ET8R {
        ET8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Channel 0 Trigger Output Select"]
    #[inline]
    pub fn tos0(&self) -> TOS0R {
        TOS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Channel 1 Trigger Output Select"]
    #[inline]
    pub fn tos1(&self) -> TOS1R {
        TOS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Channel 2 Trigger Output Select"]
    #[inline]
    pub fn tos2(&self) -> TOS2R {
        TOS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Channel 3 Trigger Output Select"]
    #[inline]
    pub fn tos3(&self) -> TOS3R {
        TOS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Channel 4 Trigger Output Select"]
    #[inline]
    pub fn tos4(&self) -> TOS4R {
        TOS4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Channel 5 Trigger Output Select"]
    #[inline]
    pub fn tos5(&self) -> TOS5R {
        TOS5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Channel 6 Trigger Output Select"]
    #[inline]
    pub fn tos6(&self) -> TOS6R {
        TOS6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Channel 7 Trigger Output Select"]
    #[inline]
    pub fn tos7(&self) -> TOS7R {
        TOS7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Channel 8 Trigger Output Select"]
    #[inline]
    pub fn tos8(&self) -> TOS8R {
        TOS8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Channel 0 Trigger Enable"]
    #[inline]
    pub fn et0(&mut self) -> _ET0W {
        _ET0W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Trigger Enable"]
    #[inline]
    pub fn et1(&mut self) -> _ET1W {
        _ET1W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Trigger Enable"]
    #[inline]
    pub fn et2(&mut self) -> _ET2W {
        _ET2W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Trigger Enable"]
    #[inline]
    pub fn et3(&mut self) -> _ET3W {
        _ET3W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Trigger Enable"]
    #[inline]
    pub fn et4(&mut self) -> _ET4W {
        _ET4W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Trigger Enable"]
    #[inline]
    pub fn et5(&mut self) -> _ET5W {
        _ET5W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Trigger Enable"]
    #[inline]
    pub fn et6(&mut self) -> _ET6W {
        _ET6W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Trigger Enable"]
    #[inline]
    pub fn et7(&mut self) -> _ET7W {
        _ET7W { w: self }
    }
    #[doc = "Bit 8 - Channel 8 Trigger Enable"]
    #[inline]
    pub fn et8(&mut self) -> _ET8W {
        _ET8W { w: self }
    }
    #[doc = "Bit 16 - Channel 0 Trigger Output Select"]
    #[inline]
    pub fn tos0(&mut self) -> _TOS0W {
        _TOS0W { w: self }
    }
    #[doc = "Bit 17 - Channel 1 Trigger Output Select"]
    #[inline]
    pub fn tos1(&mut self) -> _TOS1W {
        _TOS1W { w: self }
    }
    #[doc = "Bit 18 - Channel 2 Trigger Output Select"]
    #[inline]
    pub fn tos2(&mut self) -> _TOS2W {
        _TOS2W { w: self }
    }
    #[doc = "Bit 19 - Channel 3 Trigger Output Select"]
    #[inline]
    pub fn tos3(&mut self) -> _TOS3W {
        _TOS3W { w: self }
    }
    #[doc = "Bit 20 - Channel 4 Trigger Output Select"]
    #[inline]
    pub fn tos4(&mut self) -> _TOS4W {
        _TOS4W { w: self }
    }
    #[doc = "Bit 21 - Channel 5 Trigger Output Select"]
    #[inline]
    pub fn tos5(&mut self) -> _TOS5W {
        _TOS5W { w: self }
    }
    #[doc = "Bit 22 - Channel 6 Trigger Output Select"]
    #[inline]
    pub fn tos6(&mut self) -> _TOS6W {
        _TOS6W { w: self }
    }
    #[doc = "Bit 23 - Channel 7 Trigger Output Select"]
    #[inline]
    pub fn tos7(&mut self) -> _TOS7W {
        _TOS7W { w: self }
    }
    #[doc = "Bit 24 - Channel 8 Trigger Output Select"]
    #[inline]
    pub fn tos8(&mut self) -> _TOS8W {
        _TOS8W { w: self }
    }
}
