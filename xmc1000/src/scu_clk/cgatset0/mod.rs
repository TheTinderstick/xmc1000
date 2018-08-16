#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CGATSET0 {
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
#[doc = "Values that can be written to the field `VADC`"]
pub enum VADCW {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "enable gating"]
    VALUE2,
}
impl VADCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VADCW::VALUE1 => false,
            VADCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VADCW<'a> {
    w: &'a mut W,
}
impl<'a> _VADCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VADCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VADCW::VALUE1)
    }
    #[doc = "enable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VADCW::VALUE2)
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
#[doc = "Values that can be written to the field `CCU80`"]
pub enum CCU80W {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "enable gating"]
    VALUE2,
}
impl CCU80W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCU80W::VALUE1 => false,
            CCU80W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCU80W<'a> {
    w: &'a mut W,
}
impl<'a> _CCU80W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCU80W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU80W::VALUE1)
    }
    #[doc = "enable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU80W::VALUE2)
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
#[doc = "Values that can be written to the field `CCU40`"]
pub enum CCU40W {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "enable gating"]
    VALUE2,
}
impl CCU40W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCU40W::VALUE1 => false,
            CCU40W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCU40W<'a> {
    w: &'a mut W,
}
impl<'a> _CCU40W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCU40W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU40W::VALUE1)
    }
    #[doc = "enable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU40W::VALUE2)
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
#[doc = "Values that can be written to the field `USIC0`"]
pub enum USIC0W {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "enable gating"]
    VALUE2,
}
impl USIC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USIC0W::VALUE1 => false,
            USIC0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USIC0W<'a> {
    w: &'a mut W,
}
impl<'a> _USIC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USIC0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIC0W::VALUE1)
    }
    #[doc = "enable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIC0W::VALUE2)
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
#[doc = "Values that can be written to the field `BCCU0`"]
pub enum BCCU0W {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "enable gating"]
    VALUE2,
}
impl BCCU0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCCU0W::VALUE1 => false,
            BCCU0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCCU0W<'a> {
    w: &'a mut W,
}
impl<'a> _BCCU0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCCU0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BCCU0W::VALUE1)
    }
    #[doc = "enable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BCCU0W::VALUE2)
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
#[doc = "Values that can be written to the field `POSIF0`"]
pub enum POSIF0W {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "enable gating"]
    VALUE2,
}
impl POSIF0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POSIF0W::VALUE1 => false,
            POSIF0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POSIF0W<'a> {
    w: &'a mut W,
}
impl<'a> _POSIF0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POSIF0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(POSIF0W::VALUE1)
    }
    #[doc = "enable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(POSIF0W::VALUE2)
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
#[doc = "Values that can be written to the field `MATH`"]
pub enum MATHW {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "enable gating"]
    VALUE2,
}
impl MATHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MATHW::VALUE1 => false,
            MATHW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MATHW<'a> {
    w: &'a mut W,
}
impl<'a> _MATHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MATHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MATHW::VALUE1)
    }
    #[doc = "enable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MATHW::VALUE2)
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
#[doc = "Values that can be written to the field `WDT`"]
pub enum WDTW {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "enable gating"]
    VALUE2,
}
impl WDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTW::VALUE1 => false,
            WDTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WDTW::VALUE1)
    }
    #[doc = "enable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WDTW::VALUE2)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC`"]
pub enum RTCW {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "enable gating"]
    VALUE2,
}
impl RTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCW::VALUE1 => false,
            RTCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTCW::VALUE1)
    }
    #[doc = "enable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTCW::VALUE2)
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
    #[doc = "Bit 0 - VADC and SHS Gating Set"]
    #[inline]
    pub fn vadc(&mut self) -> _VADCW {
        _VADCW { w: self }
    }
    #[doc = "Bit 1 - CCU80 Gating Set"]
    #[inline]
    pub fn ccu80(&mut self) -> _CCU80W {
        _CCU80W { w: self }
    }
    #[doc = "Bit 2 - CCU40 Gating Set"]
    #[inline]
    pub fn ccu40(&mut self) -> _CCU40W {
        _CCU40W { w: self }
    }
    #[doc = "Bit 3 - USIC0 Gating Set"]
    #[inline]
    pub fn usic0(&mut self) -> _USIC0W {
        _USIC0W { w: self }
    }
    #[doc = "Bit 4 - BCCU0 Gating Set"]
    #[inline]
    pub fn bccu0(&mut self) -> _BCCU0W {
        _BCCU0W { w: self }
    }
    #[doc = "Bit 7 - POSIF0 Gating Set"]
    #[inline]
    pub fn posif0(&mut self) -> _POSIF0W {
        _POSIF0W { w: self }
    }
    #[doc = "Bit 8 - MATH Gating Set"]
    #[inline]
    pub fn math(&mut self) -> _MATHW {
        _MATHW { w: self }
    }
    #[doc = "Bit 9 - WDT Gating Set"]
    #[inline]
    pub fn wdt(&mut self) -> _WDTW {
        _WDTW { w: self }
    }
    #[doc = "Bit 10 - RTC Gating Set"]
    #[inline]
    pub fn rtc(&mut self) -> _RTCW {
        _RTCW { w: self }
    }
}
