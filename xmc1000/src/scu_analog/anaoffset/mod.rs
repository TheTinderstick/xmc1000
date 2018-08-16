#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::ANAOFFSET {
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
#[doc = "Possible values of the field `ADJL_OFFSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADJL_OFFSETR {
    #[doc = "- 3.75%, typ."]
    VALUE1,
    #[doc = "- 2.85%, typ."]
    VALUE2,
    #[doc = "0, default"]
    VALUE3,
    #[doc = "+ 0.95%, typ."]
    VALUE4,
    #[doc = "+ 3.75%, typ."]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADJL_OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADJL_OFFSETR::VALUE1 => 0,
            ADJL_OFFSETR::VALUE2 => 1,
            ADJL_OFFSETR::VALUE3 => 4,
            ADJL_OFFSETR::VALUE4 => 5,
            ADJL_OFFSETR::VALUE5 => 8,
            ADJL_OFFSETR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADJL_OFFSETR {
        match value {
            0 => ADJL_OFFSETR::VALUE1,
            1 => ADJL_OFFSETR::VALUE2,
            4 => ADJL_OFFSETR::VALUE3,
            5 => ADJL_OFFSETR::VALUE4,
            8 => ADJL_OFFSETR::VALUE5,
            i => ADJL_OFFSETR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ADJL_OFFSETR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ADJL_OFFSETR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ADJL_OFFSETR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ADJL_OFFSETR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == ADJL_OFFSETR::VALUE5
    }
}
#[doc = "Values that can be written to the field `ADJL_OFFSET`"]
pub enum ADJL_OFFSETW {
    #[doc = "- 3.75%, typ."]
    VALUE1,
    #[doc = "- 2.85%, typ."]
    VALUE2,
    #[doc = "0, default"]
    VALUE3,
    #[doc = "+ 0.95%, typ."]
    VALUE4,
    #[doc = "+ 3.75%, typ."]
    VALUE5,
}
impl ADJL_OFFSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADJL_OFFSETW::VALUE1 => 0,
            ADJL_OFFSETW::VALUE2 => 1,
            ADJL_OFFSETW::VALUE3 => 4,
            ADJL_OFFSETW::VALUE4 => 5,
            ADJL_OFFSETW::VALUE5 => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADJL_OFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _ADJL_OFFSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADJL_OFFSETW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "- 3.75%, typ."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ADJL_OFFSETW::VALUE1)
    }
    #[doc = "- 2.85%, typ."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ADJL_OFFSETW::VALUE2)
    }
    #[doc = "0, default"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ADJL_OFFSETW::VALUE3)
    }
    #[doc = "+ 0.95%, typ."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(ADJL_OFFSETW::VALUE4)
    }
    #[doc = "+ 3.75%, typ."]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(ADJL_OFFSETW::VALUE5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:3 - ADJL Offset register"]
    #[inline]
    pub fn adjl_offset(&self) -> ADJL_OFFSETR {
        ADJL_OFFSETR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - ADJL Offset register"]
    #[inline]
    pub fn adjl_offset(&mut self) -> _ADJL_OFFSETW {
        _ADJL_OFFSETW { w: self }
    }
}
