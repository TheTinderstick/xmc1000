#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::ANAVDEL {
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
#[doc = "Possible values of the field `VDEL_SELECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDEL_SELECTR {
    #[doc = "2.25V"]
    VALUE1,
    #[doc = "3.0V"]
    VALUE2,
    #[doc = "4.4V"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VDEL_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VDEL_SELECTR::VALUE1 => 0,
            VDEL_SELECTR::VALUE2 => 1,
            VDEL_SELECTR::VALUE3 => 2,
            VDEL_SELECTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VDEL_SELECTR {
        match value {
            0 => VDEL_SELECTR::VALUE1,
            1 => VDEL_SELECTR::VALUE2,
            2 => VDEL_SELECTR::VALUE3,
            i => VDEL_SELECTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VDEL_SELECTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VDEL_SELECTR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == VDEL_SELECTR::VALUE3
    }
}
#[doc = "Possible values of the field `VDEL_TIM_ADJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDEL_TIM_ADJR {
    #[doc = "typ 1us - slowest response time"]
    VALUE1,
    #[doc = "typ 500n"]
    VALUE2,
    #[doc = "typ 250n"]
    VALUE3,
    #[doc = "no delay - fastest response time."]
    VALUE4,
}
impl VDEL_TIM_ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VDEL_TIM_ADJR::VALUE1 => 0,
            VDEL_TIM_ADJR::VALUE2 => 1,
            VDEL_TIM_ADJR::VALUE3 => 2,
            VDEL_TIM_ADJR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VDEL_TIM_ADJR {
        match value {
            0 => VDEL_TIM_ADJR::VALUE1,
            1 => VDEL_TIM_ADJR::VALUE2,
            2 => VDEL_TIM_ADJR::VALUE3,
            3 => VDEL_TIM_ADJR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VDEL_TIM_ADJR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VDEL_TIM_ADJR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == VDEL_TIM_ADJR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == VDEL_TIM_ADJR::VALUE4
    }
}
#[doc = "Possible values of the field `VDEL_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDEL_ENR {
    #[doc = "VDEL is disabled"]
    VALUE1,
    #[doc = "VDEL is active"]
    VALUE2,
}
impl VDEL_ENR {
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
            VDEL_ENR::VALUE1 => false,
            VDEL_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VDEL_ENR {
        match value {
            false => VDEL_ENR::VALUE1,
            true => VDEL_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VDEL_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VDEL_ENR::VALUE2
    }
}
#[doc = "Values that can be written to the field `VDEL_SELECT`"]
pub enum VDEL_SELECTW {
    #[doc = "2.25V"]
    VALUE1,
    #[doc = "3.0V"]
    VALUE2,
    #[doc = "4.4V"]
    VALUE3,
}
impl VDEL_SELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VDEL_SELECTW::VALUE1 => 0,
            VDEL_SELECTW::VALUE2 => 1,
            VDEL_SELECTW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VDEL_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _VDEL_SELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VDEL_SELECTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "2.25V"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VDEL_SELECTW::VALUE1)
    }
    #[doc = "3.0V"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VDEL_SELECTW::VALUE2)
    }
    #[doc = "4.4V"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(VDEL_SELECTW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VDEL_TIM_ADJ`"]
pub enum VDEL_TIM_ADJW {
    #[doc = "typ 1us - slowest response time"]
    VALUE1,
    #[doc = "typ 500n"]
    VALUE2,
    #[doc = "typ 250n"]
    VALUE3,
    #[doc = "no delay - fastest response time."]
    VALUE4,
}
impl VDEL_TIM_ADJW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VDEL_TIM_ADJW::VALUE1 => 0,
            VDEL_TIM_ADJW::VALUE2 => 1,
            VDEL_TIM_ADJW::VALUE3 => 2,
            VDEL_TIM_ADJW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VDEL_TIM_ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _VDEL_TIM_ADJW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VDEL_TIM_ADJW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "typ 1us - slowest response time"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VDEL_TIM_ADJW::VALUE1)
    }
    #[doc = "typ 500n"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VDEL_TIM_ADJW::VALUE2)
    }
    #[doc = "typ 250n"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(VDEL_TIM_ADJW::VALUE3)
    }
    #[doc = "no delay - fastest response time."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(VDEL_TIM_ADJW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VDEL_EN`"]
pub enum VDEL_ENW {
    #[doc = "VDEL is disabled"]
    VALUE1,
    #[doc = "VDEL is active"]
    VALUE2,
}
impl VDEL_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VDEL_ENW::VALUE1 => false,
            VDEL_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VDEL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _VDEL_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VDEL_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "VDEL is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VDEL_ENW::VALUE1)
    }
    #[doc = "VDEL is active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VDEL_ENW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:1 - VDEL Range Select"]
    #[inline]
    pub fn vdel_select(&self) -> VDEL_SELECTR {
        VDEL_SELECTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 2:3 - VDEL Timing Setting"]
    #[inline]
    pub fn vdel_tim_adj(&self) -> VDEL_TIM_ADJR {
        VDEL_TIM_ADJR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 4 - VDEL unit Enable"]
    #[inline]
    pub fn vdel_en(&self) -> VDEL_ENR {
        VDEL_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 28 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - VDEL Range Select"]
    #[inline]
    pub fn vdel_select(&mut self) -> _VDEL_SELECTW {
        _VDEL_SELECTW { w: self }
    }
    #[doc = "Bits 2:3 - VDEL Timing Setting"]
    #[inline]
    pub fn vdel_tim_adj(&mut self) -> _VDEL_TIM_ADJW {
        _VDEL_TIM_ADJW { w: self }
    }
    #[doc = "Bit 4 - VDEL unit Enable"]
    #[inline]
    pub fn vdel_en(&mut self) -> _VDEL_ENW {
        _VDEL_ENW { w: self }
    }
}
