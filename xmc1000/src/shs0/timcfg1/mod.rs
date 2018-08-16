#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMCFG1 {
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
#[doc = "Possible values of the field `AT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATR {
    #[doc = "Compatible timing: Result available after standard conversion time"]
    VALUE1,
    #[doc = "Accelerated timing: Result available as soon as converted"]
    VALUE2,
}
impl ATR {
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
            ATR::VALUE1 => false,
            ATR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ATR {
        match value {
            false => ATR::VALUE1,
            true => ATR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ATR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ATR::VALUE2
    }
}
#[doc = "Possible values of the field `FCRT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCRTR {
    #[doc = "Result after tADCI * 2"]
    VALUE1,
    #[doc = "Result after tADCI * 32"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FCRTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FCRTR::VALUE1 => 0,
            FCRTR::VALUE2 => 15,
            FCRTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FCRTR {
        match value {
            0 => FCRTR::VALUE1,
            15 => FCRTR::VALUE2,
            i => FCRTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FCRTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FCRTR::VALUE2
    }
}
#[doc = "Possible values of the field `SST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSTR {
    #[doc = "Compatible timing: Sample time is defined by DIVA and STC."]
    VALUE1,
    #[doc = "Sample time is tADC * 1"]
    VALUE2,
    #[doc = "Sample time is tADC * 63"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SSTR::VALUE1 => 0,
            SSTR::VALUE2 => 1,
            SSTR::VALUE3 => 63,
            SSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SSTR {
        match value {
            0 => SSTR::VALUE1,
            1 => SSTR::VALUE2,
            63 => SSTR::VALUE3,
            i => SSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SSTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SSTR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SSTR::VALUE3
    }
}
#[doc = r" Value of the field"]
pub struct TGENR {
    bits: u16,
}
impl TGENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `AT`"]
pub enum ATW {
    #[doc = "Compatible timing: Result available after standard conversion time"]
    VALUE1,
    #[doc = "Accelerated timing: Result available as soon as converted"]
    VALUE2,
}
impl ATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATW::VALUE1 => false,
            ATW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATW<'a> {
    w: &'a mut W,
}
impl<'a> _ATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compatible timing: Result available after standard conversion time"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ATW::VALUE1)
    }
    #[doc = "Accelerated timing: Result available as soon as converted"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ATW::VALUE2)
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
#[doc = "Values that can be written to the field `FCRT`"]
pub enum FCRTW {
    #[doc = "Result after tADCI * 2"]
    VALUE1,
    #[doc = "Result after tADCI * 32"]
    VALUE2,
}
impl FCRTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FCRTW::VALUE1 => 0,
            FCRTW::VALUE2 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCRTW<'a> {
    w: &'a mut W,
}
impl<'a> _FCRTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCRTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Result after tADCI * 2"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FCRTW::VALUE1)
    }
    #[doc = "Result after tADCI * 32"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FCRTW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SST`"]
pub enum SSTW {
    #[doc = "Compatible timing: Sample time is defined by DIVA and STC."]
    VALUE1,
    #[doc = "Sample time is tADC * 1"]
    VALUE2,
    #[doc = "Sample time is tADC * 63"]
    VALUE3,
}
impl SSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSTW::VALUE1 => 0,
            SSTW::VALUE2 => 1,
            SSTW::VALUE3 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Compatible timing: Sample time is defined by DIVA and STC."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SSTW::VALUE1)
    }
    #[doc = "Sample time is tADC * 1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SSTW::VALUE2)
    }
    #[doc = "Sample time is tADC * 63"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SSTW::VALUE3)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Accelerated Timing"]
    #[inline]
    pub fn at(&self) -> ATR {
        ATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:7 - Fast Compare Mode Response Time"]
    #[inline]
    pub fn fcrt(&self) -> FCRTR {
        FCRTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - Short Sample Time"]
    #[inline]
    pub fn sst(&self) -> SSTR {
        SSTR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:29 - Timing Generator"]
    #[inline]
    pub fn tgen(&self) -> TGENR {
        let bits = {
            const MASK: u16 = 16383;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TGENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Accelerated Timing"]
    #[inline]
    pub fn at(&mut self) -> _ATW {
        _ATW { w: self }
    }
    #[doc = "Bits 4:7 - Fast Compare Mode Response Time"]
    #[inline]
    pub fn fcrt(&mut self) -> _FCRTW {
        _FCRTW { w: self }
    }
    #[doc = "Bits 8:13 - Short Sample Time"]
    #[inline]
    pub fn sst(&mut self) -> _SSTW {
        _SSTW { w: self }
    }
}
