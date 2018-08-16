#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OSCCSR {
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
#[doc = "Possible values of the field `OSC2L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC2LR {
    #[doc = "The OSC frequency is usable"]
    VALUE1,
    #[doc = "The OSC frequency is not usable. Frequency is too low."]
    VALUE2,
}
impl OSC2LR {
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
            OSC2LR::VALUE1 => false,
            OSC2LR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSC2LR {
        match value {
            false => OSC2LR::VALUE1,
            true => OSC2LR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OSC2LR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OSC2LR::VALUE2
    }
}
#[doc = "Possible values of the field `OSC2H`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC2HR {
    #[doc = "The OSC frequency is usable"]
    VALUE1,
    #[doc = "The OSC frequency is not usable. Frequency is too high."]
    VALUE2,
}
impl OSC2HR {
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
            OSC2HR::VALUE1 => false,
            OSC2HR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSC2HR {
        match value {
            false => OSC2HR::VALUE1,
            true => OSC2HR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OSC2HR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OSC2HR::VALUE2
    }
}
#[doc = "Possible values of the field `OWDRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWDRESR {
    #[doc = "The Oscillator Watchdog is not cleared and remains active"]
    VALUE1,
    #[doc = "The Oscillator Watchdog is cleared and restarted. The OSC2L and OSC2H flag will be held in the last value until it is updated after 3 standby clock cycles."]
    VALUE2,
}
impl OWDRESR {
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
            OWDRESR::VALUE1 => false,
            OWDRESR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OWDRESR {
        match value {
            false => OWDRESR::VALUE1,
            true => OWDRESR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OWDRESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OWDRESR::VALUE2
    }
}
#[doc = "Possible values of the field `OWDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWDENR {
    #[doc = "The Oscillator Watchdog is disabled"]
    VALUE1,
    #[doc = "The Oscillator Watchdog is enabled"]
    VALUE2,
}
impl OWDENR {
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
            OWDENR::VALUE1 => false,
            OWDENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OWDENR {
        match value {
            false => OWDENR::VALUE1,
            true => OWDENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OWDENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OWDENR::VALUE2
    }
}
#[doc = "Values that can be written to the field `OWDRES`"]
pub enum OWDRESW {
    #[doc = "The Oscillator Watchdog is not cleared and remains active"]
    VALUE1,
    #[doc = "The Oscillator Watchdog is cleared and restarted. The OSC2L and OSC2H flag will be held in the last value until it is updated after 3 standby clock cycles."]
    VALUE2,
}
impl OWDRESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OWDRESW::VALUE1 => false,
            OWDRESW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OWDRESW<'a> {
    w: &'a mut W,
}
impl<'a> _OWDRESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OWDRESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The Oscillator Watchdog is not cleared and remains active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OWDRESW::VALUE1)
    }
    #[doc = "The Oscillator Watchdog is cleared and restarted. The OSC2L and OSC2H flag will be held in the last value until it is updated after 3 standby clock cycles."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OWDRESW::VALUE2)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OWDEN`"]
pub enum OWDENW {
    #[doc = "The Oscillator Watchdog is disabled"]
    VALUE1,
    #[doc = "The Oscillator Watchdog is enabled"]
    VALUE2,
}
impl OWDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OWDENW::VALUE1 => false,
            OWDENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OWDENW<'a> {
    w: &'a mut W,
}
impl<'a> _OWDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OWDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The Oscillator Watchdog is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OWDENW::VALUE1)
    }
    #[doc = "The Oscillator Watchdog is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OWDENW::VALUE2)
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
        const OFFSET: u8 = 17;
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
    #[doc = "Bit 0 - Oscillator Valid Low Status Bit"]
    #[inline]
    pub fn osc2l(&self) -> OSC2LR {
        OSC2LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Oscillator Valid High Status Bit"]
    #[inline]
    pub fn osc2h(&self) -> OSC2HR {
        OSC2HR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Oscillator Watchdog Reset"]
    #[inline]
    pub fn owdres(&self) -> OWDRESR {
        OWDRESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Oscillator Watchdog Enable"]
    #[inline]
    pub fn owden(&self) -> OWDENR {
        OWDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
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
    #[doc = "Bit 16 - Oscillator Watchdog Reset"]
    #[inline]
    pub fn owdres(&mut self) -> _OWDRESW {
        _OWDRESW { w: self }
    }
    #[doc = "Bit 17 - Oscillator Watchdog Enable"]
    #[inline]
    pub fn owden(&mut self) -> _OWDENW {
        _OWDENW { w: self }
    }
}
