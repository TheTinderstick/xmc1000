#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHEN {
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
#[doc = "Possible values of the field `ECH0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECH0R {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH0R {
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
            ECH0R::VALUE1 => false,
            ECH0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECH0R {
        match value {
            false => ECH0R::VALUE1,
            true => ECH0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECH0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ECH0R::VALUE2
    }
}
#[doc = "Possible values of the field `ECH1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECH1R {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH1R {
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
            ECH1R::VALUE1 => false,
            ECH1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECH1R {
        match value {
            false => ECH1R::VALUE1,
            true => ECH1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECH1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ECH1R::VALUE2
    }
}
#[doc = "Possible values of the field `ECH2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECH2R {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH2R {
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
            ECH2R::VALUE1 => false,
            ECH2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECH2R {
        match value {
            false => ECH2R::VALUE1,
            true => ECH2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECH2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ECH2R::VALUE2
    }
}
#[doc = "Possible values of the field `ECH3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECH3R {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH3R {
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
            ECH3R::VALUE1 => false,
            ECH3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECH3R {
        match value {
            false => ECH3R::VALUE1,
            true => ECH3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECH3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ECH3R::VALUE2
    }
}
#[doc = "Possible values of the field `ECH4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECH4R {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH4R {
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
            ECH4R::VALUE1 => false,
            ECH4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECH4R {
        match value {
            false => ECH4R::VALUE1,
            true => ECH4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECH4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ECH4R::VALUE2
    }
}
#[doc = "Possible values of the field `ECH5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECH5R {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH5R {
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
            ECH5R::VALUE1 => false,
            ECH5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECH5R {
        match value {
            false => ECH5R::VALUE1,
            true => ECH5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECH5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ECH5R::VALUE2
    }
}
#[doc = "Possible values of the field `ECH6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECH6R {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH6R {
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
            ECH6R::VALUE1 => false,
            ECH6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECH6R {
        match value {
            false => ECH6R::VALUE1,
            true => ECH6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECH6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ECH6R::VALUE2
    }
}
#[doc = "Possible values of the field `ECH7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECH7R {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH7R {
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
            ECH7R::VALUE1 => false,
            ECH7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECH7R {
        match value {
            false => ECH7R::VALUE1,
            true => ECH7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECH7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ECH7R::VALUE2
    }
}
#[doc = "Possible values of the field `ECH8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECH8R {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH8R {
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
            ECH8R::VALUE1 => false,
            ECH8R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECH8R {
        match value {
            false => ECH8R::VALUE1,
            true => ECH8R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECH8R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ECH8R::VALUE2
    }
}
#[doc = "Values that can be written to the field `ECH0`"]
pub enum ECH0W {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECH0W::VALUE1 => false,
            ECH0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECH0W<'a> {
    w: &'a mut W,
}
impl<'a> _ECH0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECH0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECH0W::VALUE1)
    }
    #[doc = "Channel is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECH0W::VALUE2)
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
#[doc = "Values that can be written to the field `ECH1`"]
pub enum ECH1W {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECH1W::VALUE1 => false,
            ECH1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECH1W<'a> {
    w: &'a mut W,
}
impl<'a> _ECH1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECH1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECH1W::VALUE1)
    }
    #[doc = "Channel is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECH1W::VALUE2)
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
#[doc = "Values that can be written to the field `ECH2`"]
pub enum ECH2W {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECH2W::VALUE1 => false,
            ECH2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECH2W<'a> {
    w: &'a mut W,
}
impl<'a> _ECH2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECH2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECH2W::VALUE1)
    }
    #[doc = "Channel is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECH2W::VALUE2)
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
#[doc = "Values that can be written to the field `ECH3`"]
pub enum ECH3W {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECH3W::VALUE1 => false,
            ECH3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECH3W<'a> {
    w: &'a mut W,
}
impl<'a> _ECH3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECH3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECH3W::VALUE1)
    }
    #[doc = "Channel is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECH3W::VALUE2)
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
#[doc = "Values that can be written to the field `ECH4`"]
pub enum ECH4W {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECH4W::VALUE1 => false,
            ECH4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECH4W<'a> {
    w: &'a mut W,
}
impl<'a> _ECH4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECH4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECH4W::VALUE1)
    }
    #[doc = "Channel is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECH4W::VALUE2)
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
#[doc = "Values that can be written to the field `ECH5`"]
pub enum ECH5W {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECH5W::VALUE1 => false,
            ECH5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECH5W<'a> {
    w: &'a mut W,
}
impl<'a> _ECH5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECH5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECH5W::VALUE1)
    }
    #[doc = "Channel is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECH5W::VALUE2)
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
#[doc = "Values that can be written to the field `ECH6`"]
pub enum ECH6W {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECH6W::VALUE1 => false,
            ECH6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECH6W<'a> {
    w: &'a mut W,
}
impl<'a> _ECH6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECH6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECH6W::VALUE1)
    }
    #[doc = "Channel is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECH6W::VALUE2)
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
#[doc = "Values that can be written to the field `ECH7`"]
pub enum ECH7W {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECH7W::VALUE1 => false,
            ECH7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECH7W<'a> {
    w: &'a mut W,
}
impl<'a> _ECH7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECH7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECH7W::VALUE1)
    }
    #[doc = "Channel is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECH7W::VALUE2)
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
#[doc = "Values that can be written to the field `ECH8`"]
pub enum ECH8W {
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    VALUE1,
    #[doc = "Channel is enabled"]
    VALUE2,
}
impl ECH8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECH8W::VALUE1 => false,
            ECH8W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECH8W<'a> {
    w: &'a mut W,
}
impl<'a> _ECH8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECH8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel is disabled, the output level is passive; the Linear Walker and the Sigma-Delta Modulator are reset, the Packer FIFO is flushed; all internal logic and INTy are reset when the channel gets disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECH8W::VALUE1)
    }
    #[doc = "Channel is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECH8W::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Channel 0 Enable"]
    #[inline]
    pub fn ech0(&self) -> ECH0R {
        ECH0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel 1 Enable"]
    #[inline]
    pub fn ech1(&self) -> ECH1R {
        ECH1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 2 Enable"]
    #[inline]
    pub fn ech2(&self) -> ECH2R {
        ECH2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 3 Enable"]
    #[inline]
    pub fn ech3(&self) -> ECH3R {
        ECH3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel 4 Enable"]
    #[inline]
    pub fn ech4(&self) -> ECH4R {
        ECH4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel 5 Enable"]
    #[inline]
    pub fn ech5(&self) -> ECH5R {
        ECH5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channel 6 Enable"]
    #[inline]
    pub fn ech6(&self) -> ECH6R {
        ECH6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel 7 Enable"]
    #[inline]
    pub fn ech7(&self) -> ECH7R {
        ECH7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Channel 8 Enable"]
    #[inline]
    pub fn ech8(&self) -> ECH8R {
        ECH8R::_from({
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
    #[doc = "Bit 0 - Channel 0 Enable"]
    #[inline]
    pub fn ech0(&mut self) -> _ECH0W {
        _ECH0W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Enable"]
    #[inline]
    pub fn ech1(&mut self) -> _ECH1W {
        _ECH1W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Enable"]
    #[inline]
    pub fn ech2(&mut self) -> _ECH2W {
        _ECH2W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Enable"]
    #[inline]
    pub fn ech3(&mut self) -> _ECH3W {
        _ECH3W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Enable"]
    #[inline]
    pub fn ech4(&mut self) -> _ECH4W {
        _ECH4W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Enable"]
    #[inline]
    pub fn ech5(&mut self) -> _ECH5W {
        _ECH5W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Enable"]
    #[inline]
    pub fn ech6(&mut self) -> _ECH6W {
        _ECH6W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Enable"]
    #[inline]
    pub fn ech7(&mut self) -> _ECH7W {
        _ECH7W { w: self }
    }
    #[doc = "Bit 8 - Channel 8 Enable"]
    #[inline]
    pub fn ech8(&mut self) -> _ECH8W {
        _ECH8W { w: self }
    }
}
