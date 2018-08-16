#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GLOBCLK {
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
#[doc = "Possible values of the field `FCLK_PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCLK_PSR {
    #[doc = "No clock"]
    VALUE1,
    #[doc = "Divide by 1"]
    VALUE2,
    #[doc = "Divide by 4095"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl FCLK_PSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            FCLK_PSR::VALUE1 => 0,
            FCLK_PSR::VALUE2 => 1,
            FCLK_PSR::VALUE3 => 4095,
            FCLK_PSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> FCLK_PSR {
        match value {
            0 => FCLK_PSR::VALUE1,
            1 => FCLK_PSR::VALUE2,
            4095 => FCLK_PSR::VALUE3,
            i => FCLK_PSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FCLK_PSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FCLK_PSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == FCLK_PSR::VALUE3
    }
}
#[doc = "Possible values of the field `BCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCSR {
    #[doc = "Normal Mode: BCCU_bclk is generated from BCCU_fclk by a division of 4"]
    VALUE1,
    #[doc = "Fast Mode: BCCU_bclk is the same as BCCU_fclk"]
    VALUE2,
}
impl BCSR {
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
            BCSR::VALUE1 => false,
            BCSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCSR {
        match value {
            false => BCSR::VALUE1,
            true => BCSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BCSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BCSR::VALUE2
    }
}
#[doc = "Possible values of the field `DCLK_PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCLK_PSR {
    #[doc = "No clock"]
    VALUE1,
    #[doc = "Divide by 1"]
    VALUE2,
    #[doc = "Divide by 4095"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl DCLK_PSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            DCLK_PSR::VALUE1 => 0,
            DCLK_PSR::VALUE2 => 1,
            DCLK_PSR::VALUE3 => 4095,
            DCLK_PSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> DCLK_PSR {
        match value {
            0 => DCLK_PSR::VALUE1,
            1 => DCLK_PSR::VALUE2,
            4095 => DCLK_PSR::VALUE3,
            i => DCLK_PSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DCLK_PSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DCLK_PSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DCLK_PSR::VALUE3
    }
}
#[doc = "Values that can be written to the field `FCLK_PS`"]
pub enum FCLK_PSW {
    #[doc = "No clock"]
    VALUE1,
    #[doc = "Divide by 1"]
    VALUE2,
    #[doc = "Divide by 4095"]
    VALUE3,
}
impl FCLK_PSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            FCLK_PSW::VALUE1 => 0,
            FCLK_PSW::VALUE2 => 1,
            FCLK_PSW::VALUE3 => 4095,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCLK_PSW<'a> {
    w: &'a mut W,
}
impl<'a> _FCLK_PSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCLK_PSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No clock"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FCLK_PSW::VALUE1)
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FCLK_PSW::VALUE2)
    }
    #[doc = "Divide by 4095"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(FCLK_PSW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BCS`"]
pub enum BCSW {
    #[doc = "Normal Mode: BCCU_bclk is generated from BCCU_fclk by a division of 4"]
    VALUE1,
    #[doc = "Fast Mode: BCCU_bclk is the same as BCCU_fclk"]
    VALUE2,
}
impl BCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCSW::VALUE1 => false,
            BCSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCSW<'a> {
    w: &'a mut W,
}
impl<'a> _BCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal Mode: BCCU_bclk is generated from BCCU_fclk by a division of 4"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BCSW::VALUE1)
    }
    #[doc = "Fast Mode: BCCU_bclk is the same as BCCU_fclk"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BCSW::VALUE2)
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
#[doc = "Values that can be written to the field `DCLK_PS`"]
pub enum DCLK_PSW {
    #[doc = "No clock"]
    VALUE1,
    #[doc = "Divide by 1"]
    VALUE2,
    #[doc = "Divide by 4095"]
    VALUE3,
}
impl DCLK_PSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            DCLK_PSW::VALUE1 => 0,
            DCLK_PSW::VALUE2 => 1,
            DCLK_PSW::VALUE3 => 4095,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCLK_PSW<'a> {
    w: &'a mut W,
}
impl<'a> _DCLK_PSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCLK_PSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No clock"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DCLK_PSW::VALUE1)
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DCLK_PSW::VALUE2)
    }
    #[doc = "Divide by 4095"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DCLK_PSW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:11 - Fast Clock Prescaler Factor"]
    #[inline]
    pub fn fclk_ps(&self) -> FCLK_PSR {
        FCLK_PSR::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bit 15 - Bit-Clock Selector"]
    #[inline]
    pub fn bcs(&self) -> BCSR {
        BCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:27 - Dimmer Clock Prescaler Factor"]
    #[inline]
    pub fn dclk_ps(&self) -> DCLK_PSR {
        DCLK_PSR::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 14352784 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:11 - Fast Clock Prescaler Factor"]
    #[inline]
    pub fn fclk_ps(&mut self) -> _FCLK_PSW {
        _FCLK_PSW { w: self }
    }
    #[doc = "Bit 15 - Bit-Clock Selector"]
    #[inline]
    pub fn bcs(&mut self) -> _BCSW {
        _BCSW { w: self }
    }
    #[doc = "Bits 16:27 - Dimmer Clock Prescaler Factor"]
    #[inline]
    pub fn dclk_ps(&mut self) -> _DCLK_PSW {
        _DCLK_PSW { w: self }
    }
}
