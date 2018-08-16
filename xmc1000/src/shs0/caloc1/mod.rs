#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CALOC1 {
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
#[doc = r" Value of the field"]
pub struct CALOFFVAL0R {
    bits: u8,
}
impl CALOFFVAL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CALOFFVAL1R {
    bits: u8,
}
impl CALOFFVAL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CALOFFVAL2R {
    bits: u8,
}
impl CALOFFVAL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CALOFFVAL3R {
    bits: u8,
}
impl CALOFFVAL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DISCAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCALR {
    #[doc = "Calibration enabled (offset and gain)"]
    VALUE1,
    #[doc = "No calibration"]
    VALUE2,
}
impl DISCALR {
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
            DISCALR::VALUE1 => false,
            DISCALR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISCALR {
        match value {
            false => DISCALR::VALUE1,
            true => DISCALR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DISCALR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DISCALR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _CALOFFVAL0W<'a> {
    w: &'a mut W,
}
impl<'a> _CALOFFVAL0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CALOFFVAL1W<'a> {
    w: &'a mut W,
}
impl<'a> _CALOFFVAL1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CALOFFVAL2W<'a> {
    w: &'a mut W,
}
impl<'a> _CALOFFVAL2W<'a> {
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
pub struct _CALOFFVAL3W<'a> {
    w: &'a mut W,
}
impl<'a> _CALOFFVAL3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OFFWC`"]
pub enum OFFWCW {
    #[doc = "No write access to offset cal. parameters"]
    VALUE1,
    #[doc = "CALOFFVALz can be written"]
    VALUE2,
}
impl OFFWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OFFWCW::VALUE1 => false,
            OFFWCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OFFWCW<'a> {
    w: &'a mut W,
}
impl<'a> _OFFWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFFWCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No write access to offset cal. parameters"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OFFWCW::VALUE1)
    }
    #[doc = "CALOFFVALz can be written"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OFFWCW::VALUE2)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DISCAL`"]
pub enum DISCALW {
    #[doc = "Calibration enabled (offset and gain)"]
    VALUE1,
    #[doc = "No calibration"]
    VALUE2,
}
impl DISCALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISCALW::VALUE1 => false,
            DISCALW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISCALW<'a> {
    w: &'a mut W,
}
impl<'a> _DISCALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISCALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Calibration enabled (offset and gain)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DISCALW::VALUE1)
    }
    #[doc = "No calibration"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DISCALW::VALUE2)
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
    #[doc = "Bits 0:6 - Offset Calibration Value for Gain Level z"]
    #[inline]
    pub fn caloffval0(&self) -> CALOFFVAL0R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CALOFFVAL0R { bits }
    }
    #[doc = "Bits 8:14 - Offset Calibration Value for Gain Level z"]
    #[inline]
    pub fn caloffval1(&self) -> CALOFFVAL1R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CALOFFVAL1R { bits }
    }
    #[doc = "Bits 16:22 - Offset Calibration Value for Gain Level z"]
    #[inline]
    pub fn caloffval2(&self) -> CALOFFVAL2R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CALOFFVAL2R { bits }
    }
    #[doc = "Bits 24:30 - Offset Calibration Value for Gain Level z"]
    #[inline]
    pub fn caloffval3(&self) -> CALOFFVAL3R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CALOFFVAL3R { bits }
    }
    #[doc = "Bit 31 - Disable Calibration"]
    #[inline]
    pub fn discal(&self) -> DISCALR {
        DISCALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:6 - Offset Calibration Value for Gain Level z"]
    #[inline]
    pub fn caloffval0(&mut self) -> _CALOFFVAL0W {
        _CALOFFVAL0W { w: self }
    }
    #[doc = "Bits 8:14 - Offset Calibration Value for Gain Level z"]
    #[inline]
    pub fn caloffval1(&mut self) -> _CALOFFVAL1W {
        _CALOFFVAL1W { w: self }
    }
    #[doc = "Bits 16:22 - Offset Calibration Value for Gain Level z"]
    #[inline]
    pub fn caloffval2(&mut self) -> _CALOFFVAL2W {
        _CALOFFVAL2W { w: self }
    }
    #[doc = "Bits 24:30 - Offset Calibration Value for Gain Level z"]
    #[inline]
    pub fn caloffval3(&mut self) -> _CALOFFVAL3W {
        _CALOFFVAL3W { w: self }
    }
    #[doc = "Bit 15 - Offset Calibration Write Control"]
    #[inline]
    pub fn offwc(&mut self) -> _OFFWCW {
        _OFFWCW { w: self }
    }
    #[doc = "Bit 31 - Disable Calibration"]
    #[inline]
    pub fn discal(&mut self) -> _DISCALW {
        _DISCALW { w: self }
    }
}
