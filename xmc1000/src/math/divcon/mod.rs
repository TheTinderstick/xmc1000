#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DIVCON {
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
#[doc = "Possible values of the field `ST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STR {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Start the division operation when STMODE=1#"]
    VALUE2,
}
impl STR {
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
            STR::VALUE1 => false,
            STR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STR {
        match value {
            false => STR::VALUE1,
            true => STR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STR::VALUE2
    }
}
#[doc = "Possible values of the field `STMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STMODER {
    #[doc = "Calculation is automatically started with a write to DVS register"]
    VALUE1,
    #[doc = "Calculation is started by setting the ST bit to 1"]
    VALUE2,
}
impl STMODER {
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
            STMODER::VALUE1 => false,
            STMODER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STMODER {
        match value {
            false => STMODER::VALUE1,
            true => STMODER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STMODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STMODER::VALUE2
    }
}
#[doc = "Possible values of the field `USIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIGNR {
    #[doc = "Signed division is selected"]
    VALUE1,
    #[doc = "Unsigned division is selected"]
    VALUE2,
}
impl USIGNR {
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
            USIGNR::VALUE1 => false,
            USIGNR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USIGNR {
        match value {
            false => USIGNR::VALUE1,
            true => USIGNR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USIGNR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == USIGNR::VALUE2
    }
}
#[doc = "Possible values of the field `DIVMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVMODER {
    #[doc = "32-bit divide by 32-bit"]
    VALUE1,
    #[doc = "32-bit divide by 16-bit"]
    VALUE2,
    #[doc = "16-bit divide by 16-bit"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DIVMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVMODER::VALUE1 => 0,
            DIVMODER::VALUE2 => 1,
            DIVMODER::VALUE3 => 2,
            DIVMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVMODER {
        match value {
            0 => DIVMODER::VALUE1,
            1 => DIVMODER::VALUE2,
            2 => DIVMODER::VALUE3,
            i => DIVMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIVMODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIVMODER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DIVMODER::VALUE3
    }
}
#[doc = "Possible values of the field `QSDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QSDIRR {
    #[doc = "Left shift"]
    VALUE1,
    #[doc = "Right shift"]
    VALUE2,
}
impl QSDIRR {
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
            QSDIRR::VALUE1 => false,
            QSDIRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QSDIRR {
        match value {
            false => QSDIRR::VALUE1,
            true => QSDIRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == QSDIRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == QSDIRR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct QSCNTR {
    bits: u8,
}
impl QSCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DVDSLCR {
    bits: u8,
}
impl DVDSLCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DVSSRCR {
    bits: u8,
}
impl DVSSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `ST`"]
pub enum STW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Start the division operation when STMODE=1#"]
    VALUE2,
}
impl STW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STW::VALUE1 => false,
            STW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STW<'a> {
    w: &'a mut W,
}
impl<'a> _STW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STW::VALUE1)
    }
    #[doc = "Start the division operation when STMODE=1#"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STW::VALUE2)
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
#[doc = "Values that can be written to the field `STMODE`"]
pub enum STMODEW {
    #[doc = "Calculation is automatically started with a write to DVS register"]
    VALUE1,
    #[doc = "Calculation is started by setting the ST bit to 1"]
    VALUE2,
}
impl STMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STMODEW::VALUE1 => false,
            STMODEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _STMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Calculation is automatically started with a write to DVS register"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STMODEW::VALUE1)
    }
    #[doc = "Calculation is started by setting the ST bit to 1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STMODEW::VALUE2)
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
#[doc = "Values that can be written to the field `USIGN`"]
pub enum USIGNW {
    #[doc = "Signed division is selected"]
    VALUE1,
    #[doc = "Unsigned division is selected"]
    VALUE2,
}
impl USIGNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USIGNW::VALUE1 => false,
            USIGNW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _USIGNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USIGNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Signed division is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIGNW::VALUE1)
    }
    #[doc = "Unsigned division is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIGNW::VALUE2)
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
#[doc = "Values that can be written to the field `DIVMODE`"]
pub enum DIVMODEW {
    #[doc = "32-bit divide by 32-bit"]
    VALUE1,
    #[doc = "32-bit divide by 16-bit"]
    VALUE2,
    #[doc = "16-bit divide by 16-bit"]
    VALUE3,
}
impl DIVMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVMODEW::VALUE1 => 0,
            DIVMODEW::VALUE2 => 1,
            DIVMODEW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "32-bit divide by 32-bit"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVMODEW::VALUE1)
    }
    #[doc = "32-bit divide by 16-bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVMODEW::VALUE2)
    }
    #[doc = "16-bit divide by 16-bit"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DIVMODEW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `QSDIR`"]
pub enum QSDIRW {
    #[doc = "Left shift"]
    VALUE1,
    #[doc = "Right shift"]
    VALUE2,
}
impl QSDIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QSDIRW::VALUE1 => false,
            QSDIRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QSDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _QSDIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QSDIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Left shift"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(QSDIRW::VALUE1)
    }
    #[doc = "Right shift"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(QSDIRW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _QSCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _QSCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DVDSLCW<'a> {
    w: &'a mut W,
}
impl<'a> _DVDSLCW<'a> {
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
pub struct _DVSSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _DVSSRCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Start Bit"]
    #[inline]
    pub fn st(&self) -> STR {
        STR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Start Mode"]
    #[inline]
    pub fn stmode(&self) -> STMODER {
        STMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Unsigned Division Enable"]
    #[inline]
    pub fn usign(&self) -> USIGNR {
        USIGNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:4 - Division Mode"]
    #[inline]
    pub fn divmode(&self) -> DIVMODER {
        DIVMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Quotient Shift Direction"]
    #[inline]
    pub fn qsdir(&self) -> QSDIRR {
        QSDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:12 - Quotient Shift Count"]
    #[inline]
    pub fn qscnt(&self) -> QSCNTR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        QSCNTR { bits }
    }
    #[doc = "Bits 16:20 - Dividend Shift Left Count"]
    #[inline]
    pub fn dvdslc(&self) -> DVDSLCR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DVDSLCR { bits }
    }
    #[doc = "Bits 24:28 - Divisor Shift Right Count"]
    #[inline]
    pub fn dvssrc(&self) -> DVSSRCR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DVSSRCR { bits }
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
    #[doc = "Bit 0 - Start Bit"]
    #[inline]
    pub fn st(&mut self) -> _STW {
        _STW { w: self }
    }
    #[doc = "Bit 1 - Start Mode"]
    #[inline]
    pub fn stmode(&mut self) -> _STMODEW {
        _STMODEW { w: self }
    }
    #[doc = "Bit 2 - Unsigned Division Enable"]
    #[inline]
    pub fn usign(&mut self) -> _USIGNW {
        _USIGNW { w: self }
    }
    #[doc = "Bits 3:4 - Division Mode"]
    #[inline]
    pub fn divmode(&mut self) -> _DIVMODEW {
        _DIVMODEW { w: self }
    }
    #[doc = "Bit 15 - Quotient Shift Direction"]
    #[inline]
    pub fn qsdir(&mut self) -> _QSDIRW {
        _QSDIRW { w: self }
    }
    #[doc = "Bits 8:12 - Quotient Shift Count"]
    #[inline]
    pub fn qscnt(&mut self) -> _QSCNTW {
        _QSCNTW { w: self }
    }
    #[doc = "Bits 16:20 - Dividend Shift Left Count"]
    #[inline]
    pub fn dvdslc(&mut self) -> _DVDSLCW {
        _DVDSLCW { w: self }
    }
    #[doc = "Bits 24:28 - Divisor Shift Right Count"]
    #[inline]
    pub fn dvssrc(&mut self) -> _DVSSRCW {
        _DVSSRCW { w: self }
    }
}
