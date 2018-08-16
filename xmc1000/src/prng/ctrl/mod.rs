#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `KLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KLDR {
    #[doc = "Streaming mode (default)"]
    VALUE1,
    #[doc = "Key loading mode"]
    VALUE2,
}
impl KLDR {
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
            KLDR::VALUE1 => false,
            KLDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KLDR {
        match value {
            false => KLDR::VALUE1,
            true => KLDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == KLDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == KLDR::VALUE2
    }
}
#[doc = "Possible values of the field `RDBS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDBSR {
    #[doc = "Reset state (no random data block size defined), value of PRNG_WORD is undefined."]
    VALUE1,
    #[doc = "8 bits in PRNG_WORD.RDATA[7:0]"]
    VALUE2,
    #[doc = "16 bits in PRNG_WORD.RDATA[15:0]"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RDBSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RDBSR::VALUE1 => 0,
            RDBSR::VALUE2 => 1,
            RDBSR::VALUE3 => 2,
            RDBSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RDBSR {
        match value {
            0 => RDBSR::VALUE1,
            1 => RDBSR::VALUE2,
            2 => RDBSR::VALUE3,
            i => RDBSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RDBSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RDBSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RDBSR::VALUE3
    }
}
#[doc = "Values that can be written to the field `KLD`"]
pub enum KLDW {
    #[doc = "Streaming mode (default)"]
    VALUE1,
    #[doc = "Key loading mode"]
    VALUE2,
}
impl KLDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KLDW::VALUE1 => false,
            KLDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KLDW<'a> {
    w: &'a mut W,
}
impl<'a> _KLDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KLDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Streaming mode (default)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(KLDW::VALUE1)
    }
    #[doc = "Key loading mode"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(KLDW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RDBS`"]
pub enum RDBSW {
    #[doc = "Reset state (no random data block size defined), value of PRNG_WORD is undefined."]
    VALUE1,
    #[doc = "8 bits in PRNG_WORD.RDATA[7:0]"]
    VALUE2,
    #[doc = "16 bits in PRNG_WORD.RDATA[15:0]"]
    VALUE3,
}
impl RDBSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RDBSW::VALUE1 => 0,
            RDBSW::VALUE2 => 1,
            RDBSW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDBSW<'a> {
    w: &'a mut W,
}
impl<'a> _RDBSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDBSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset state (no random data block size defined), value of PRNG_WORD is undefined."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RDBSW::VALUE1)
    }
    #[doc = "8 bits in PRNG_WORD.RDATA[7:0]"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RDBSW::VALUE2)
    }
    #[doc = "16 bits in PRNG_WORD.RDATA[15:0]"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RDBSW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
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
    #[doc = "Bit 3 - Key Load Operation Mode"]
    #[inline]
    pub fn kld(&self) -> KLDR {
        KLDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 1:2 - Random Data Block Size"]
    #[inline]
    pub fn rdbs(&self) -> RDBSR {
        RDBSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) as u8
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 3 - Key Load Operation Mode"]
    #[inline]
    pub fn kld(&mut self) -> _KLDW {
        _KLDW { w: self }
    }
    #[doc = "Bits 1:2 - Random Data Block Size"]
    #[inline]
    pub fn rdbs(&mut self) -> _RDBSW {
        _RDBSW { w: self }
    }
}
