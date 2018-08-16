#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYNCTR {
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
#[doc = "Possible values of the field `STSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STSELR {
    #[doc = "Kernel is synchronization master: Use own bitfield GxARBCFG.ANONC"]
    VALUE1,
    #[doc = "Kernel is synchronization slave: Control information from input CI1"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STSELR::VALUE1 => 0,
            STSELR::VALUE2 => 1,
            STSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STSELR {
        match value {
            0 => STSELR::VALUE1,
            1 => STSELR::VALUE2,
            i => STSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STSELR::VALUE2
    }
}
#[doc = "Possible values of the field `EVALR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVALR1R {
    #[doc = "No ready input control"]
    VALUE1,
    #[doc = "Ready input R1 is considered for the start of a parallel conversion of this conversion group"]
    VALUE2,
}
impl EVALR1R {
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
            EVALR1R::VALUE1 => false,
            EVALR1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EVALR1R {
        match value {
            false => EVALR1R::VALUE1,
            true => EVALR1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EVALR1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EVALR1R::VALUE2
    }
}
#[doc = "Values that can be written to the field `STSEL`"]
pub enum STSELW {
    #[doc = "Kernel is synchronization master: Use own bitfield GxARBCFG.ANONC"]
    VALUE1,
    #[doc = "Kernel is synchronization slave: Control information from input CI1"]
    VALUE2,
}
impl STSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STSELW::VALUE1 => 0,
            STSELW::VALUE2 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STSELW<'a> {
    w: &'a mut W,
}
impl<'a> _STSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Kernel is synchronization master: Use own bitfield GxARBCFG.ANONC"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STSELW::VALUE1)
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STSELW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EVALR1`"]
pub enum EVALR1W {
    #[doc = "No ready input control"]
    VALUE1,
    #[doc = "Ready input R1 is considered for the start of a parallel conversion of this conversion group"]
    VALUE2,
}
impl EVALR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EVALR1W::VALUE1 => false,
            EVALR1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EVALR1W<'a> {
    w: &'a mut W,
}
impl<'a> _EVALR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EVALR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No ready input control"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EVALR1W::VALUE1)
    }
    #[doc = "Ready input R1 is considered for the start of a parallel conversion of this conversion group"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EVALR1W::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Start Selection"]
    #[inline]
    pub fn stsel(&self) -> STSELR {
        STSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Evaluate Ready Input R1"]
    #[inline]
    pub fn evalr1(&self) -> EVALR1R {
        EVALR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:1 - Start Selection"]
    #[inline]
    pub fn stsel(&mut self) -> _STSELW {
        _STSELW { w: self }
    }
    #[doc = "Bit 4 - Evaluate Ready Input R1"]
    #[inline]
    pub fn evalr1(&mut self) -> _EVALR1W {
        _EVALR1W { w: self }
    }
}
