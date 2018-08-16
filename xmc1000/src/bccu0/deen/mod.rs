#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEEN {
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
#[doc = "Possible values of the field `EDE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDE0R {
    #[doc = "Dimming Engine is disabled; the output dimming level (DLz.DLEV) is reset to 0 when the dimming engine gets disabled"]
    VALUE1,
    #[doc = "Dimming Engine is enabled"]
    VALUE2,
}
impl EDE0R {
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
            EDE0R::VALUE1 => false,
            EDE0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDE0R {
        match value {
            false => EDE0R::VALUE1,
            true => EDE0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EDE0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EDE0R::VALUE2
    }
}
#[doc = "Possible values of the field `EDE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDE1R {
    #[doc = "Dimming Engine is disabled; the output dimming level (DLz.DLEV) is reset to 0 when the dimming engine gets disabled"]
    VALUE1,
    #[doc = "Dimming Engine is enabled"]
    VALUE2,
}
impl EDE1R {
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
            EDE1R::VALUE1 => false,
            EDE1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDE1R {
        match value {
            false => EDE1R::VALUE1,
            true => EDE1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EDE1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EDE1R::VALUE2
    }
}
#[doc = "Possible values of the field `EDE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDE2R {
    #[doc = "Dimming Engine is disabled; the output dimming level (DLz.DLEV) is reset to 0 when the dimming engine gets disabled"]
    VALUE1,
    #[doc = "Dimming Engine is enabled"]
    VALUE2,
}
impl EDE2R {
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
            EDE2R::VALUE1 => false,
            EDE2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDE2R {
        match value {
            false => EDE2R::VALUE1,
            true => EDE2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EDE2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EDE2R::VALUE2
    }
}
#[doc = "Values that can be written to the field `EDE0`"]
pub enum EDE0W {
    #[doc = "Dimming Engine is disabled; the output dimming level (DLz.DLEV) is reset to 0 when the dimming engine gets disabled"]
    VALUE1,
    #[doc = "Dimming Engine is enabled"]
    VALUE2,
}
impl EDE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDE0W::VALUE1 => false,
            EDE0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDE0W<'a> {
    w: &'a mut W,
}
impl<'a> _EDE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dimming Engine is disabled; the output dimming level (DLz.DLEV) is reset to 0 when the dimming engine gets disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EDE0W::VALUE1)
    }
    #[doc = "Dimming Engine is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EDE0W::VALUE2)
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
#[doc = "Values that can be written to the field `EDE1`"]
pub enum EDE1W {
    #[doc = "Dimming Engine is disabled; the output dimming level (DLz.DLEV) is reset to 0 when the dimming engine gets disabled"]
    VALUE1,
    #[doc = "Dimming Engine is enabled"]
    VALUE2,
}
impl EDE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDE1W::VALUE1 => false,
            EDE1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDE1W<'a> {
    w: &'a mut W,
}
impl<'a> _EDE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dimming Engine is disabled; the output dimming level (DLz.DLEV) is reset to 0 when the dimming engine gets disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EDE1W::VALUE1)
    }
    #[doc = "Dimming Engine is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EDE1W::VALUE2)
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
#[doc = "Values that can be written to the field `EDE2`"]
pub enum EDE2W {
    #[doc = "Dimming Engine is disabled; the output dimming level (DLz.DLEV) is reset to 0 when the dimming engine gets disabled"]
    VALUE1,
    #[doc = "Dimming Engine is enabled"]
    VALUE2,
}
impl EDE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDE2W::VALUE1 => false,
            EDE2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDE2W<'a> {
    w: &'a mut W,
}
impl<'a> _EDE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDE2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dimming Engine is disabled; the output dimming level (DLz.DLEV) is reset to 0 when the dimming engine gets disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EDE2W::VALUE1)
    }
    #[doc = "Dimming Engine is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EDE2W::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Dimming Engine 0 Enable"]
    #[inline]
    pub fn ede0(&self) -> EDE0R {
        EDE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Dimming Engine 1 Enable"]
    #[inline]
    pub fn ede1(&self) -> EDE1R {
        EDE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Dimming Engine 2 Enable"]
    #[inline]
    pub fn ede2(&self) -> EDE2R {
        EDE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Dimming Engine 0 Enable"]
    #[inline]
    pub fn ede0(&mut self) -> _EDE0W {
        _EDE0W { w: self }
    }
    #[doc = "Bit 1 - Dimming Engine 1 Enable"]
    #[inline]
    pub fn ede1(&mut self) -> _EDE1W {
        _EDE1W { w: self }
    }
    #[doc = "Bit 2 - Dimming Engine 2 Enable"]
    #[inline]
    pub fn ede2(&mut self) -> _EDE2W {
        _EDE2W { w: self }
    }
}
