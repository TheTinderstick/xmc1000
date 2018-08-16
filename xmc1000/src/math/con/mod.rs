#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CON {
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
pub struct STR {
    bits: bool,
}
impl STR {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Linear Mode"]
    VALUE1,
    #[doc = "Circular Mode (default)"]
    VALUE2,
    #[doc = "Hyperbolic Mode"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::VALUE1 => 0,
            MODER::VALUE2 => 1,
            MODER::VALUE4 => 3,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::VALUE1,
            1 => MODER::VALUE2,
            3 => MODER::VALUE4,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MODER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == MODER::VALUE4
    }
}
#[doc = "Possible values of the field `ROTVEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROTVECR {
    #[doc = "Vectoring Mode (default)"]
    VALUE1,
    #[doc = "Rotation Mode"]
    VALUE2,
}
impl ROTVECR {
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
            ROTVECR::VALUE1 => false,
            ROTVECR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROTVECR {
        match value {
            false => ROTVECR::VALUE1,
            true => ROTVECR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ROTVECR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ROTVECR::VALUE2
    }
}
#[doc = "Possible values of the field `ST_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ST_MODER {
    #[doc = "Auto start of calculation after write access to X parameter data register CORDX(default)."]
    VALUE1,
    #[doc = "Start calculation only after bit ST is set"]
    VALUE2,
}
impl ST_MODER {
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
            ST_MODER::VALUE1 => false,
            ST_MODER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ST_MODER {
        match value {
            false => ST_MODER::VALUE1,
            true => ST_MODER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ST_MODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ST_MODER::VALUE2
    }
}
#[doc = "Possible values of the field `X_USIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum X_USIGNR {
    #[doc = "Signed, twos complement"]
    VALUE1,
    #[doc = "Unsigned (default)"]
    VALUE2,
}
impl X_USIGNR {
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
            X_USIGNR::VALUE1 => false,
            X_USIGNR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> X_USIGNR {
        match value {
            false => X_USIGNR::VALUE1,
            true => X_USIGNR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == X_USIGNR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == X_USIGNR::VALUE2
    }
}
#[doc = "Possible values of the field `MPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPSR {
    #[doc = "Divide by 1"]
    VALUE1,
    #[doc = "Divide by 2 (default)"]
    VALUE2,
    #[doc = "Divide by 4"]
    VALUE3,
    #[doc = "Reserved, retain the last MPS setting"]
    VALUE4,
}
impl MPSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MPSR::VALUE1 => 0,
            MPSR::VALUE2 => 1,
            MPSR::VALUE3 => 2,
            MPSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MPSR {
        match value {
            0 => MPSR::VALUE1,
            1 => MPSR::VALUE2,
            2 => MPSR::VALUE3,
            3 => MPSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MPSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MPSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == MPSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == MPSR::VALUE4
    }
}
#[doc = r" Proxy"]
pub struct _STW<'a> {
    w: &'a mut W,
}
impl<'a> _STW<'a> {
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Linear Mode"]
    VALUE1,
    #[doc = "Circular Mode (default)"]
    VALUE2,
    #[doc = "Hyperbolic Mode"]
    VALUE4,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::VALUE1 => 0,
            MODEW::VALUE2 => 1,
            MODEW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Linear Mode"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MODEW::VALUE1)
    }
    #[doc = "Circular Mode (default)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MODEW::VALUE2)
    }
    #[doc = "Hyperbolic Mode"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(MODEW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ROTVEC`"]
pub enum ROTVECW {
    #[doc = "Vectoring Mode (default)"]
    VALUE1,
    #[doc = "Rotation Mode"]
    VALUE2,
}
impl ROTVECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROTVECW::VALUE1 => false,
            ROTVECW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROTVECW<'a> {
    w: &'a mut W,
}
impl<'a> _ROTVECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROTVECW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Vectoring Mode (default)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ROTVECW::VALUE1)
    }
    #[doc = "Rotation Mode"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ROTVECW::VALUE2)
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
#[doc = "Values that can be written to the field `ST_MODE`"]
pub enum ST_MODEW {
    #[doc = "Auto start of calculation after write access to X parameter data register CORDX(default)."]
    VALUE1,
    #[doc = "Start calculation only after bit ST is set"]
    VALUE2,
}
impl ST_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ST_MODEW::VALUE1 => false,
            ST_MODEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ST_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ST_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ST_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Auto start of calculation after write access to X parameter data register CORDX(default)."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ST_MODEW::VALUE1)
    }
    #[doc = "Start calculation only after bit ST is set"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ST_MODEW::VALUE2)
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
#[doc = "Values that can be written to the field `X_USIGN`"]
pub enum X_USIGNW {
    #[doc = "Signed, twos complement"]
    VALUE1,
    #[doc = "Unsigned (default)"]
    VALUE2,
}
impl X_USIGNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            X_USIGNW::VALUE1 => false,
            X_USIGNW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _X_USIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _X_USIGNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: X_USIGNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Signed, twos complement"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(X_USIGNW::VALUE1)
    }
    #[doc = "Unsigned (default)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(X_USIGNW::VALUE2)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MPS`"]
pub enum MPSW {
    #[doc = "Divide by 1"]
    VALUE1,
    #[doc = "Divide by 2 (default)"]
    VALUE2,
    #[doc = "Divide by 4"]
    VALUE3,
    #[doc = "Reserved, retain the last MPS setting"]
    VALUE4,
}
impl MPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MPSW::VALUE1 => 0,
            MPSW::VALUE2 => 1,
            MPSW::VALUE3 => 2,
            MPSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPSW<'a> {
    w: &'a mut W,
}
impl<'a> _MPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MPSW::VALUE1)
    }
    #[doc = "Divide by 2 (default)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MPSW::VALUE2)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(MPSW::VALUE3)
    }
    #[doc = "Reserved, retain the last MPS setting"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(MPSW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - Start Calculation"]
    #[inline]
    pub fn st(&self) -> STR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STR { bits }
    }
    #[doc = "Bits 1:2 - Operating Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Rotation Vectoring Selection"]
    #[inline]
    pub fn rotvec(&self) -> ROTVECR {
        ROTVECR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Start Method"]
    #[inline]
    pub fn st_mode(&self) -> ST_MODER {
        ST_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Result Data Format for X in Circular Vectoring Mode"]
    #[inline]
    pub fn x_usign(&self) -> X_USIGNR {
        X_USIGNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - X and Y Magnitude Prescaler"]
    #[inline]
    pub fn mps(&self) -> MPSR {
        MPSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 98 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Start Calculation"]
    #[inline]
    pub fn st(&mut self) -> _STW {
        _STW { w: self }
    }
    #[doc = "Bits 1:2 - Operating Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 3 - Rotation Vectoring Selection"]
    #[inline]
    pub fn rotvec(&mut self) -> _ROTVECW {
        _ROTVECW { w: self }
    }
    #[doc = "Bit 4 - Start Method"]
    #[inline]
    pub fn st_mode(&mut self) -> _ST_MODEW {
        _ST_MODEW { w: self }
    }
    #[doc = "Bit 5 - Result Data Format for X in Circular Vectoring Mode"]
    #[inline]
    pub fn x_usign(&mut self) -> _X_USIGNW {
        _X_USIGNW { w: self }
    }
    #[doc = "Bits 6:7 - X and Y Magnitude Prescaler"]
    #[inline]
    pub fn mps(&mut self) -> _MPSW {
        _MPSW { w: self }
    }
}
