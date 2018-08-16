#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ORCCTRL {
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
#[doc = "Possible values of the field `ENORC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENORC0R {
    #[doc = "Out of range comparator disabled."]
    VALUE1,
    #[doc = "Out of range comparator enabled."]
    VALUE2,
}
impl ENORC0R {
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
            ENORC0R::VALUE1 => false,
            ENORC0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENORC0R {
        match value {
            false => ENORC0R::VALUE1,
            true => ENORC0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENORC0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENORC0R::VALUE2
    }
}
#[doc = "Possible values of the field `ENORC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENORC1R {
    #[doc = "Out of range comparator disabled."]
    VALUE1,
    #[doc = "Out of range comparator enabled."]
    VALUE2,
}
impl ENORC1R {
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
            ENORC1R::VALUE1 => false,
            ENORC1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENORC1R {
        match value {
            false => ENORC1R::VALUE1,
            true => ENORC1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENORC1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENORC1R::VALUE2
    }
}
#[doc = "Possible values of the field `ENORC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENORC2R {
    #[doc = "Out of range comparator disabled."]
    VALUE1,
    #[doc = "Out of range comparator enabled."]
    VALUE2,
}
impl ENORC2R {
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
            ENORC2R::VALUE1 => false,
            ENORC2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENORC2R {
        match value {
            false => ENORC2R::VALUE1,
            true => ENORC2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENORC2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENORC2R::VALUE2
    }
}
#[doc = "Possible values of the field `ENORC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENORC3R {
    #[doc = "Out of range comparator disabled."]
    VALUE1,
    #[doc = "Out of range comparator enabled."]
    VALUE2,
}
impl ENORC3R {
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
            ENORC3R::VALUE1 => false,
            ENORC3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENORC3R {
        match value {
            false => ENORC3R::VALUE1,
            true => ENORC3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENORC3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENORC3R::VALUE2
    }
}
#[doc = "Possible values of the field `ENORC4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENORC4R {
    #[doc = "Out of range comparator disabled."]
    VALUE1,
    #[doc = "Out of range comparator enabled."]
    VALUE2,
}
impl ENORC4R {
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
            ENORC4R::VALUE1 => false,
            ENORC4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENORC4R {
        match value {
            false => ENORC4R::VALUE1,
            true => ENORC4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENORC4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENORC4R::VALUE2
    }
}
#[doc = "Possible values of the field `ENORC5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENORC5R {
    #[doc = "Out of range comparator disabled."]
    VALUE1,
    #[doc = "Out of range comparator enabled."]
    VALUE2,
}
impl ENORC5R {
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
            ENORC5R::VALUE1 => false,
            ENORC5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENORC5R {
        match value {
            false => ENORC5R::VALUE1,
            true => ENORC5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENORC5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENORC5R::VALUE2
    }
}
#[doc = "Possible values of the field `ENORC6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENORC6R {
    #[doc = "Out of range comparator disabled."]
    VALUE1,
    #[doc = "Out of range comparator enabled."]
    VALUE2,
}
impl ENORC6R {
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
            ENORC6R::VALUE1 => false,
            ENORC6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENORC6R {
        match value {
            false => ENORC6R::VALUE1,
            true => ENORC6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENORC6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENORC6R::VALUE2
    }
}
#[doc = "Possible values of the field `ENORC7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENORC7R {
    #[doc = "Out of range comparator disabled."]
    VALUE1,
    #[doc = "Out of range comparator enabled."]
    VALUE2,
}
impl ENORC7R {
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
            ENORC7R::VALUE1 => false,
            ENORC7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENORC7R {
        match value {
            false => ENORC7R::VALUE1,
            true => ENORC7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENORC7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENORC7R::VALUE2
    }
}
#[doc = "Possible values of the field `CNF0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF0R {
    #[doc = "Falling edge trigger out of range event register."]
    VALUE1,
    #[doc = "Rising edge trigger out of range event register."]
    VALUE2,
}
impl CNF0R {
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
            CNF0R::VALUE1 => false,
            CNF0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNF0R {
        match value {
            false => CNF0R::VALUE1,
            true => CNF0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CNF0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CNF0R::VALUE2
    }
}
#[doc = "Possible values of the field `CNF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF1R {
    #[doc = "Falling edge trigger out of range event register."]
    VALUE1,
    #[doc = "Rising edge trigger out of range event register."]
    VALUE2,
}
impl CNF1R {
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
            CNF1R::VALUE1 => false,
            CNF1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNF1R {
        match value {
            false => CNF1R::VALUE1,
            true => CNF1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CNF1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CNF1R::VALUE2
    }
}
#[doc = "Possible values of the field `CNF2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF2R {
    #[doc = "Falling edge trigger out of range event register."]
    VALUE1,
    #[doc = "Rising edge trigger out of range event register."]
    VALUE2,
}
impl CNF2R {
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
            CNF2R::VALUE1 => false,
            CNF2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNF2R {
        match value {
            false => CNF2R::VALUE1,
            true => CNF2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CNF2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CNF2R::VALUE2
    }
}
#[doc = "Possible values of the field `CNF3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF3R {
    #[doc = "Falling edge trigger out of range event register."]
    VALUE1,
    #[doc = "Rising edge trigger out of range event register."]
    VALUE2,
}
impl CNF3R {
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
            CNF3R::VALUE1 => false,
            CNF3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNF3R {
        match value {
            false => CNF3R::VALUE1,
            true => CNF3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CNF3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CNF3R::VALUE2
    }
}
#[doc = "Possible values of the field `CNF4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF4R {
    #[doc = "Falling edge trigger out of range event register."]
    VALUE1,
    #[doc = "Rising edge trigger out of range event register."]
    VALUE2,
}
impl CNF4R {
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
            CNF4R::VALUE1 => false,
            CNF4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNF4R {
        match value {
            false => CNF4R::VALUE1,
            true => CNF4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CNF4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CNF4R::VALUE2
    }
}
#[doc = "Possible values of the field `CNF5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF5R {
    #[doc = "Falling edge trigger out of range event register."]
    VALUE1,
    #[doc = "Rising edge trigger out of range event register."]
    VALUE2,
}
impl CNF5R {
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
            CNF5R::VALUE1 => false,
            CNF5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNF5R {
        match value {
            false => CNF5R::VALUE1,
            true => CNF5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CNF5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CNF5R::VALUE2
    }
}
#[doc = "Possible values of the field `CNF6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF6R {
    #[doc = "Falling edge trigger out of range event register."]
    VALUE1,
    #[doc = "Rising edge trigger out of range event register."]
    VALUE2,
}
impl CNF6R {
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
            CNF6R::VALUE1 => false,
            CNF6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNF6R {
        match value {
            false => CNF6R::VALUE1,
            true => CNF6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CNF6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CNF6R::VALUE2
    }
}
#[doc = "Possible values of the field `CNF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF7R {
    #[doc = "Falling edge trigger out of range event register."]
    VALUE1,
    #[doc = "Rising edge trigger out of range event register."]
    VALUE2,
}
impl CNF7R {
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
            CNF7R::VALUE1 => false,
            CNF7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNF7R {
        match value {
            false => CNF7R::VALUE1,
            true => CNF7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CNF7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CNF7R::VALUE2
    }
}
#[doc = "Values that can be written to the field `ENORC0`"]
pub enum ENORC0W {
    #[doc = "Out of range comparator disabled."]
    VALUE1,
    #[doc = "Out of range comparator enabled."]
    VALUE2,
}
impl ENORC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENORC0W::VALUE1 => false,
            ENORC0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENORC0W<'a> {
    w: &'a mut W,
}
impl<'a> _ENORC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENORC0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Out of range comparator disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENORC0W::VALUE1)
    }
    #[doc = "Out of range comparator enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENORC0W::VALUE2)
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
#[doc = "Values that can be written to the field `ENORC1`"]
pub enum ENORC1W {
    #[doc = "Out of range comparator disabled."]
    VALUE1,
    #[doc = "Out of range comparator enabled."]
    VALUE2,
}
impl ENORC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENORC1W::VALUE1 => false,
            ENORC1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENORC1W<'a> {
    w: &'a mut W,
}
impl<'a> _ENORC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENORC1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Out of range comparator disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENORC1W::VALUE1)
    }
    #[doc = "Out of range comparator enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENORC1W::VALUE2)
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
#[doc = "Values that can be written to the field `ENORC2`"]
pub enum ENORC2W {
    #[doc = "Out of range comparator disabled."]
    VALUE1,
    #[doc = "Out of range comparator enabled."]
    VALUE2,
}
impl ENORC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENORC2W::VALUE1 => false,
            ENORC2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENORC2W<'a> {
    w: &'a mut W,
}
impl<'a> _ENORC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENORC2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Out of range comparator disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENORC2W::VALUE1)
    }
    #[doc = "Out of range comparator enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENORC2W::VALUE2)
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
#[doc = "Values that can be written to the field `ENORC3`"]
pub enum ENORC3W {
    #[doc = "Out of range comparator disabled."]
    VALUE1,
    #[doc = "Out of range comparator enabled."]
    VALUE2,
}
impl ENORC3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENORC3W::VALUE1 => false,
            ENORC3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENORC3W<'a> {
    w: &'a mut W,
}
impl<'a> _ENORC3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENORC3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Out of range comparator disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENORC3W::VALUE1)
    }
    #[doc = "Out of range comparator enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENORC3W::VALUE2)
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
#[doc = "Values that can be written to the field `ENORC4`"]
pub enum ENORC4W {
    #[doc = "Out of range comparator disabled."]
    VALUE1,
    #[doc = "Out of range comparator enabled."]
    VALUE2,
}
impl ENORC4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENORC4W::VALUE1 => false,
            ENORC4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENORC4W<'a> {
    w: &'a mut W,
}
impl<'a> _ENORC4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENORC4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Out of range comparator disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENORC4W::VALUE1)
    }
    #[doc = "Out of range comparator enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENORC4W::VALUE2)
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
#[doc = "Values that can be written to the field `ENORC5`"]
pub enum ENORC5W {
    #[doc = "Out of range comparator disabled."]
    VALUE1,
    #[doc = "Out of range comparator enabled."]
    VALUE2,
}
impl ENORC5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENORC5W::VALUE1 => false,
            ENORC5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENORC5W<'a> {
    w: &'a mut W,
}
impl<'a> _ENORC5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENORC5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Out of range comparator disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENORC5W::VALUE1)
    }
    #[doc = "Out of range comparator enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENORC5W::VALUE2)
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
#[doc = "Values that can be written to the field `ENORC6`"]
pub enum ENORC6W {
    #[doc = "Out of range comparator disabled."]
    VALUE1,
    #[doc = "Out of range comparator enabled."]
    VALUE2,
}
impl ENORC6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENORC6W::VALUE1 => false,
            ENORC6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENORC6W<'a> {
    w: &'a mut W,
}
impl<'a> _ENORC6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENORC6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Out of range comparator disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENORC6W::VALUE1)
    }
    #[doc = "Out of range comparator enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENORC6W::VALUE2)
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
#[doc = "Values that can be written to the field `ENORC7`"]
pub enum ENORC7W {
    #[doc = "Out of range comparator disabled."]
    VALUE1,
    #[doc = "Out of range comparator enabled."]
    VALUE2,
}
impl ENORC7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENORC7W::VALUE1 => false,
            ENORC7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENORC7W<'a> {
    w: &'a mut W,
}
impl<'a> _ENORC7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENORC7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Out of range comparator disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENORC7W::VALUE1)
    }
    #[doc = "Out of range comparator enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENORC7W::VALUE2)
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
#[doc = "Values that can be written to the field `CNF0`"]
pub enum CNF0W {
    #[doc = "Falling edge trigger out of range event register."]
    VALUE1,
    #[doc = "Rising edge trigger out of range event register."]
    VALUE2,
}
impl CNF0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNF0W::VALUE1 => false,
            CNF0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNF0W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge trigger out of range event register."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CNF0W::VALUE1)
    }
    #[doc = "Rising edge trigger out of range event register."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CNF0W::VALUE2)
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
#[doc = "Values that can be written to the field `CNF1`"]
pub enum CNF1W {
    #[doc = "Falling edge trigger out of range event register."]
    VALUE1,
    #[doc = "Rising edge trigger out of range event register."]
    VALUE2,
}
impl CNF1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNF1W::VALUE1 => false,
            CNF1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNF1W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge trigger out of range event register."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CNF1W::VALUE1)
    }
    #[doc = "Rising edge trigger out of range event register."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CNF1W::VALUE2)
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
#[doc = "Values that can be written to the field `CNF2`"]
pub enum CNF2W {
    #[doc = "Falling edge trigger out of range event register."]
    VALUE1,
    #[doc = "Rising edge trigger out of range event register."]
    VALUE2,
}
impl CNF2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNF2W::VALUE1 => false,
            CNF2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNF2W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge trigger out of range event register."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CNF2W::VALUE1)
    }
    #[doc = "Rising edge trigger out of range event register."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CNF2W::VALUE2)
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
#[doc = "Values that can be written to the field `CNF3`"]
pub enum CNF3W {
    #[doc = "Falling edge trigger out of range event register."]
    VALUE1,
    #[doc = "Rising edge trigger out of range event register."]
    VALUE2,
}
impl CNF3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNF3W::VALUE1 => false,
            CNF3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNF3W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge trigger out of range event register."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CNF3W::VALUE1)
    }
    #[doc = "Rising edge trigger out of range event register."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CNF3W::VALUE2)
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
#[doc = "Values that can be written to the field `CNF4`"]
pub enum CNF4W {
    #[doc = "Falling edge trigger out of range event register."]
    VALUE1,
    #[doc = "Rising edge trigger out of range event register."]
    VALUE2,
}
impl CNF4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNF4W::VALUE1 => false,
            CNF4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNF4W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge trigger out of range event register."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CNF4W::VALUE1)
    }
    #[doc = "Rising edge trigger out of range event register."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CNF4W::VALUE2)
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
#[doc = "Values that can be written to the field `CNF5`"]
pub enum CNF5W {
    #[doc = "Falling edge trigger out of range event register."]
    VALUE1,
    #[doc = "Rising edge trigger out of range event register."]
    VALUE2,
}
impl CNF5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNF5W::VALUE1 => false,
            CNF5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNF5W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge trigger out of range event register."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CNF5W::VALUE1)
    }
    #[doc = "Rising edge trigger out of range event register."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CNF5W::VALUE2)
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
#[doc = "Values that can be written to the field `CNF6`"]
pub enum CNF6W {
    #[doc = "Falling edge trigger out of range event register."]
    VALUE1,
    #[doc = "Rising edge trigger out of range event register."]
    VALUE2,
}
impl CNF6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNF6W::VALUE1 => false,
            CNF6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNF6W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge trigger out of range event register."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CNF6W::VALUE1)
    }
    #[doc = "Rising edge trigger out of range event register."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CNF6W::VALUE2)
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
#[doc = "Values that can be written to the field `CNF7`"]
pub enum CNF7W {
    #[doc = "Falling edge trigger out of range event register."]
    VALUE1,
    #[doc = "Rising edge trigger out of range event register."]
    VALUE2,
}
impl CNF7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNF7W::VALUE1 => false,
            CNF7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNF7W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge trigger out of range event register."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CNF7W::VALUE1)
    }
    #[doc = "Rising edge trigger out of range event register."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CNF7W::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable Out of Range Comparator 0"]
    #[inline]
    pub fn enorc0(&self) -> ENORC0R {
        ENORC0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable Out of Range Comparator 1"]
    #[inline]
    pub fn enorc1(&self) -> ENORC1R {
        ENORC1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable Out of Range Comparator 2"]
    #[inline]
    pub fn enorc2(&self) -> ENORC2R {
        ENORC2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable Out of Range Comparator 3"]
    #[inline]
    pub fn enorc3(&self) -> ENORC3R {
        ENORC3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable Out of Range Comparator 4"]
    #[inline]
    pub fn enorc4(&self) -> ENORC4R {
        ENORC4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable Out of Range Comparator 5"]
    #[inline]
    pub fn enorc5(&self) -> ENORC5R {
        ENORC5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable Out of Range Comparator 6"]
    #[inline]
    pub fn enorc6(&self) -> ENORC6R {
        ENORC6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable Out of Range Comparator 7"]
    #[inline]
    pub fn enorc7(&self) -> ENORC7R {
        ENORC7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Out of Range Comparator Flag 0"]
    #[inline]
    pub fn cnf0(&self) -> CNF0R {
        CNF0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Out of Range Comparator Flag 1"]
    #[inline]
    pub fn cnf1(&self) -> CNF1R {
        CNF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Out of Range Comparator Flag 2"]
    #[inline]
    pub fn cnf2(&self) -> CNF2R {
        CNF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Out of Range Comparator Flag 3"]
    #[inline]
    pub fn cnf3(&self) -> CNF3R {
        CNF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Out of Range Comparator Flag 4"]
    #[inline]
    pub fn cnf4(&self) -> CNF4R {
        CNF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Out of Range Comparator Flag 5"]
    #[inline]
    pub fn cnf5(&self) -> CNF5R {
        CNF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Out of Range Comparator Flag 6"]
    #[inline]
    pub fn cnf6(&self) -> CNF6R {
        CNF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Out of Range Comparator Flag 7"]
    #[inline]
    pub fn cnf7(&self) -> CNF7R {
        CNF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
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
    #[doc = "Bit 0 - Enable Out of Range Comparator 0"]
    #[inline]
    pub fn enorc0(&mut self) -> _ENORC0W {
        _ENORC0W { w: self }
    }
    #[doc = "Bit 1 - Enable Out of Range Comparator 1"]
    #[inline]
    pub fn enorc1(&mut self) -> _ENORC1W {
        _ENORC1W { w: self }
    }
    #[doc = "Bit 2 - Enable Out of Range Comparator 2"]
    #[inline]
    pub fn enorc2(&mut self) -> _ENORC2W {
        _ENORC2W { w: self }
    }
    #[doc = "Bit 3 - Enable Out of Range Comparator 3"]
    #[inline]
    pub fn enorc3(&mut self) -> _ENORC3W {
        _ENORC3W { w: self }
    }
    #[doc = "Bit 4 - Enable Out of Range Comparator 4"]
    #[inline]
    pub fn enorc4(&mut self) -> _ENORC4W {
        _ENORC4W { w: self }
    }
    #[doc = "Bit 5 - Enable Out of Range Comparator 5"]
    #[inline]
    pub fn enorc5(&mut self) -> _ENORC5W {
        _ENORC5W { w: self }
    }
    #[doc = "Bit 6 - Enable Out of Range Comparator 6"]
    #[inline]
    pub fn enorc6(&mut self) -> _ENORC6W {
        _ENORC6W { w: self }
    }
    #[doc = "Bit 7 - Enable Out of Range Comparator 7"]
    #[inline]
    pub fn enorc7(&mut self) -> _ENORC7W {
        _ENORC7W { w: self }
    }
    #[doc = "Bit 16 - Out of Range Comparator Flag 0"]
    #[inline]
    pub fn cnf0(&mut self) -> _CNF0W {
        _CNF0W { w: self }
    }
    #[doc = "Bit 17 - Out of Range Comparator Flag 1"]
    #[inline]
    pub fn cnf1(&mut self) -> _CNF1W {
        _CNF1W { w: self }
    }
    #[doc = "Bit 18 - Out of Range Comparator Flag 2"]
    #[inline]
    pub fn cnf2(&mut self) -> _CNF2W {
        _CNF2W { w: self }
    }
    #[doc = "Bit 19 - Out of Range Comparator Flag 3"]
    #[inline]
    pub fn cnf3(&mut self) -> _CNF3W {
        _CNF3W { w: self }
    }
    #[doc = "Bit 20 - Out of Range Comparator Flag 4"]
    #[inline]
    pub fn cnf4(&mut self) -> _CNF4W {
        _CNF4W { w: self }
    }
    #[doc = "Bit 21 - Out of Range Comparator Flag 5"]
    #[inline]
    pub fn cnf5(&mut self) -> _CNF5W {
        _CNF5W { w: self }
    }
    #[doc = "Bit 22 - Out of Range Comparator Flag 6"]
    #[inline]
    pub fn cnf6(&mut self) -> _CNF6W {
        _CNF6W { w: self }
    }
    #[doc = "Bit 23 - Out of Range Comparator Flag 7"]
    #[inline]
    pub fn cnf7(&mut self) -> _CNF7W {
        _CNF7W { w: self }
    }
}
