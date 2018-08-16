#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHSTRCON {
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
#[doc = "Possible values of the field `CH0S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0SR {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH0SR {
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
            CH0SR::VALUE1 => false,
            CH0SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0SR {
        match value {
            false => CH0SR::VALUE1,
            true => CH0SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH0SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH0SR::VALUE2
    }
}
#[doc = "Possible values of the field `CH1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1SR {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH1SR {
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
            CH1SR::VALUE1 => false,
            CH1SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1SR {
        match value {
            false => CH1SR::VALUE1,
            true => CH1SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH1SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH1SR::VALUE2
    }
}
#[doc = "Possible values of the field `CH2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2SR {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH2SR {
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
            CH2SR::VALUE1 => false,
            CH2SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2SR {
        match value {
            false => CH2SR::VALUE1,
            true => CH2SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH2SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH2SR::VALUE2
    }
}
#[doc = "Possible values of the field `CH3S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3SR {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH3SR {
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
            CH3SR::VALUE1 => false,
            CH3SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3SR {
        match value {
            false => CH3SR::VALUE1,
            true => CH3SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH3SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH3SR::VALUE2
    }
}
#[doc = "Possible values of the field `CH4S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4SR {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH4SR {
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
            CH4SR::VALUE1 => false,
            CH4SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH4SR {
        match value {
            false => CH4SR::VALUE1,
            true => CH4SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH4SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH4SR::VALUE2
    }
}
#[doc = "Possible values of the field `CH5S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5SR {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH5SR {
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
            CH5SR::VALUE1 => false,
            CH5SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH5SR {
        match value {
            false => CH5SR::VALUE1,
            true => CH5SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH5SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH5SR::VALUE2
    }
}
#[doc = "Possible values of the field `CH6S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6SR {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH6SR {
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
            CH6SR::VALUE1 => false,
            CH6SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH6SR {
        match value {
            false => CH6SR::VALUE1,
            true => CH6SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH6SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH6SR::VALUE2
    }
}
#[doc = "Possible values of the field `CH7S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7SR {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH7SR {
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
            CH7SR::VALUE1 => false,
            CH7SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH7SR {
        match value {
            false => CH7SR::VALUE1,
            true => CH7SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH7SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH7SR::VALUE2
    }
}
#[doc = "Possible values of the field `CH8S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8SR {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH8SR {
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
            CH8SR::VALUE1 => false,
            CH8SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH8SR {
        match value {
            false => CH8SR::VALUE1,
            true => CH8SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH8SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH8SR::VALUE2
    }
}
#[doc = "Values that can be written to the field `CH0S`"]
pub enum CH0SW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH0SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0SW::VALUE1 => false,
            CH0SW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0SW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH0SW::VALUE1)
    }
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH0SW::VALUE2)
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
#[doc = "Values that can be written to the field `CH1S`"]
pub enum CH1SW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH1SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1SW::VALUE1 => false,
            CH1SW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1SW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH1SW::VALUE1)
    }
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH1SW::VALUE2)
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
#[doc = "Values that can be written to the field `CH2S`"]
pub enum CH2SW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH2SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2SW::VALUE1 => false,
            CH2SW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2SW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH2SW::VALUE1)
    }
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH2SW::VALUE2)
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
#[doc = "Values that can be written to the field `CH3S`"]
pub enum CH3SW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH3SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3SW::VALUE1 => false,
            CH3SW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3SW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH3SW::VALUE1)
    }
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH3SW::VALUE2)
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
#[doc = "Values that can be written to the field `CH4S`"]
pub enum CH4SW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH4SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4SW::VALUE1 => false,
            CH4SW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH4SW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH4SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH4SW::VALUE1)
    }
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH4SW::VALUE2)
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
#[doc = "Values that can be written to the field `CH5S`"]
pub enum CH5SW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH5SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5SW::VALUE1 => false,
            CH5SW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH5SW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH5SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH5SW::VALUE1)
    }
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH5SW::VALUE2)
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
#[doc = "Values that can be written to the field `CH6S`"]
pub enum CH6SW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH6SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6SW::VALUE1 => false,
            CH6SW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH6SW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH6SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH6SW::VALUE1)
    }
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH6SW::VALUE2)
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
#[doc = "Values that can be written to the field `CH7S`"]
pub enum CH7SW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH7SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7SW::VALUE1 => false,
            CH7SW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH7SW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH7SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH7SW::VALUE1)
    }
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH7SW::VALUE2)
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
#[doc = "Values that can be written to the field `CH8S`"]
pub enum CH8SW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    VALUE2,
}
impl CH8SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH8SW::VALUE1 => false,
            CH8SW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH8SW<'a> {
    w: &'a mut W,
}
impl<'a> _CH8SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH8SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH8SW::VALUE1)
    }
    #[doc = "Initiate channel y target intensity shadow transfer. The linear walk will start and channel y intensity will start to change towards the target. Cleared by hardware when the linear walk is complete and the target has been reached."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH8SW::VALUE2)
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
#[doc = "Values that can be written to the field `CH0A`"]
pub enum CH0AW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    VALUE2,
}
impl CH0AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0AW::VALUE1 => false,
            CH0AW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0AW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0AW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH0AW::VALUE1)
    }
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH0AW::VALUE2)
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
#[doc = "Values that can be written to the field `CH1A`"]
pub enum CH1AW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    VALUE2,
}
impl CH1AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1AW::VALUE1 => false,
            CH1AW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1AW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1AW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH1AW::VALUE1)
    }
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH1AW::VALUE2)
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
#[doc = "Values that can be written to the field `CH2A`"]
pub enum CH2AW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    VALUE2,
}
impl CH2AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2AW::VALUE1 => false,
            CH2AW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2AW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2AW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH2AW::VALUE1)
    }
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH2AW::VALUE2)
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
#[doc = "Values that can be written to the field `CH3A`"]
pub enum CH3AW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    VALUE2,
}
impl CH3AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3AW::VALUE1 => false,
            CH3AW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3AW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3AW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH3AW::VALUE1)
    }
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH3AW::VALUE2)
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
#[doc = "Values that can be written to the field `CH4A`"]
pub enum CH4AW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    VALUE2,
}
impl CH4AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4AW::VALUE1 => false,
            CH4AW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH4AW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH4AW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH4AW::VALUE1)
    }
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH4AW::VALUE2)
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
#[doc = "Values that can be written to the field `CH5A`"]
pub enum CH5AW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    VALUE2,
}
impl CH5AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5AW::VALUE1 => false,
            CH5AW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH5AW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH5AW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH5AW::VALUE1)
    }
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH5AW::VALUE2)
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
#[doc = "Values that can be written to the field `CH6A`"]
pub enum CH6AW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    VALUE2,
}
impl CH6AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6AW::VALUE1 => false,
            CH6AW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH6AW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH6AW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH6AW::VALUE1)
    }
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH6AW::VALUE2)
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
#[doc = "Values that can be written to the field `CH7A`"]
pub enum CH7AW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    VALUE2,
}
impl CH7AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7AW::VALUE1 => false,
            CH7AW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH7AW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH7AW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH7AW::VALUE1)
    }
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH7AW::VALUE2)
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
#[doc = "Values that can be written to the field `CH8A`"]
pub enum CH8AW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    VALUE2,
}
impl CH8AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH8AW::VALUE1 => false,
            CH8AW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH8AW<'a> {
    w: &'a mut W,
}
impl<'a> _CH8AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH8AW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH8AW::VALUE1)
    }
    #[doc = "Abort linear walk; CHyS is cleared, channel y intensity stops changing"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH8AW::VALUE2)
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
    #[doc = "Bit 0 - Channel 0 Shadow Transfer"]
    #[inline]
    pub fn ch0s(&self) -> CH0SR {
        CH0SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel 1 Shadow Transfer"]
    #[inline]
    pub fn ch1s(&self) -> CH1SR {
        CH1SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 2 Shadow Transfer"]
    #[inline]
    pub fn ch2s(&self) -> CH2SR {
        CH2SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 3 Shadow Transfer"]
    #[inline]
    pub fn ch3s(&self) -> CH3SR {
        CH3SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel 4 Shadow Transfer"]
    #[inline]
    pub fn ch4s(&self) -> CH4SR {
        CH4SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel 5 Shadow Transfer"]
    #[inline]
    pub fn ch5s(&self) -> CH5SR {
        CH5SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channel 6 Shadow Transfer"]
    #[inline]
    pub fn ch6s(&self) -> CH6SR {
        CH6SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel 7 Shadow Transfer"]
    #[inline]
    pub fn ch7s(&self) -> CH7SR {
        CH7SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Channel 8 Shadow Transfer"]
    #[inline]
    pub fn ch8s(&self) -> CH8SR {
        CH8SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Channel 0 Shadow Transfer"]
    #[inline]
    pub fn ch0s(&mut self) -> _CH0SW {
        _CH0SW { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Shadow Transfer"]
    #[inline]
    pub fn ch1s(&mut self) -> _CH1SW {
        _CH1SW { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Shadow Transfer"]
    #[inline]
    pub fn ch2s(&mut self) -> _CH2SW {
        _CH2SW { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Shadow Transfer"]
    #[inline]
    pub fn ch3s(&mut self) -> _CH3SW {
        _CH3SW { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Shadow Transfer"]
    #[inline]
    pub fn ch4s(&mut self) -> _CH4SW {
        _CH4SW { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Shadow Transfer"]
    #[inline]
    pub fn ch5s(&mut self) -> _CH5SW {
        _CH5SW { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Shadow Transfer"]
    #[inline]
    pub fn ch6s(&mut self) -> _CH6SW {
        _CH6SW { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Shadow Transfer"]
    #[inline]
    pub fn ch7s(&mut self) -> _CH7SW {
        _CH7SW { w: self }
    }
    #[doc = "Bit 8 - Channel 8 Shadow Transfer"]
    #[inline]
    pub fn ch8s(&mut self) -> _CH8SW {
        _CH8SW { w: self }
    }
    #[doc = "Bit 16 - Channel 0 Linear Walk Abort"]
    #[inline]
    pub fn ch0a(&mut self) -> _CH0AW {
        _CH0AW { w: self }
    }
    #[doc = "Bit 17 - Channel 1 Linear Walk Abort"]
    #[inline]
    pub fn ch1a(&mut self) -> _CH1AW {
        _CH1AW { w: self }
    }
    #[doc = "Bit 18 - Channel 2 Linear Walk Abort"]
    #[inline]
    pub fn ch2a(&mut self) -> _CH2AW {
        _CH2AW { w: self }
    }
    #[doc = "Bit 19 - Channel 3 Linear Walk Abort"]
    #[inline]
    pub fn ch3a(&mut self) -> _CH3AW {
        _CH3AW { w: self }
    }
    #[doc = "Bit 20 - Channel 4 Linear Walk Abort"]
    #[inline]
    pub fn ch4a(&mut self) -> _CH4AW {
        _CH4AW { w: self }
    }
    #[doc = "Bit 21 - Channel 5 Linear Walk Abort"]
    #[inline]
    pub fn ch5a(&mut self) -> _CH5AW {
        _CH5AW { w: self }
    }
    #[doc = "Bit 22 - Channel 6 Linear Walk Abort"]
    #[inline]
    pub fn ch6a(&mut self) -> _CH6AW {
        _CH6AW { w: self }
    }
    #[doc = "Bit 23 - Channel 7 Linear Walk Abort"]
    #[inline]
    pub fn ch7a(&mut self) -> _CH7AW {
        _CH7AW { w: self }
    }
    #[doc = "Bit 24 - Channel 8 Linear Walk Abort"]
    #[inline]
    pub fn ch8a(&mut self) -> _CH8AW {
        _CH8AW { w: self }
    }
}
