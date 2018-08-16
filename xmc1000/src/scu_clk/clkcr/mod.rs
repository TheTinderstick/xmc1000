#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKCR {
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
pub struct FDIVR {
    bits: u8,
}
impl FDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `IDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDIVR {
    #[doc = "Divider is bypassed."]
    VALUE1,
    #[doc = "1; MCLK = 32 MHz"]
    VALUE2,
    #[doc = "2; MCLK = 16 MHz"]
    VALUE3,
    #[doc = "3; MCLK = 10.67 MHz"]
    VALUE4,
    #[doc = "4; MCLK = 8 MHz"]
    VALUE5,
    #[doc = "254; MCLK = 126 kHz"]
    VALUE6,
    #[doc = "255; MCLK = 125.5 kHz"]
    VALUE7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDIVR::VALUE1 => 0,
            IDIVR::VALUE2 => 1,
            IDIVR::VALUE3 => 2,
            IDIVR::VALUE4 => 3,
            IDIVR::VALUE5 => 4,
            IDIVR::VALUE6 => 254,
            IDIVR::VALUE7 => 255,
            IDIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDIVR {
        match value {
            0 => IDIVR::VALUE1,
            1 => IDIVR::VALUE2,
            2 => IDIVR::VALUE3,
            3 => IDIVR::VALUE4,
            4 => IDIVR::VALUE5,
            254 => IDIVR::VALUE6,
            255 => IDIVR::VALUE7,
            i => IDIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IDIVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IDIVR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == IDIVR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == IDIVR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == IDIVR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == IDIVR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == IDIVR::VALUE7
    }
}
#[doc = "Possible values of the field `PCLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLKSELR {
    #[doc = "PCLK = MCLK"]
    VALUE1,
    #[doc = "PCLK = 2 x MCLK"]
    VALUE2,
}
impl PCLKSELR {
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
            PCLKSELR::VALUE1 => false,
            PCLKSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCLKSELR {
        match value {
            false => PCLKSELR::VALUE1,
            true => PCLKSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PCLKSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PCLKSELR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct RTCCLKSELR {
    bits: u8,
}
impl RTCCLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CNTADJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTADJR {
    #[doc = "1 clock cycles of the DCO1, 64MHz clock"]
    VALUE1,
    #[doc = "2 clock cycles of the DCO1, 64MHz clock"]
    VALUE2,
    #[doc = "3 clock cycles of the DCO1, 64MHz clock"]
    VALUE3,
    #[doc = "4 clock cycles of the DCO1, 64MHz clock"]
    VALUE4,
    #[doc = "5 clock cycles of the DCO1, 64MHz clock"]
    VALUE5,
    #[doc = "1023 clock cycles of the DCO1, 64MHz clock"]
    VALUE6,
    #[doc = "1024 clock cycles of the DCO1, 64MHz clock"]
    VALUE7,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl CNTADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            CNTADJR::VALUE1 => 0,
            CNTADJR::VALUE2 => 1,
            CNTADJR::VALUE3 => 2,
            CNTADJR::VALUE4 => 3,
            CNTADJR::VALUE5 => 4,
            CNTADJR::VALUE6 => 1022,
            CNTADJR::VALUE7 => 1023,
            CNTADJR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> CNTADJR {
        match value {
            0 => CNTADJR::VALUE1,
            1 => CNTADJR::VALUE2,
            2 => CNTADJR::VALUE3,
            3 => CNTADJR::VALUE4,
            4 => CNTADJR::VALUE5,
            1022 => CNTADJR::VALUE6,
            1023 => CNTADJR::VALUE7,
            i => CNTADJR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CNTADJR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CNTADJR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CNTADJR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CNTADJR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == CNTADJR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == CNTADJR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == CNTADJR::VALUE7
    }
}
#[doc = "Possible values of the field `VDDC2LOW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDC2LOWR {
    #[doc = "VDDC is not too low and the fractional divider input clock is running at the targeted frequency"]
    VALUE1,
    #[doc = "VDDC is too low and the fractional divider input clock is not running at the targeted frequency"]
    VALUE2,
}
impl VDDC2LOWR {
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
            VDDC2LOWR::VALUE1 => false,
            VDDC2LOWR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VDDC2LOWR {
        match value {
            false => VDDC2LOWR::VALUE1,
            true => VDDC2LOWR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VDDC2LOWR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VDDC2LOWR::VALUE2
    }
}
#[doc = "Possible values of the field `VDDC2HIGH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDC2HIGHR {
    #[doc = "VDDC is not too high"]
    VALUE1,
    #[doc = "VDDC is too high"]
    VALUE2,
}
impl VDDC2HIGHR {
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
            VDDC2HIGHR::VALUE1 => false,
            VDDC2HIGHR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VDDC2HIGHR {
        match value {
            false => VDDC2HIGHR::VALUE1,
            true => VDDC2HIGHR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VDDC2HIGHR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VDDC2HIGHR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _FDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _FDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDIV`"]
pub enum IDIVW {
    #[doc = "Divider is bypassed."]
    VALUE1,
    #[doc = "1; MCLK = 32 MHz"]
    VALUE2,
    #[doc = "2; MCLK = 16 MHz"]
    VALUE3,
    #[doc = "3; MCLK = 10.67 MHz"]
    VALUE4,
    #[doc = "4; MCLK = 8 MHz"]
    VALUE5,
    #[doc = "254; MCLK = 126 kHz"]
    VALUE6,
    #[doc = "255; MCLK = 125.5 kHz"]
    VALUE7,
}
impl IDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDIVW::VALUE1 => 0,
            IDIVW::VALUE2 => 1,
            IDIVW::VALUE3 => 2,
            IDIVW::VALUE4 => 3,
            IDIVW::VALUE5 => 4,
            IDIVW::VALUE6 => 254,
            IDIVW::VALUE7 => 255,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _IDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divider is bypassed."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IDIVW::VALUE1)
    }
    #[doc = "1; MCLK = 32 MHz"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IDIVW::VALUE2)
    }
    #[doc = "2; MCLK = 16 MHz"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(IDIVW::VALUE3)
    }
    #[doc = "3; MCLK = 10.67 MHz"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(IDIVW::VALUE4)
    }
    #[doc = "4; MCLK = 8 MHz"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(IDIVW::VALUE5)
    }
    #[doc = "254; MCLK = 126 kHz"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(IDIVW::VALUE6)
    }
    #[doc = "255; MCLK = 125.5 kHz"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(IDIVW::VALUE7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCLKSEL`"]
pub enum PCLKSELW {
    #[doc = "PCLK = MCLK"]
    VALUE1,
    #[doc = "PCLK = 2 x MCLK"]
    VALUE2,
}
impl PCLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCLKSELW::VALUE1 => false,
            PCLKSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PCLK = MCLK"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCLKSELW::VALUE1)
    }
    #[doc = "PCLK = 2 x MCLK"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCLKSELW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _RTCCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCCLKSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNTADJ`"]
pub enum CNTADJW {
    #[doc = "1 clock cycles of the DCO1, 64MHz clock"]
    VALUE1,
    #[doc = "2 clock cycles of the DCO1, 64MHz clock"]
    VALUE2,
    #[doc = "3 clock cycles of the DCO1, 64MHz clock"]
    VALUE3,
    #[doc = "4 clock cycles of the DCO1, 64MHz clock"]
    VALUE4,
    #[doc = "5 clock cycles of the DCO1, 64MHz clock"]
    VALUE5,
    #[doc = "1023 clock cycles of the DCO1, 64MHz clock"]
    VALUE6,
    #[doc = "1024 clock cycles of the DCO1, 64MHz clock"]
    VALUE7,
}
impl CNTADJW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            CNTADJW::VALUE1 => 0,
            CNTADJW::VALUE2 => 1,
            CNTADJW::VALUE3 => 2,
            CNTADJW::VALUE4 => 3,
            CNTADJW::VALUE5 => 4,
            CNTADJW::VALUE6 => 1022,
            CNTADJW::VALUE7 => 1023,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNTADJW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTADJW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNTADJW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 clock cycles of the DCO1, 64MHz clock"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CNTADJW::VALUE1)
    }
    #[doc = "2 clock cycles of the DCO1, 64MHz clock"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CNTADJW::VALUE2)
    }
    #[doc = "3 clock cycles of the DCO1, 64MHz clock"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CNTADJW::VALUE3)
    }
    #[doc = "4 clock cycles of the DCO1, 64MHz clock"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CNTADJW::VALUE4)
    }
    #[doc = "5 clock cycles of the DCO1, 64MHz clock"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(CNTADJW::VALUE5)
    }
    #[doc = "1023 clock cycles of the DCO1, 64MHz clock"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(CNTADJW::VALUE6)
    }
    #[doc = "1024 clock cycles of the DCO1, 64MHz clock"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(CNTADJW::VALUE7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:7 - Fractional Divider Selection"]
    #[inline]
    pub fn fdiv(&self) -> FDIVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FDIVR { bits }
    }
    #[doc = "Bits 8:15 - Divider Selection"]
    #[inline]
    pub fn idiv(&self) -> IDIVR {
        IDIVR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - PCLK Clock Select"]
    #[inline]
    pub fn pclksel(&self) -> PCLKSELR {
        PCLKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 17:19 - RTC Clock Select"]
    #[inline]
    pub fn rtcclksel(&self) -> RTCCLKSELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RTCCLKSELR { bits }
    }
    #[doc = "Bits 20:29 - Counter Adjustment"]
    #[inline]
    pub fn cntadj(&self) -> CNTADJR {
        CNTADJR::_from({
            const MASK: u16 = 1023;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bit 30 - VDDC too low"]
    #[inline]
    pub fn vddc2low(&self) -> VDDC2LOWR {
        VDDC2LOWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - VDDC too high"]
    #[inline]
    pub fn vddc2high(&self) -> VDDC2HIGHR {
        VDDC2HIGHR::_from({
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
        W { bits: 1072694272 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Fractional Divider Selection"]
    #[inline]
    pub fn fdiv(&mut self) -> _FDIVW {
        _FDIVW { w: self }
    }
    #[doc = "Bits 8:15 - Divider Selection"]
    #[inline]
    pub fn idiv(&mut self) -> _IDIVW {
        _IDIVW { w: self }
    }
    #[doc = "Bit 16 - PCLK Clock Select"]
    #[inline]
    pub fn pclksel(&mut self) -> _PCLKSELW {
        _PCLKSELW { w: self }
    }
    #[doc = "Bits 17:19 - RTC Clock Select"]
    #[inline]
    pub fn rtcclksel(&mut self) -> _RTCCLKSELW {
        _RTCCLKSELW { w: self }
    }
    #[doc = "Bits 20:29 - Counter Adjustment"]
    #[inline]
    pub fn cntadj(&mut self) -> _CNTADJW {
        _CNTADJW { w: self }
    }
}
