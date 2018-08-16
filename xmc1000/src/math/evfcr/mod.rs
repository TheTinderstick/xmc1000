#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVFCR {
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
#[doc = "Values that can be written to the field `DIVEOCC`"]
pub enum DIVEOCCW {
    #[doc = "No effect."]
    VALUE1,
    #[doc = "Clears the Divider end of calculation event flag in EVFR register."]
    VALUE2,
}
impl DIVEOCCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIVEOCCW::VALUE1 => false,
            DIVEOCCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVEOCCW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVEOCCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVEOCCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVEOCCW::VALUE1)
    }
    #[doc = "Clears the Divider end of calculation event flag in EVFR register."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVEOCCW::VALUE2)
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
#[doc = "Values that can be written to the field `DIVERRC`"]
pub enum DIVERRCW {
    #[doc = "No effect."]
    VALUE1,
    #[doc = "Clears the Divider error event flag in EVFR register."]
    VALUE2,
}
impl DIVERRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIVERRCW::VALUE1 => false,
            DIVERRCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVERRCW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVERRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVERRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVERRCW::VALUE1)
    }
    #[doc = "Clears the Divider error event flag in EVFR register."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVERRCW::VALUE2)
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
#[doc = "Values that can be written to the field `CDEOCC`"]
pub enum CDEOCCW {
    #[doc = "No effect."]
    VALUE1,
    #[doc = "Clears the CORDIC end of calculation event flag in EVFR register."]
    VALUE2,
}
impl CDEOCCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDEOCCW::VALUE1 => false,
            CDEOCCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDEOCCW<'a> {
    w: &'a mut W,
}
impl<'a> _CDEOCCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDEOCCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDEOCCW::VALUE1)
    }
    #[doc = "Clears the CORDIC end of calculation event flag in EVFR register."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDEOCCW::VALUE2)
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
#[doc = "Values that can be written to the field `CDERRC`"]
pub enum CDERRCW {
    #[doc = "No effect."]
    VALUE1,
    #[doc = "Clears the CORDIC error event flag in EVFR register."]
    VALUE2,
}
impl CDERRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDERRCW::VALUE1 => false,
            CDERRCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDERRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CDERRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDERRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDERRCW::VALUE1)
    }
    #[doc = "Clears the CORDIC error event flag in EVFR register."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDERRCW::VALUE2)
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
    #[doc = "Bit 0 - Divider End of Calculation Event Flag Clear"]
    #[inline]
    pub fn diveocc(&mut self) -> _DIVEOCCW {
        _DIVEOCCW { w: self }
    }
    #[doc = "Bit 1 - Divider Error Event Flag Clear"]
    #[inline]
    pub fn diverrc(&mut self) -> _DIVERRCW {
        _DIVERRCW { w: self }
    }
    #[doc = "Bit 2 - CORDIC End of Calculation Event Flag Clear"]
    #[inline]
    pub fn cdeocc(&mut self) -> _CDEOCCW {
        _CDEOCCW { w: self }
    }
    #[doc = "Bit 3 - CORDIC Error Event Flag Clear"]
    #[inline]
    pub fn cderrc(&mut self) -> _CDERRCW {
        _CDERRCW { w: self }
    }
}
