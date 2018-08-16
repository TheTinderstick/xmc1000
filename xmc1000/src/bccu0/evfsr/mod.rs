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
#[doc = "Values that can be written to the field `T0FS`"]
pub enum T0FSW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Sets the Trigger 0 Flag in EVFR and an interrupt will be generated if enabled in EVIER"]
    VALUE2,
}
impl T0FSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            T0FSW::VALUE1 => false,
            T0FSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _T0FSW<'a> {
    w: &'a mut W,
}
impl<'a> _T0FSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: T0FSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(T0FSW::VALUE1)
    }
    #[doc = "Sets the Trigger 0 Flag in EVFR and an interrupt will be generated if enabled in EVIER"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(T0FSW::VALUE2)
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
#[doc = "Values that can be written to the field `T1FS`"]
pub enum T1FSW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Sets the Trigger 1 Flag in EVFR and an interrupt will be generated if enabled in EVIER"]
    VALUE2,
}
impl T1FSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            T1FSW::VALUE1 => false,
            T1FSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _T1FSW<'a> {
    w: &'a mut W,
}
impl<'a> _T1FSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: T1FSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(T1FSW::VALUE1)
    }
    #[doc = "Sets the Trigger 1 Flag in EVFR and an interrupt will be generated if enabled in EVIER"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(T1FSW::VALUE2)
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
#[doc = "Values that can be written to the field `FFS`"]
pub enum FFSW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Sets the FIFO Full Flag in EVFR and an interrupt will be generated if enabled in EVIER"]
    VALUE2,
}
impl FFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FFSW::VALUE1 => false,
            FFSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FFSW<'a> {
    w: &'a mut W,
}
impl<'a> _FFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FFSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FFSW::VALUE1)
    }
    #[doc = "Sets the FIFO Full Flag in EVFR and an interrupt will be generated if enabled in EVIER"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FFSW::VALUE2)
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
#[doc = "Values that can be written to the field `EFS`"]
pub enum EFSW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Sets the FIFO Empty Flag in EVFR and an interrupt will be generated if enabled in EVIER"]
    VALUE2,
}
impl EFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EFSW::VALUE1 => false,
            EFSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EFSW<'a> {
    w: &'a mut W,
}
impl<'a> _EFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EFSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EFSW::VALUE1)
    }
    #[doc = "Sets the FIFO Empty Flag in EVFR and an interrupt will be generated if enabled in EVIER"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EFSW::VALUE2)
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
#[doc = "Values that can be written to the field `TPFS`"]
pub enum TPFSW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Sets the Trap Flag in EVFR and an interrupt will be generated if enabled in EVIER, no trap will occur"]
    VALUE2,
}
impl TPFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPFSW::VALUE1 => false,
            TPFSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPFSW<'a> {
    w: &'a mut W,
}
impl<'a> _TPFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPFSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TPFSW::VALUE1)
    }
    #[doc = "Sets the Trap Flag in EVFR and an interrupt will be generated if enabled in EVIER, no trap will occur"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TPFSW::VALUE2)
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
#[doc = "Values that can be written to the field `TPS`"]
pub enum TPSW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Sets the Trap State Flag and Trap Flag in EVFR, a trap will be generated and an interrupt will be generated if enabled in EVIER"]
    VALUE2,
}
impl TPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPSW::VALUE1 => false,
            TPSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPSW<'a> {
    w: &'a mut W,
}
impl<'a> _TPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TPSW::VALUE1)
    }
    #[doc = "Sets the Trap State Flag and Trap Flag in EVFR, a trap will be generated and an interrupt will be generated if enabled in EVIER"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TPSW::VALUE2)
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
    #[doc = "Bit 0 - Trigger 0 Flag Set"]
    #[inline]
    pub fn t0fs(&mut self) -> _T0FSW {
        _T0FSW { w: self }
    }
    #[doc = "Bit 1 - Trigger 1 Flag Set"]
    #[inline]
    pub fn t1fs(&mut self) -> _T1FSW {
        _T1FSW { w: self }
    }
    #[doc = "Bit 2 - FIFO Full Flag Set"]
    #[inline]
    pub fn ffs(&mut self) -> _FFSW {
        _FFSW { w: self }
    }
    #[doc = "Bit 3 - FIFO Empty Flag Set"]
    #[inline]
    pub fn efs(&mut self) -> _EFSW {
        _EFSW { w: self }
    }
    #[doc = "Bit 4 - Trap Flag Set"]
    #[inline]
    pub fn tpfs(&mut self) -> _TPFSW {
        _TPFSW { w: self }
    }
    #[doc = "Bit 6 - Trap Set"]
    #[inline]
    pub fn tps(&mut self) -> _TPSW {
        _TPSW { w: self }
    }
}
