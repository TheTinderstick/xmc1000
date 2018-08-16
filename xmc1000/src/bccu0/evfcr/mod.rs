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
#[doc = "Values that can be written to the field `T0FC`"]
pub enum T0FCW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clears the Trigger 0 Flag in EVFR"]
    VALUE2,
}
impl T0FCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            T0FCW::VALUE1 => false,
            T0FCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _T0FCW<'a> {
    w: &'a mut W,
}
impl<'a> _T0FCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: T0FCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(T0FCW::VALUE1)
    }
    #[doc = "Clears the Trigger 0 Flag in EVFR"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(T0FCW::VALUE2)
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
#[doc = "Values that can be written to the field `T1FC`"]
pub enum T1FCW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clears the Trigger 1 Flag in EVFR"]
    VALUE2,
}
impl T1FCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            T1FCW::VALUE1 => false,
            T1FCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _T1FCW<'a> {
    w: &'a mut W,
}
impl<'a> _T1FCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: T1FCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(T1FCW::VALUE1)
    }
    #[doc = "Clears the Trigger 1 Flag in EVFR"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(T1FCW::VALUE2)
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
#[doc = "Values that can be written to the field `FFC`"]
pub enum FFCW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clears the FIFO Full Flag in EVFR"]
    VALUE2,
}
impl FFCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FFCW::VALUE1 => false,
            FFCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FFCW<'a> {
    w: &'a mut W,
}
impl<'a> _FFCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FFCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FFCW::VALUE1)
    }
    #[doc = "Clears the FIFO Full Flag in EVFR"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FFCW::VALUE2)
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
#[doc = "Values that can be written to the field `EFC`"]
pub enum EFCW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clears the FIFO Empty Flag in EVFR"]
    VALUE2,
}
impl EFCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EFCW::VALUE1 => false,
            EFCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EFCW<'a> {
    w: &'a mut W,
}
impl<'a> _EFCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EFCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EFCW::VALUE1)
    }
    #[doc = "Clears the FIFO Empty Flag in EVFR"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EFCW::VALUE2)
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
#[doc = "Values that can be written to the field `TPFC`"]
pub enum TPFCW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clears the Trap Flag in EVFR"]
    VALUE2,
}
impl TPFCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPFCW::VALUE1 => false,
            TPFCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPFCW<'a> {
    w: &'a mut W,
}
impl<'a> _TPFCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPFCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TPFCW::VALUE1)
    }
    #[doc = "Clears the Trap Flag in EVFR"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TPFCW::VALUE2)
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
#[doc = "Values that can be written to the field `TPC`"]
pub enum TPCW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clears the Trap State Flag in EVFR; trap state is exited, the affected channels will return to their normal output levels"]
    VALUE2,
}
impl TPCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPCW::VALUE1 => false,
            TPCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPCW<'a> {
    w: &'a mut W,
}
impl<'a> _TPCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TPCW::VALUE1)
    }
    #[doc = "Clears the Trap State Flag in EVFR; trap state is exited, the affected channels will return to their normal output levels"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TPCW::VALUE2)
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
    #[doc = "Bit 0 - Trigger 0 Flag Clear"]
    #[inline]
    pub fn t0fc(&mut self) -> _T0FCW {
        _T0FCW { w: self }
    }
    #[doc = "Bit 1 - Trigger 1 Flag Clear"]
    #[inline]
    pub fn t1fc(&mut self) -> _T1FCW {
        _T1FCW { w: self }
    }
    #[doc = "Bit 2 - FIFO Full Flag Clear"]
    #[inline]
    pub fn ffc(&mut self) -> _FFCW {
        _FFCW { w: self }
    }
    #[doc = "Bit 3 - FIFO Empty Flag Clear"]
    #[inline]
    pub fn efc(&mut self) -> _EFCW {
        _EFCW { w: self }
    }
    #[doc = "Bit 4 - Trap Flag Clear"]
    #[inline]
    pub fn tpfc(&mut self) -> _TPFCW {
        _TPFCW { w: self }
    }
    #[doc = "Bit 6 - Trap Clear"]
    #[inline]
    pub fn tpc(&mut self) -> _TPCW {
        _TPCW { w: self }
    }
}
