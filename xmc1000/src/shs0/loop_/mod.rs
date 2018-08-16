#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LOOP {
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
pub struct LPCH0R {
    bits: u8,
}
impl LPCH0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPCH1R {
    bits: u8,
}
impl LPCH1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPSH0R {
    bits: bool,
}
impl LPSH0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct LPSH1R {
    bits: bool,
}
impl LPSH1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `LPEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPEN0R {
    #[doc = "Off: standard operation"]
    VALUE1,
    #[doc = "ON: sigma-delta-loop is active"]
    VALUE2,
}
impl LPEN0R {
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
            LPEN0R::VALUE1 => false,
            LPEN0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPEN0R {
        match value {
            false => LPEN0R::VALUE1,
            true => LPEN0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LPEN0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LPEN0R::VALUE2
    }
}
#[doc = "Possible values of the field `LPEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPEN1R {
    #[doc = "Off: standard operation"]
    VALUE1,
    #[doc = "ON: sigma-delta-loop is active"]
    VALUE2,
}
impl LPEN1R {
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
            LPEN1R::VALUE1 => false,
            LPEN1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPEN1R {
        match value {
            false => LPEN1R::VALUE1,
            true => LPEN1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LPEN1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LPEN1R::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _LPCH0W<'a> {
    w: &'a mut W,
}
impl<'a> _LPCH0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPCH1W<'a> {
    w: &'a mut W,
}
impl<'a> _LPCH1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPSH0W<'a> {
    w: &'a mut W,
}
impl<'a> _LPSH0W<'a> {
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
#[doc = r" Proxy"]
pub struct _LPSH1W<'a> {
    w: &'a mut W,
}
impl<'a> _LPSH1W<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPEN0`"]
pub enum LPEN0W {
    #[doc = "Off: standard operation"]
    VALUE1,
    #[doc = "ON: sigma-delta-loop is active"]
    VALUE2,
}
impl LPEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPEN0W::VALUE1 => false,
            LPEN0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _LPEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off: standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPEN0W::VALUE1)
    }
    #[doc = "ON: sigma-delta-loop is active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPEN0W::VALUE2)
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
#[doc = "Values that can be written to the field `LPEN1`"]
pub enum LPEN1W {
    #[doc = "Off: standard operation"]
    VALUE1,
    #[doc = "ON: sigma-delta-loop is active"]
    VALUE2,
}
impl LPEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPEN1W::VALUE1 => false,
            LPEN1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _LPEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off: standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPEN1W::VALUE1)
    }
    #[doc = "ON: sigma-delta-loop is active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPEN1W::VALUE2)
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
    #[doc = "Bits 0:4 - Loop y Channel"]
    #[inline]
    pub fn lpch0(&self) -> LPCH0R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPCH0R { bits }
    }
    #[doc = "Bits 16:20 - Loop y Channel"]
    #[inline]
    pub fn lpch1(&self) -> LPCH1R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPCH1R { bits }
    }
    #[doc = "Bit 8 - Loop y Sample&Hold Unit"]
    #[inline]
    pub fn lpsh0(&self) -> LPSH0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPSH0R { bits }
    }
    #[doc = "Bit 24 - Loop y Sample&Hold Unit"]
    #[inline]
    pub fn lpsh1(&self) -> LPSH1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPSH1R { bits }
    }
    #[doc = "Bit 15 - Loop y Enable"]
    #[inline]
    pub fn lpen0(&self) -> LPEN0R {
        LPEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Loop y Enable"]
    #[inline]
    pub fn lpen1(&self) -> LPEN1R {
        LPEN1R::_from({
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
    #[doc = "Bits 0:4 - Loop y Channel"]
    #[inline]
    pub fn lpch0(&mut self) -> _LPCH0W {
        _LPCH0W { w: self }
    }
    #[doc = "Bits 16:20 - Loop y Channel"]
    #[inline]
    pub fn lpch1(&mut self) -> _LPCH1W {
        _LPCH1W { w: self }
    }
    #[doc = "Bit 8 - Loop y Sample&Hold Unit"]
    #[inline]
    pub fn lpsh0(&mut self) -> _LPSH0W {
        _LPSH0W { w: self }
    }
    #[doc = "Bit 24 - Loop y Sample&Hold Unit"]
    #[inline]
    pub fn lpsh1(&mut self) -> _LPSH1W {
        _LPSH1W { w: self }
    }
    #[doc = "Bit 15 - Loop y Enable"]
    #[inline]
    pub fn lpen0(&mut self) -> _LPEN0W {
        _LPEN0W { w: self }
    }
    #[doc = "Bit 31 - Loop y Enable"]
    #[inline]
    pub fn lpen1(&mut self) -> _LPEN1W {
        _LPEN1W { w: self }
    }
}
