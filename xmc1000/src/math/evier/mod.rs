#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVIER {
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
#[doc = "Possible values of the field `DIVEOCIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVEOCIENR {
    #[doc = "Divider end of calculation interrupt generation is disabled."]
    VALUE1,
    #[doc = "Divider end of calculation interrupt generation is enabled."]
    VALUE2,
}
impl DIVEOCIENR {
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
            DIVEOCIENR::VALUE1 => false,
            DIVEOCIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIVEOCIENR {
        match value {
            false => DIVEOCIENR::VALUE1,
            true => DIVEOCIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIVEOCIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIVEOCIENR::VALUE2
    }
}
#[doc = "Possible values of the field `DIVERRIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVERRIENR {
    #[doc = "Divider error interrupt generation is disabled"]
    VALUE1,
    #[doc = "Divider error interrupt generation is enabled"]
    VALUE2,
}
impl DIVERRIENR {
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
            DIVERRIENR::VALUE1 => false,
            DIVERRIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIVERRIENR {
        match value {
            false => DIVERRIENR::VALUE1,
            true => DIVERRIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIVERRIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIVERRIENR::VALUE2
    }
}
#[doc = "Possible values of the field `CDEOCIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDEOCIENR {
    #[doc = "CORDIC end of calculation interrupt generation is disabled."]
    VALUE1,
    #[doc = "CORDIC end of calculation interrupt generation is enabled."]
    VALUE2,
}
impl CDEOCIENR {
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
            CDEOCIENR::VALUE1 => false,
            CDEOCIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CDEOCIENR {
        match value {
            false => CDEOCIENR::VALUE1,
            true => CDEOCIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CDEOCIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CDEOCIENR::VALUE2
    }
}
#[doc = "Possible values of the field `CDERRIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDERRIENR {
    #[doc = "CORDIC error interrupt generation is disabled"]
    VALUE1,
    #[doc = "CORDIC error interrupt generation is enabled"]
    VALUE2,
}
impl CDERRIENR {
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
            CDERRIENR::VALUE1 => false,
            CDERRIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CDERRIENR {
        match value {
            false => CDERRIENR::VALUE1,
            true => CDERRIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CDERRIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CDERRIENR::VALUE2
    }
}
#[doc = "Values that can be written to the field `DIVEOCIEN`"]
pub enum DIVEOCIENW {
    #[doc = "Divider end of calculation interrupt generation is disabled."]
    VALUE1,
    #[doc = "Divider end of calculation interrupt generation is enabled."]
    VALUE2,
}
impl DIVEOCIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIVEOCIENW::VALUE1 => false,
            DIVEOCIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVEOCIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVEOCIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVEOCIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Divider end of calculation interrupt generation is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVEOCIENW::VALUE1)
    }
    #[doc = "Divider end of calculation interrupt generation is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVEOCIENW::VALUE2)
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
#[doc = "Values that can be written to the field `DIVERRIEN`"]
pub enum DIVERRIENW {
    #[doc = "Divider error interrupt generation is disabled"]
    VALUE1,
    #[doc = "Divider error interrupt generation is enabled"]
    VALUE2,
}
impl DIVERRIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIVERRIENW::VALUE1 => false,
            DIVERRIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVERRIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVERRIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVERRIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Divider error interrupt generation is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVERRIENW::VALUE1)
    }
    #[doc = "Divider error interrupt generation is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVERRIENW::VALUE2)
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
#[doc = "Values that can be written to the field `CDEOCIEN`"]
pub enum CDEOCIENW {
    #[doc = "CORDIC end of calculation interrupt generation is disabled."]
    VALUE1,
    #[doc = "CORDIC end of calculation interrupt generation is enabled."]
    VALUE2,
}
impl CDEOCIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDEOCIENW::VALUE1 => false,
            CDEOCIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDEOCIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CDEOCIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDEOCIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORDIC end of calculation interrupt generation is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDEOCIENW::VALUE1)
    }
    #[doc = "CORDIC end of calculation interrupt generation is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDEOCIENW::VALUE2)
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
#[doc = "Values that can be written to the field `CDERRIEN`"]
pub enum CDERRIENW {
    #[doc = "CORDIC error interrupt generation is disabled"]
    VALUE1,
    #[doc = "CORDIC error interrupt generation is enabled"]
    VALUE2,
}
impl CDERRIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDERRIENW::VALUE1 => false,
            CDERRIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDERRIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CDERRIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDERRIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORDIC error interrupt generation is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDERRIENW::VALUE1)
    }
    #[doc = "CORDIC error interrupt generation is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDERRIENW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Divider End of Calculation Interrupt Enable"]
    #[inline]
    pub fn diveocien(&self) -> DIVEOCIENR {
        DIVEOCIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Divider Error Interrupt Enable"]
    #[inline]
    pub fn diverrien(&self) -> DIVERRIENR {
        DIVERRIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - CORDIC End of Calculation Interrupt Enable"]
    #[inline]
    pub fn cdeocien(&self) -> CDEOCIENR {
        CDEOCIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - CORDIC Error Interrupt Enable"]
    #[inline]
    pub fn cderrien(&self) -> CDERRIENR {
        CDERRIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Divider End of Calculation Interrupt Enable"]
    #[inline]
    pub fn diveocien(&mut self) -> _DIVEOCIENW {
        _DIVEOCIENW { w: self }
    }
    #[doc = "Bit 1 - Divider Error Interrupt Enable"]
    #[inline]
    pub fn diverrien(&mut self) -> _DIVERRIENW {
        _DIVERRIENW { w: self }
    }
    #[doc = "Bit 2 - CORDIC End of Calculation Interrupt Enable"]
    #[inline]
    pub fn cdeocien(&mut self) -> _CDEOCIENW {
        _CDEOCIENW { w: self }
    }
    #[doc = "Bit 3 - CORDIC Error Interrupt Enable"]
    #[inline]
    pub fn cderrien(&mut self) -> _CDERRIENW {
        _CDERRIENW { w: self }
    }
}
