#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVFSR {
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
#[doc = "Values that can be written to the field `DIVEOCS`"]
pub enum DIVEOCSW {
    #[doc = "No effect."]
    VALUE1,
    #[doc = "Sets the Divider end of calculation event flag in EVFR register. Interrupt will be generated if enabled in EVIER register."]
    VALUE2,
}
impl DIVEOCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIVEOCSW::VALUE1 => false,
            DIVEOCSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVEOCSW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVEOCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVEOCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVEOCSW::VALUE1)
    }
    #[doc = "Sets the Divider end of calculation event flag in EVFR register. Interrupt will be generated if enabled in EVIER register."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVEOCSW::VALUE2)
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
#[doc = "Values that can be written to the field `DIVERRS`"]
pub enum DIVERRSW {
    #[doc = "No effect."]
    VALUE1,
    #[doc = "Sets the Divider error event flag in EVFR register. Interrupt will be generated if enabled in EVIER register."]
    VALUE2,
}
impl DIVERRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIVERRSW::VALUE1 => false,
            DIVERRSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVERRSW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVERRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVERRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVERRSW::VALUE1)
    }
    #[doc = "Sets the Divider error event flag in EVFR register. Interrupt will be generated if enabled in EVIER register."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVERRSW::VALUE2)
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
#[doc = "Values that can be written to the field `CDEOCS`"]
pub enum CDEOCSW {
    #[doc = "No effect."]
    VALUE1,
    #[doc = "Sets the CORDIC end of calculation event flag in EVFR register. Interrupt will be generated if enabled in EVIER register."]
    VALUE2,
}
impl CDEOCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDEOCSW::VALUE1 => false,
            CDEOCSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDEOCSW<'a> {
    w: &'a mut W,
}
impl<'a> _CDEOCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDEOCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDEOCSW::VALUE1)
    }
    #[doc = "Sets the CORDIC end of calculation event flag in EVFR register. Interrupt will be generated if enabled in EVIER register."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDEOCSW::VALUE2)
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
#[doc = "Values that can be written to the field `CDERRS`"]
pub enum CDERRSW {
    #[doc = "No effect."]
    VALUE1,
    #[doc = "Sets the CORDIC error event flag in EVFR register. Interrupt will be generated if enabled in EVIER register."]
    VALUE2,
}
impl CDERRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDERRSW::VALUE1 => false,
            CDERRSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDERRSW<'a> {
    w: &'a mut W,
}
impl<'a> _CDERRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDERRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDERRSW::VALUE1)
    }
    #[doc = "Sets the CORDIC error event flag in EVFR register. Interrupt will be generated if enabled in EVIER register."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDERRSW::VALUE2)
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
    #[doc = "Bit 0 - Divider End of Calculation Event Flag Set"]
    #[inline]
    pub fn diveocs(&mut self) -> _DIVEOCSW {
        _DIVEOCSW { w: self }
    }
    #[doc = "Bit 1 - Divider Error Event Flag Set"]
    #[inline]
    pub fn diverrs(&mut self) -> _DIVERRSW {
        _DIVERRSW { w: self }
    }
    #[doc = "Bit 2 - CORDIC Event Flag Set"]
    #[inline]
    pub fn cdeocs(&mut self) -> _CDEOCSW {
        _CDEOCSW { w: self }
    }
    #[doc = "Bit 3 - CORDIC Error Event Flag Set"]
    #[inline]
    pub fn cderrs(&mut self) -> _CDERRSW {
        _CDERRSW { w: self }
    }
}
