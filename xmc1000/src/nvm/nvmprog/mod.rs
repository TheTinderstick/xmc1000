#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::NVMPROG {
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
#[doc = "Possible values of the field `RSTECC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTECCR {
    #[doc = "No action."]
    VALUE1,
    #[doc = "Reset of .ECCxREAD and NVMSTATUS.WRPERR."]
    VALUE2,
}
impl RSTECCR {
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
            RSTECCR::VALUE1 => false,
            RSTECCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTECCR {
        match value {
            false => RSTECCR::VALUE1,
            true => RSTECCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RSTECCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RSTECCR::VALUE2
    }
}
#[doc = "Possible values of the field `RSTVERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTVERRR {
    #[doc = "No action."]
    VALUE1,
    #[doc = "Reset of .VERR."]
    VALUE2,
}
impl RSTVERRR {
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
            RSTVERRR::VALUE1 => false,
            RSTVERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTVERRR {
        match value {
            false => RSTVERRR::VALUE1,
            true => RSTVERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RSTVERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RSTVERRR::VALUE2
    }
}
#[doc = "Possible values of the field `ACTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIONR {
    #[doc = "Idle state, no action triggered. Writing 0x00 exits current mode."]
    VALUE1,
    #[doc = "Start one-shot write operation with automatic verify."]
    VALUE2,
    #[doc = "Start one-shot write operation without verify."]
    VALUE3,
    #[doc = "Start continuous write operation with automatic verify of every write."]
    VALUE4,
    #[doc = "Start continuous write operation without verify."]
    VALUE5,
    #[doc = "Start one-shot page erase operation."]
    VALUE6,
    #[doc = "Start continuous page erase operation."]
    VALUE7,
    #[doc = "Start one-shot verify-only: Written data is compared to array content."]
    VALUE8,
    #[doc = "Start continuous verify-only: Written data is compared to array content."]
    VALUE9,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ACTIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACTIONR::VALUE1 => 0,
            ACTIONR::VALUE2 => 81,
            ACTIONR::VALUE3 => 145,
            ACTIONR::VALUE4 => 97,
            ACTIONR::VALUE5 => 161,
            ACTIONR::VALUE6 => 146,
            ACTIONR::VALUE7 => 162,
            ACTIONR::VALUE8 => 208,
            ACTIONR::VALUE9 => 224,
            ACTIONR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACTIONR {
        match value {
            0 => ACTIONR::VALUE1,
            81 => ACTIONR::VALUE2,
            145 => ACTIONR::VALUE3,
            97 => ACTIONR::VALUE4,
            161 => ACTIONR::VALUE5,
            146 => ACTIONR::VALUE6,
            162 => ACTIONR::VALUE7,
            208 => ACTIONR::VALUE8,
            224 => ACTIONR::VALUE9,
            i => ACTIONR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ACTIONR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ACTIONR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ACTIONR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ACTIONR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == ACTIONR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == ACTIONR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == ACTIONR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == ACTIONR::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == ACTIONR::VALUE9
    }
}
#[doc = "Values that can be written to the field `RSTECC`"]
pub enum RSTECCW {
    #[doc = "No action."]
    VALUE1,
    #[doc = "Reset of .ECCxREAD and NVMSTATUS.WRPERR."]
    VALUE2,
}
impl RSTECCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTECCW::VALUE1 => false,
            RSTECCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTECCW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTECCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTECCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSTECCW::VALUE1)
    }
    #[doc = "Reset of .ECCxREAD and NVMSTATUS.WRPERR."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSTECCW::VALUE2)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSTVERR`"]
pub enum RSTVERRW {
    #[doc = "No action."]
    VALUE1,
    #[doc = "Reset of .VERR."]
    VALUE2,
}
impl RSTVERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTVERRW::VALUE1 => false,
            RSTVERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTVERRW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTVERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTVERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSTVERRW::VALUE1)
    }
    #[doc = "Reset of .VERR."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSTVERRW::VALUE2)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACTION`"]
pub enum ACTIONW {
    #[doc = "Idle state, no action triggered. Writing 0x00 exits current mode."]
    VALUE1,
    #[doc = "Start one-shot write operation with automatic verify."]
    VALUE2,
    #[doc = "Start one-shot write operation without verify."]
    VALUE3,
    #[doc = "Start continuous write operation with automatic verify of every write."]
    VALUE4,
    #[doc = "Start continuous write operation without verify."]
    VALUE5,
    #[doc = "Start one-shot page erase operation."]
    VALUE6,
    #[doc = "Start continuous page erase operation."]
    VALUE7,
    #[doc = "Start one-shot verify-only: Written data is compared to array content."]
    VALUE8,
    #[doc = "Start continuous verify-only: Written data is compared to array content."]
    VALUE9,
}
impl ACTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACTIONW::VALUE1 => 0,
            ACTIONW::VALUE2 => 81,
            ACTIONW::VALUE3 => 145,
            ACTIONW::VALUE4 => 97,
            ACTIONW::VALUE5 => 161,
            ACTIONW::VALUE6 => 146,
            ACTIONW::VALUE7 => 162,
            ACTIONW::VALUE8 => 208,
            ACTIONW::VALUE9 => 224,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACTIONW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Idle state, no action triggered. Writing 0x00 exits current mode."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACTIONW::VALUE1)
    }
    #[doc = "Start one-shot write operation with automatic verify."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACTIONW::VALUE2)
    }
    #[doc = "Start one-shot write operation without verify."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ACTIONW::VALUE3)
    }
    #[doc = "Start continuous write operation with automatic verify of every write."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(ACTIONW::VALUE4)
    }
    #[doc = "Start continuous write operation without verify."]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(ACTIONW::VALUE5)
    }
    #[doc = "Start one-shot page erase operation."]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(ACTIONW::VALUE6)
    }
    #[doc = "Start continuous page erase operation."]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(ACTIONW::VALUE7)
    }
    #[doc = "Start one-shot verify-only: Written data is compared to array content."]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(ACTIONW::VALUE8)
    }
    #[doc = "Start continuous verify-only: Written data is compared to array content."]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(ACTIONW::VALUE9)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bit 13 - Reset ECC"]
    #[inline]
    pub fn rstecc(&self) -> RSTECCR {
        RSTECCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 12 - Reset Verify Error"]
    #[inline]
    pub fn rstverr(&self) -> RSTVERRR {
        RSTVERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 0:7 - ACTION: [VERIFY, ONE_SHOT, OPTYPE]"]
    #[inline]
    pub fn action(&self) -> ACTIONR {
        ACTIONR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 13 - Reset ECC"]
    #[inline]
    pub fn rstecc(&mut self) -> _RSTECCW {
        _RSTECCW { w: self }
    }
    #[doc = "Bit 12 - Reset Verify Error"]
    #[inline]
    pub fn rstverr(&mut self) -> _RSTVERRW {
        _RSTVERRW { w: self }
    }
    #[doc = "Bits 0:7 - ACTION: [VERIFY, ONE_SHOT, OPTYPE]"]
    #[inline]
    pub fn action(&mut self) -> _ACTIONW {
        _ACTIONW { w: self }
    }
}
