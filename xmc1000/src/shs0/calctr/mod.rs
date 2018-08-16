#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CALCTR {
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
#[doc = "Possible values of the field `CALORD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALORDR {
    #[doc = "Do conversions then calibration"]
    VALUE1,
    #[doc = "Do calibration then conversions"]
    VALUE2,
}
impl CALORDR {
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
            CALORDR::VALUE1 => false,
            CALORDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CALORDR {
        match value {
            false => CALORDR::VALUE1,
            true => CALORDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CALORDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CALORDR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct CALGNSTCR {
    bits: u8,
}
impl CALGNSTCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SUCALVALR {
    bits: u8,
}
impl SUCALVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CALMAXR {
    bits: u8,
}
impl CALMAXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `CALORD`"]
pub enum CALORDW {
    #[doc = "Do conversions then calibration"]
    VALUE1,
    #[doc = "Do calibration then conversions"]
    VALUE2,
}
impl CALORDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CALORDW::VALUE1 => false,
            CALORDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CALORDW<'a> {
    w: &'a mut W,
}
impl<'a> _CALORDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CALORDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do conversions then calibration"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CALORDW::VALUE1)
    }
    #[doc = "Do calibration then conversions"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CALORDW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _CALGNSTCW<'a> {
    w: &'a mut W,
}
impl<'a> _CALGNSTCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SUCALVALW<'a> {
    w: &'a mut W,
}
impl<'a> _SUCALVALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CALMAXW<'a> {
    w: &'a mut W,
}
impl<'a> _CALMAXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SUCAL`"]
pub enum SUCALW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate the start-up calibration phase (indication in bitfield SHSCFG.STATE)"]
    VALUE2,
}
impl SUCALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUCALW::VALUE1 => false,
            SUCALW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUCALW<'a> {
    w: &'a mut W,
}
impl<'a> _SUCALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUCALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUCALW::VALUE1)
    }
    #[doc = "Initiate the start-up calibration phase (indication in bitfield SHSCFG.STATE)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUCALW::VALUE2)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Calibration Order"]
    #[inline]
    pub fn calord(&self) -> CALORDR {
        CALORDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:13 - Gain Calibration Sample Time Control"]
    #[inline]
    pub fn calgnstc(&self) -> CALGNSTCR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CALGNSTCR { bits }
    }
    #[doc = "Bits 16:22 - Startup Calibration Cycles"]
    #[inline]
    pub fn sucalval(&self) -> SUCALVALR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SUCALVALR { bits }
    }
    #[doc = "Bits 24:29 - Calibration Maximum Timing"]
    #[inline]
    pub fn calmax(&self) -> CALMAXR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CALMAXR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1049600 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Calibration Order"]
    #[inline]
    pub fn calord(&mut self) -> _CALORDW {
        _CALORDW { w: self }
    }
    #[doc = "Bits 8:13 - Gain Calibration Sample Time Control"]
    #[inline]
    pub fn calgnstc(&mut self) -> _CALGNSTCW {
        _CALGNSTCW { w: self }
    }
    #[doc = "Bits 16:22 - Startup Calibration Cycles"]
    #[inline]
    pub fn sucalval(&mut self) -> _SUCALVALW {
        _SUCALVALW { w: self }
    }
    #[doc = "Bits 24:29 - Calibration Maximum Timing"]
    #[inline]
    pub fn calmax(&mut self) -> _CALMAXW {
        _CALMAXW { w: self }
    }
    #[doc = "Bit 31 - Start-Up Calibration"]
    #[inline]
    pub fn sucal(&mut self) -> _SUCALW {
        _SUCALW { w: self }
    }
}
