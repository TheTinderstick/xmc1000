#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RSTSET {
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
}
#[doc = "Values that can be written to the field `LCKEN`"]
pub enum LCKENW {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "Enable reset when Lockup gets asserted"]
    VALUE2,
}
impl LCKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LCKENW::VALUE1 => false,
            LCKENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCKENW<'a> {
    w: &'a mut W,
}
impl<'a> _LCKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LCKENW::VALUE1)
    }
    #[doc = "Enable reset when Lockup gets asserted"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LCKENW::VALUE2)
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
    #[doc = "Bit 10 - Enable Lockup Reset"]
    #[inline]
    pub fn lcken(&mut self) -> _LCKENW {
        _LCKENW { w: self }
    }
}
