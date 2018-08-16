#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHOCON {
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
#[doc = "Possible values of the field `CH0OP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0OPR {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH0OPR {
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
            CH0OPR::VALUE1 => false,
            CH0OPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0OPR {
        match value {
            false => CH0OPR::VALUE1,
            true => CH0OPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH0OPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH0OPR::VALUE2
    }
}
#[doc = "Possible values of the field `CH1OP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1OPR {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH1OPR {
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
            CH1OPR::VALUE1 => false,
            CH1OPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1OPR {
        match value {
            false => CH1OPR::VALUE1,
            true => CH1OPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH1OPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH1OPR::VALUE2
    }
}
#[doc = "Possible values of the field `CH2OP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2OPR {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH2OPR {
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
            CH2OPR::VALUE1 => false,
            CH2OPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2OPR {
        match value {
            false => CH2OPR::VALUE1,
            true => CH2OPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH2OPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH2OPR::VALUE2
    }
}
#[doc = "Possible values of the field `CH3OP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3OPR {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH3OPR {
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
            CH3OPR::VALUE1 => false,
            CH3OPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3OPR {
        match value {
            false => CH3OPR::VALUE1,
            true => CH3OPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH3OPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH3OPR::VALUE2
    }
}
#[doc = "Possible values of the field `CH4OP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4OPR {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH4OPR {
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
            CH4OPR::VALUE1 => false,
            CH4OPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH4OPR {
        match value {
            false => CH4OPR::VALUE1,
            true => CH4OPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH4OPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH4OPR::VALUE2
    }
}
#[doc = "Possible values of the field `CH5OP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5OPR {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH5OPR {
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
            CH5OPR::VALUE1 => false,
            CH5OPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH5OPR {
        match value {
            false => CH5OPR::VALUE1,
            true => CH5OPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH5OPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH5OPR::VALUE2
    }
}
#[doc = "Possible values of the field `CH6OP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6OPR {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH6OPR {
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
            CH6OPR::VALUE1 => false,
            CH6OPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH6OPR {
        match value {
            false => CH6OPR::VALUE1,
            true => CH6OPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH6OPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH6OPR::VALUE2
    }
}
#[doc = "Possible values of the field `CH7OP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7OPR {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH7OPR {
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
            CH7OPR::VALUE1 => false,
            CH7OPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH7OPR {
        match value {
            false => CH7OPR::VALUE1,
            true => CH7OPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH7OPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH7OPR::VALUE2
    }
}
#[doc = "Possible values of the field `CH8OP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8OPR {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH8OPR {
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
            CH8OPR::VALUE1 => false,
            CH8OPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH8OPR {
        match value {
            false => CH8OPR::VALUE1,
            true => CH8OPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH8OPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH8OPR::VALUE2
    }
}
#[doc = "Possible values of the field `CH0TPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0TPER {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH0TPER {
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
            CH0TPER::VALUE1 => false,
            CH0TPER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0TPER {
        match value {
            false => CH0TPER::VALUE1,
            true => CH0TPER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH0TPER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH0TPER::VALUE2
    }
}
#[doc = "Possible values of the field `CH1TPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1TPER {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH1TPER {
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
            CH1TPER::VALUE1 => false,
            CH1TPER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1TPER {
        match value {
            false => CH1TPER::VALUE1,
            true => CH1TPER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH1TPER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH1TPER::VALUE2
    }
}
#[doc = "Possible values of the field `CH2TPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2TPER {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH2TPER {
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
            CH2TPER::VALUE1 => false,
            CH2TPER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2TPER {
        match value {
            false => CH2TPER::VALUE1,
            true => CH2TPER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH2TPER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH2TPER::VALUE2
    }
}
#[doc = "Possible values of the field `CH3TPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3TPER {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH3TPER {
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
            CH3TPER::VALUE1 => false,
            CH3TPER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3TPER {
        match value {
            false => CH3TPER::VALUE1,
            true => CH3TPER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH3TPER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH3TPER::VALUE2
    }
}
#[doc = "Possible values of the field `CH4TPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4TPER {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH4TPER {
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
            CH4TPER::VALUE1 => false,
            CH4TPER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH4TPER {
        match value {
            false => CH4TPER::VALUE1,
            true => CH4TPER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH4TPER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH4TPER::VALUE2
    }
}
#[doc = "Possible values of the field `CH5TPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5TPER {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH5TPER {
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
            CH5TPER::VALUE1 => false,
            CH5TPER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH5TPER {
        match value {
            false => CH5TPER::VALUE1,
            true => CH5TPER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH5TPER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH5TPER::VALUE2
    }
}
#[doc = "Possible values of the field `CH6TPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6TPER {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH6TPER {
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
            CH6TPER::VALUE1 => false,
            CH6TPER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH6TPER {
        match value {
            false => CH6TPER::VALUE1,
            true => CH6TPER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH6TPER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH6TPER::VALUE2
    }
}
#[doc = "Possible values of the field `CH7TPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7TPER {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH7TPER {
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
            CH7TPER::VALUE1 => false,
            CH7TPER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH7TPER {
        match value {
            false => CH7TPER::VALUE1,
            true => CH7TPER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH7TPER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH7TPER::VALUE2
    }
}
#[doc = "Possible values of the field `CH8TPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8TPER {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH8TPER {
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
            CH8TPER::VALUE1 => false,
            CH8TPER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH8TPER {
        match value {
            false => CH8TPER::VALUE1,
            true => CH8TPER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH8TPER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH8TPER::VALUE2
    }
}
#[doc = "Values that can be written to the field `CH0OP`"]
pub enum CH0OPW {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH0OPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0OPW::VALUE1 => false,
            CH0OPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0OPW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0OPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0OPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active high"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH0OPW::VALUE1)
    }
    #[doc = "Active low"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH0OPW::VALUE2)
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
#[doc = "Values that can be written to the field `CH1OP`"]
pub enum CH1OPW {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH1OPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1OPW::VALUE1 => false,
            CH1OPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1OPW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1OPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1OPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active high"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH1OPW::VALUE1)
    }
    #[doc = "Active low"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH1OPW::VALUE2)
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
#[doc = "Values that can be written to the field `CH2OP`"]
pub enum CH2OPW {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH2OPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2OPW::VALUE1 => false,
            CH2OPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2OPW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2OPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2OPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active high"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH2OPW::VALUE1)
    }
    #[doc = "Active low"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH2OPW::VALUE2)
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
#[doc = "Values that can be written to the field `CH3OP`"]
pub enum CH3OPW {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH3OPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3OPW::VALUE1 => false,
            CH3OPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3OPW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3OPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3OPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active high"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH3OPW::VALUE1)
    }
    #[doc = "Active low"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH3OPW::VALUE2)
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
#[doc = "Values that can be written to the field `CH4OP`"]
pub enum CH4OPW {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH4OPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4OPW::VALUE1 => false,
            CH4OPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH4OPW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4OPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH4OPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active high"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH4OPW::VALUE1)
    }
    #[doc = "Active low"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH4OPW::VALUE2)
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
#[doc = "Values that can be written to the field `CH5OP`"]
pub enum CH5OPW {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH5OPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5OPW::VALUE1 => false,
            CH5OPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH5OPW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5OPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH5OPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active high"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH5OPW::VALUE1)
    }
    #[doc = "Active low"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH5OPW::VALUE2)
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
#[doc = "Values that can be written to the field `CH6OP`"]
pub enum CH6OPW {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH6OPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6OPW::VALUE1 => false,
            CH6OPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH6OPW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6OPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH6OPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active high"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH6OPW::VALUE1)
    }
    #[doc = "Active low"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH6OPW::VALUE2)
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
#[doc = "Values that can be written to the field `CH7OP`"]
pub enum CH7OPW {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH7OPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7OPW::VALUE1 => false,
            CH7OPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH7OPW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7OPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH7OPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active high"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH7OPW::VALUE1)
    }
    #[doc = "Active low"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH7OPW::VALUE2)
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
#[doc = "Values that can be written to the field `CH8OP`"]
pub enum CH8OPW {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl CH8OPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH8OPW::VALUE1 => false,
            CH8OPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH8OPW<'a> {
    w: &'a mut W,
}
impl<'a> _CH8OPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH8OPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active high"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH8OPW::VALUE1)
    }
    #[doc = "Active low"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH8OPW::VALUE2)
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
#[doc = "Values that can be written to the field `CH0TPE`"]
pub enum CH0TPEW {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH0TPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0TPEW::VALUE1 => false,
            CH0TPEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0TPEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0TPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0TPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap function on channel is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH0TPEW::VALUE1)
    }
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH0TPEW::VALUE2)
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
#[doc = "Values that can be written to the field `CH1TPE`"]
pub enum CH1TPEW {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH1TPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1TPEW::VALUE1 => false,
            CH1TPEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1TPEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1TPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1TPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap function on channel is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH1TPEW::VALUE1)
    }
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH1TPEW::VALUE2)
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
#[doc = "Values that can be written to the field `CH2TPE`"]
pub enum CH2TPEW {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH2TPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2TPEW::VALUE1 => false,
            CH2TPEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2TPEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2TPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2TPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap function on channel is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH2TPEW::VALUE1)
    }
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH2TPEW::VALUE2)
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
#[doc = "Values that can be written to the field `CH3TPE`"]
pub enum CH3TPEW {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH3TPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3TPEW::VALUE1 => false,
            CH3TPEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3TPEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3TPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3TPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap function on channel is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH3TPEW::VALUE1)
    }
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH3TPEW::VALUE2)
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
#[doc = "Values that can be written to the field `CH4TPE`"]
pub enum CH4TPEW {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH4TPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4TPEW::VALUE1 => false,
            CH4TPEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH4TPEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4TPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH4TPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap function on channel is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH4TPEW::VALUE1)
    }
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH4TPEW::VALUE2)
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
#[doc = "Values that can be written to the field `CH5TPE`"]
pub enum CH5TPEW {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH5TPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5TPEW::VALUE1 => false,
            CH5TPEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH5TPEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5TPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH5TPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap function on channel is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH5TPEW::VALUE1)
    }
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH5TPEW::VALUE2)
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
#[doc = "Values that can be written to the field `CH6TPE`"]
pub enum CH6TPEW {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH6TPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6TPEW::VALUE1 => false,
            CH6TPEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH6TPEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6TPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH6TPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap function on channel is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH6TPEW::VALUE1)
    }
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH6TPEW::VALUE2)
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
#[doc = "Values that can be written to the field `CH7TPE`"]
pub enum CH7TPEW {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH7TPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7TPEW::VALUE1 => false,
            CH7TPEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH7TPEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7TPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH7TPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap function on channel is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH7TPEW::VALUE1)
    }
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH7TPEW::VALUE2)
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
#[doc = "Values that can be written to the field `CH8TPE`"]
pub enum CH8TPEW {
    #[doc = "Trap function on channel is disabled"]
    VALUE1,
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    VALUE2,
}
impl CH8TPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH8TPEW::VALUE1 => false,
            CH8TPEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH8TPEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH8TPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH8TPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap function on channel is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH8TPEW::VALUE1)
    }
    #[doc = "Trap function on channel is enabled, the output goes to passive level when trap occurs"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH8TPEW::VALUE2)
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
    #[doc = "Bit 0 - Channel 0 Output Passive Level"]
    #[inline]
    pub fn ch0op(&self) -> CH0OPR {
        CH0OPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel 1 Output Passive Level"]
    #[inline]
    pub fn ch1op(&self) -> CH1OPR {
        CH1OPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 2 Output Passive Level"]
    #[inline]
    pub fn ch2op(&self) -> CH2OPR {
        CH2OPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 3 Output Passive Level"]
    #[inline]
    pub fn ch3op(&self) -> CH3OPR {
        CH3OPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel 4 Output Passive Level"]
    #[inline]
    pub fn ch4op(&self) -> CH4OPR {
        CH4OPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel 5 Output Passive Level"]
    #[inline]
    pub fn ch5op(&self) -> CH5OPR {
        CH5OPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channel 6 Output Passive Level"]
    #[inline]
    pub fn ch6op(&self) -> CH6OPR {
        CH6OPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel 7 Output Passive Level"]
    #[inline]
    pub fn ch7op(&self) -> CH7OPR {
        CH7OPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Channel 8 Output Passive Level"]
    #[inline]
    pub fn ch8op(&self) -> CH8OPR {
        CH8OPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Channel 0 Trap Enable"]
    #[inline]
    pub fn ch0tpe(&self) -> CH0TPER {
        CH0TPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Channel 1 Trap Enable"]
    #[inline]
    pub fn ch1tpe(&self) -> CH1TPER {
        CH1TPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Channel 2 Trap Enable"]
    #[inline]
    pub fn ch2tpe(&self) -> CH2TPER {
        CH2TPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Channel 3 Trap Enable"]
    #[inline]
    pub fn ch3tpe(&self) -> CH3TPER {
        CH3TPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Channel 4 Trap Enable"]
    #[inline]
    pub fn ch4tpe(&self) -> CH4TPER {
        CH4TPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Channel 5 Trap Enable"]
    #[inline]
    pub fn ch5tpe(&self) -> CH5TPER {
        CH5TPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Channel 6 Trap Enable"]
    #[inline]
    pub fn ch6tpe(&self) -> CH6TPER {
        CH6TPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Channel 7 Trap Enable"]
    #[inline]
    pub fn ch7tpe(&self) -> CH7TPER {
        CH7TPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Channel 8 Trap Enable"]
    #[inline]
    pub fn ch8tpe(&self) -> CH8TPER {
        CH8TPER::_from({
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
    #[doc = "Bit 0 - Channel 0 Output Passive Level"]
    #[inline]
    pub fn ch0op(&mut self) -> _CH0OPW {
        _CH0OPW { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Output Passive Level"]
    #[inline]
    pub fn ch1op(&mut self) -> _CH1OPW {
        _CH1OPW { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Output Passive Level"]
    #[inline]
    pub fn ch2op(&mut self) -> _CH2OPW {
        _CH2OPW { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Output Passive Level"]
    #[inline]
    pub fn ch3op(&mut self) -> _CH3OPW {
        _CH3OPW { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Output Passive Level"]
    #[inline]
    pub fn ch4op(&mut self) -> _CH4OPW {
        _CH4OPW { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Output Passive Level"]
    #[inline]
    pub fn ch5op(&mut self) -> _CH5OPW {
        _CH5OPW { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Output Passive Level"]
    #[inline]
    pub fn ch6op(&mut self) -> _CH6OPW {
        _CH6OPW { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Output Passive Level"]
    #[inline]
    pub fn ch7op(&mut self) -> _CH7OPW {
        _CH7OPW { w: self }
    }
    #[doc = "Bit 8 - Channel 8 Output Passive Level"]
    #[inline]
    pub fn ch8op(&mut self) -> _CH8OPW {
        _CH8OPW { w: self }
    }
    #[doc = "Bit 16 - Channel 0 Trap Enable"]
    #[inline]
    pub fn ch0tpe(&mut self) -> _CH0TPEW {
        _CH0TPEW { w: self }
    }
    #[doc = "Bit 17 - Channel 1 Trap Enable"]
    #[inline]
    pub fn ch1tpe(&mut self) -> _CH1TPEW {
        _CH1TPEW { w: self }
    }
    #[doc = "Bit 18 - Channel 2 Trap Enable"]
    #[inline]
    pub fn ch2tpe(&mut self) -> _CH2TPEW {
        _CH2TPEW { w: self }
    }
    #[doc = "Bit 19 - Channel 3 Trap Enable"]
    #[inline]
    pub fn ch3tpe(&mut self) -> _CH3TPEW {
        _CH3TPEW { w: self }
    }
    #[doc = "Bit 20 - Channel 4 Trap Enable"]
    #[inline]
    pub fn ch4tpe(&mut self) -> _CH4TPEW {
        _CH4TPEW { w: self }
    }
    #[doc = "Bit 21 - Channel 5 Trap Enable"]
    #[inline]
    pub fn ch5tpe(&mut self) -> _CH5TPEW {
        _CH5TPEW { w: self }
    }
    #[doc = "Bit 22 - Channel 6 Trap Enable"]
    #[inline]
    pub fn ch6tpe(&mut self) -> _CH6TPEW {
        _CH6TPEW { w: self }
    }
    #[doc = "Bit 23 - Channel 7 Trap Enable"]
    #[inline]
    pub fn ch7tpe(&mut self) -> _CH7TPEW {
        _CH7TPEW { w: self }
    }
    #[doc = "Bit 24 - Channel 8 Trap Enable"]
    #[inline]
    pub fn ch8tpe(&mut self) -> _CH8TPEW {
        _CH8TPEW { w: self }
    }
}
