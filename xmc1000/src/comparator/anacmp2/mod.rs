#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::ANACMP2 {
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
#[doc = "Possible values of the field `CMP_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_ENR {
    #[doc = "Comparator is disabled"]
    VALUE1,
    #[doc = "Comparator is enabled"]
    VALUE2,
}
impl CMP_ENR {
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
            CMP_ENR::VALUE1 => false,
            CMP_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMP_ENR {
        match value {
            false => CMP_ENR::VALUE1,
            true => CMP_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMP_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMP_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `CMP_FLT_OFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_FLT_OFFR {
    #[doc = "filter is active"]
    VALUE1,
    #[doc = "filter is switched off (to prevent a filter delay)"]
    VALUE2,
}
impl CMP_FLT_OFFR {
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
            CMP_FLT_OFFR::VALUE1 => false,
            CMP_FLT_OFFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMP_FLT_OFFR {
        match value {
            false => CMP_FLT_OFFR::VALUE1,
            true => CMP_FLT_OFFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMP_FLT_OFFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMP_FLT_OFFR::VALUE2
    }
}
#[doc = "Possible values of the field `CMP_INV_OUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_INV_OUTR {
    #[doc = "no inversion of comparator signal"]
    VALUE1,
    #[doc = "comparator signal is inverted"]
    VALUE2,
}
impl CMP_INV_OUTR {
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
            CMP_INV_OUTR::VALUE1 => false,
            CMP_INV_OUTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMP_INV_OUTR {
        match value {
            false => CMP_INV_OUTR::VALUE1,
            true => CMP_INV_OUTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMP_INV_OUTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMP_INV_OUTR::VALUE2
    }
}
#[doc = "Possible values of the field `CMP_HYST_ADJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_HYST_ADJR {
    #[doc = "Comparator hysteresis is switched off"]
    VALUE1,
    #[doc = "Hysteresis_typ = 10mV"]
    VALUE2,
    #[doc = "Hysteresis_typ = 15mV"]
    VALUE3,
    #[doc = "Hysteresis_typ = 20mV"]
    VALUE4,
}
impl CMP_HYST_ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMP_HYST_ADJR::VALUE1 => 0,
            CMP_HYST_ADJR::VALUE2 => 1,
            CMP_HYST_ADJR::VALUE3 => 2,
            CMP_HYST_ADJR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMP_HYST_ADJR {
        match value {
            0 => CMP_HYST_ADJR::VALUE1,
            1 => CMP_HYST_ADJR::VALUE2,
            2 => CMP_HYST_ADJR::VALUE3,
            3 => CMP_HYST_ADJR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMP_HYST_ADJR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMP_HYST_ADJR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CMP_HYST_ADJR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CMP_HYST_ADJR::VALUE4
    }
}
#[doc = "Possible values of the field `ACMP2_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP2_SELR {
    #[doc = "ACMP2.INP is not connected"]
    VALUE1,
    #[doc = "ACMP2.INP is connected to ACMP1.INP"]
    VALUE2,
}
impl ACMP2_SELR {
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
            ACMP2_SELR::VALUE1 => false,
            ACMP2_SELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP2_SELR {
        match value {
            false => ACMP2_SELR::VALUE1,
            true => ACMP2_SELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ACMP2_SELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ACMP2_SELR::VALUE2
    }
}
#[doc = "Possible values of the field `CMP_OUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_OUTR {
    #[doc = "state \"Vminus > Vplus\""]
    VALUE1,
    #[doc = "state \"Vminus < Vplus\""]
    VALUE2,
}
impl CMP_OUTR {
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
            CMP_OUTR::VALUE1 => false,
            CMP_OUTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMP_OUTR {
        match value {
            false => CMP_OUTR::VALUE1,
            true => CMP_OUTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMP_OUTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMP_OUTR::VALUE2
    }
}
#[doc = "Values that can be written to the field `CMP_EN`"]
pub enum CMP_ENW {
    #[doc = "Comparator is disabled"]
    VALUE1,
    #[doc = "Comparator is enabled"]
    VALUE2,
}
impl CMP_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMP_ENW::VALUE1 => false,
            CMP_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMP_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMP_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMP_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Comparator is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMP_ENW::VALUE1)
    }
    #[doc = "Comparator is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMP_ENW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMP_FLT_OFF`"]
pub enum CMP_FLT_OFFW {
    #[doc = "filter is active"]
    VALUE1,
    #[doc = "filter is switched off (to prevent a filter delay)"]
    VALUE2,
}
impl CMP_FLT_OFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMP_FLT_OFFW::VALUE1 => false,
            CMP_FLT_OFFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMP_FLT_OFFW<'a> {
    w: &'a mut W,
}
impl<'a> _CMP_FLT_OFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMP_FLT_OFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "filter is active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMP_FLT_OFFW::VALUE1)
    }
    #[doc = "filter is switched off (to prevent a filter delay)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMP_FLT_OFFW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMP_INV_OUT`"]
pub enum CMP_INV_OUTW {
    #[doc = "no inversion of comparator signal"]
    VALUE1,
    #[doc = "comparator signal is inverted"]
    VALUE2,
}
impl CMP_INV_OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMP_INV_OUTW::VALUE1 => false,
            CMP_INV_OUTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMP_INV_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _CMP_INV_OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMP_INV_OUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no inversion of comparator signal"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMP_INV_OUTW::VALUE1)
    }
    #[doc = "comparator signal is inverted"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMP_INV_OUTW::VALUE2)
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
#[doc = "Values that can be written to the field `CMP_HYST_ADJ`"]
pub enum CMP_HYST_ADJW {
    #[doc = "Comparator hysteresis is switched off"]
    VALUE1,
    #[doc = "Hysteresis_typ = 10mV"]
    VALUE2,
    #[doc = "Hysteresis_typ = 15mV"]
    VALUE3,
    #[doc = "Hysteresis_typ = 20mV"]
    VALUE4,
}
impl CMP_HYST_ADJW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMP_HYST_ADJW::VALUE1 => 0,
            CMP_HYST_ADJW::VALUE2 => 1,
            CMP_HYST_ADJW::VALUE3 => 2,
            CMP_HYST_ADJW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMP_HYST_ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _CMP_HYST_ADJW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMP_HYST_ADJW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Comparator hysteresis is switched off"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMP_HYST_ADJW::VALUE1)
    }
    #[doc = "Hysteresis_typ = 10mV"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMP_HYST_ADJW::VALUE2)
    }
    #[doc = "Hysteresis_typ = 15mV"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMP_HYST_ADJW::VALUE3)
    }
    #[doc = "Hysteresis_typ = 20mV"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CMP_HYST_ADJW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACMP2_SEL`"]
pub enum ACMP2_SELW {
    #[doc = "ACMP2.INP is not connected"]
    VALUE1,
    #[doc = "ACMP2.INP is connected to ACMP1.INP"]
    VALUE2,
}
impl ACMP2_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP2_SELW::VALUE1 => false,
            ACMP2_SELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP2_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP2_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP2_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ACMP2.INP is not connected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACMP2_SELW::VALUE1)
    }
    #[doc = "ACMP2.INP is connected to ACMP1.INP"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACMP2_SELW::VALUE2)
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
        const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - Comparator enable"]
    #[inline]
    pub fn cmp_en(&self) -> CMP_ENR {
        CMP_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Disables comparator filter"]
    #[inline]
    pub fn cmp_flt_off(&self) -> CMP_FLT_OFFR {
        CMP_FLT_OFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Inverted Comparator output"]
    #[inline]
    pub fn cmp_inv_out(&self) -> CMP_INV_OUTR {
        CMP_INV_OUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 4:5 - Comparator hysteresis adjust"]
    #[inline]
    pub fn cmp_hyst_adj(&self) -> CMP_HYST_ADJR {
        CMP_HYST_ADJR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 6 - Connect ACMP2.INP to ACMP1.INP"]
    #[inline]
    pub fn acmp2_sel(&self) -> ACMP2_SELR {
        ACMP2_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 15 - Comparator output monitor bit"]
    #[inline]
    pub fn cmp_out(&self) -> CMP_OUTR {
        CMP_OUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Comparator enable"]
    #[inline]
    pub fn cmp_en(&mut self) -> _CMP_ENW {
        _CMP_ENW { w: self }
    }
    #[doc = "Bit 1 - Disables comparator filter"]
    #[inline]
    pub fn cmp_flt_off(&mut self) -> _CMP_FLT_OFFW {
        _CMP_FLT_OFFW { w: self }
    }
    #[doc = "Bit 3 - Inverted Comparator output"]
    #[inline]
    pub fn cmp_inv_out(&mut self) -> _CMP_INV_OUTW {
        _CMP_INV_OUTW { w: self }
    }
    #[doc = "Bits 4:5 - Comparator hysteresis adjust"]
    #[inline]
    pub fn cmp_hyst_adj(&mut self) -> _CMP_HYST_ADJW {
        _CMP_HYST_ADJW { w: self }
    }
    #[doc = "Bit 6 - Connect ACMP2.INP to ACMP1.INP"]
    #[inline]
    pub fn acmp2_sel(&mut self) -> _ACMP2_SELW {
        _ACMP2_SELW { w: self }
    }
}
