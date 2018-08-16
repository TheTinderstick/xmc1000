#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICSR {
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
#[doc = "Possible values of the field `VECTACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTACTIVER {
    #[doc = "Thread mode"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VECTACTIVER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VECTACTIVER::VALUE1 => 0,
            VECTACTIVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VECTACTIVER {
        match value {
            0 => VECTACTIVER::VALUE1,
            i => VECTACTIVER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VECTACTIVER::VALUE1
    }
}
#[doc = "Possible values of the field `VECTPENDING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTPENDINGR {
    #[doc = "No pending exceptions"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VECTPENDINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VECTPENDINGR::VALUE1 => 0,
            VECTPENDINGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VECTPENDINGR {
        match value {
            0 => VECTPENDINGR::VALUE1,
            i => VECTPENDINGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VECTPENDINGR::VALUE1
    }
}
#[doc = "Possible values of the field `ISRPENDING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISRPENDINGR {
    #[doc = "Interrupt not pending"]
    VALUE1,
    #[doc = "Interrupt pending."]
    VALUE2,
}
impl ISRPENDINGR {
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
            ISRPENDINGR::VALUE1 => false,
            ISRPENDINGR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISRPENDINGR {
        match value {
            false => ISRPENDINGR::VALUE1,
            true => ISRPENDINGR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ISRPENDINGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ISRPENDINGR::VALUE2
    }
}
#[doc = "Possible values of the field `PENDSTSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTSETR {
    #[doc = "SysTick exception is not pending"]
    VALUE1,
    #[doc = "SysTick exception is pending."]
    VALUE2,
}
impl PENDSTSETR {
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
            PENDSTSETR::VALUE1 => false,
            PENDSTSETR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PENDSTSETR {
        match value {
            false => PENDSTSETR::VALUE1,
            true => PENDSTSETR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PENDSTSETR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PENDSTSETR::VALUE2
    }
}
#[doc = "Possible values of the field `PENDSVSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVSETR {
    #[doc = "PendSV exception is not pending."]
    VALUE1,
    #[doc = "PendSV excepton is pending."]
    VALUE2,
}
impl PENDSVSETR {
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
            PENDSVSETR::VALUE1 => false,
            PENDSVSETR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PENDSVSETR {
        match value {
            false => PENDSVSETR::VALUE1,
            true => PENDSVSETR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PENDSVSETR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PENDSVSETR::VALUE2
    }
}
#[doc = "Values that can be written to the field `PENDSTCLR`"]
pub enum PENDSTCLRW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "removes the pending state from the SysTick exception."]
    VALUE2,
}
impl PENDSTCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSTCLRW::VALUE1 => false,
            PENDSTCLRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSTCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSTCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PENDSTCLRW::VALUE1)
    }
    #[doc = "removes the pending state from the SysTick exception."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PENDSTCLRW::VALUE2)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PENDSTSET`"]
pub enum PENDSTSETW {
    #[doc = "SysTick exception is not pending"]
    VALUE1,
    #[doc = "SysTick exception is pending."]
    VALUE2,
}
impl PENDSTSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSTSETW::VALUE1 => false,
            PENDSTSETW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSTSETW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSTSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSTSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SysTick exception is not pending"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PENDSTSETW::VALUE1)
    }
    #[doc = "SysTick exception is pending."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PENDSTSETW::VALUE2)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PENDSVCLR`"]
pub enum PENDSVCLRW {
    #[doc = "Do not clear."]
    VALUE1,
    #[doc = "Removes pending state from PendSV exception."]
    VALUE2,
}
impl PENDSVCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSVCLRW::VALUE1 => false,
            PENDSVCLRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSVCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSVCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSVCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not clear."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PENDSVCLRW::VALUE1)
    }
    #[doc = "Removes pending state from PendSV exception."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PENDSVCLRW::VALUE2)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PENDSVSET`"]
pub enum PENDSVSETW {
    #[doc = "PendSV exception is not pending."]
    VALUE1,
    #[doc = "PendSV excepton is pending."]
    VALUE2,
}
impl PENDSVSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSVSETW::VALUE1 => false,
            PENDSVSETW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSVSETW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSVSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSVSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PendSV exception is not pending."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PENDSVSETW::VALUE1)
    }
    #[doc = "PendSV excepton is pending."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PENDSVSETW::VALUE2)
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
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:5 - Active Exception Number"]
    #[inline]
    pub fn vectactive(&self) -> VECTACTIVER {
        VECTACTIVER::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:17 - Pending Exception Number"]
    #[inline]
    pub fn vectpending(&self) -> VECTPENDINGR {
        VECTPENDINGR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - Interrupt Pending Flag"]
    #[inline]
    pub fn isrpending(&self) -> ISRPENDINGR {
        ISRPENDINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - SysTick Exception Set-pending"]
    #[inline]
    pub fn pendstset(&self) -> PENDSTSETR {
        PENDSTSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - PendSV Set Pending"]
    #[inline]
    pub fn pendsvset(&self) -> PENDSVSETR {
        PENDSVSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 25 - SysTick Exception Clear-pending"]
    #[inline]
    pub fn pendstclr(&mut self) -> _PENDSTCLRW {
        _PENDSTCLRW { w: self }
    }
    #[doc = "Bit 26 - SysTick Exception Set-pending"]
    #[inline]
    pub fn pendstset(&mut self) -> _PENDSTSETW {
        _PENDSTSETW { w: self }
    }
    #[doc = "Bit 27 - PendSV Clear Pending"]
    #[inline]
    pub fn pendsvclr(&mut self) -> _PENDSVCLRW {
        _PENDSVCLRW { w: self }
    }
    #[doc = "Bit 28 - PendSV Set Pending"]
    #[inline]
    pub fn pendsvset(&mut self) -> _PENDSVSETW {
        _PENDSVSETW { w: self }
    }
}
