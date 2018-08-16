#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CALGC0 {
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
pub struct CALGNVALSR {
    bits: u16,
}
impl CALGNVALSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CALGNVALAR {
    bits: u16,
}
impl CALGNVALAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CALGNVALSW<'a> {
    w: &'a mut W,
}
impl<'a> _CALGNVALSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 16383;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GNSWC`"]
pub enum GNSWCW {
    #[doc = "No write access to gain calibration parameter"]
    VALUE1,
    #[doc = "CALGNVALS can be written"]
    VALUE2,
}
impl GNSWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GNSWCW::VALUE1 => false,
            GNSWCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GNSWCW<'a> {
    w: &'a mut W,
}
impl<'a> _GNSWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GNSWCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No write access to gain calibration parameter"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GNSWCW::VALUE1)
    }
    #[doc = "CALGNVALS can be written"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GNSWCW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _CALGNVALAW<'a> {
    w: &'a mut W,
}
impl<'a> _CALGNVALAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 16383;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GNAWC`"]
pub enum GNAWCW {
    #[doc = "No write access to gain calibration parameter"]
    VALUE1,
    #[doc = "CALGNVALA can be written"]
    VALUE2,
}
impl GNAWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GNAWCW::VALUE1 => false,
            GNAWCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GNAWCW<'a> {
    w: &'a mut W,
}
impl<'a> _GNAWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GNAWCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No write access to gain calibration parameter"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GNAWCW::VALUE1)
    }
    #[doc = "CALGNVALA can be written"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GNAWCW::VALUE2)
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
    #[doc = "Bits 0:13 - Gain Calibration Value, Standard Reference"]
    #[inline]
    pub fn calgnvals(&self) -> CALGNVALSR {
        let bits = {
            const MASK: u16 = 16383;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CALGNVALSR { bits }
    }
    #[doc = "Bits 16:29 - Gain Calibration Value, Alternate Reference"]
    #[inline]
    pub fn calgnvala(&self) -> CALGNVALAR {
        let bits = {
            const MASK: u16 = 16383;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CALGNVALAR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 536879104 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:13 - Gain Calibration Value, Standard Reference"]
    #[inline]
    pub fn calgnvals(&mut self) -> _CALGNVALSW {
        _CALGNVALSW { w: self }
    }
    #[doc = "Bit 15 - Gain Calibration Write Control, Standard"]
    #[inline]
    pub fn gnswc(&mut self) -> _GNSWCW {
        _GNSWCW { w: self }
    }
    #[doc = "Bits 16:29 - Gain Calibration Value, Alternate Reference"]
    #[inline]
    pub fn calgnvala(&mut self) -> _CALGNVALAW {
        _CALGNVALAW { w: self }
    }
    #[doc = "Bit 31 - Gain Calibration Write Control, Alternate"]
    #[inline]
    pub fn gnawc(&mut self) -> _GNAWCW {
        _GNAWCW { w: self }
    }
}
