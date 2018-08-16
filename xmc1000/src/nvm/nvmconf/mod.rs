#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::NVMCONF {
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
#[doc = "Possible values of the field `NVM_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVM_ONR {
    #[doc = "NVM is switched to or stays in sleep mode."]
    VALUE1,
    #[doc = "NVM is switched to or stays in normal mode."]
    VALUE2,
}
impl NVM_ONR {
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
            NVM_ONR::VALUE1 => false,
            NVM_ONR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NVM_ONR {
        match value {
            false => NVM_ONR::VALUE1,
            true => NVM_ONR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NVM_ONR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NVM_ONR::VALUE2
    }
}
#[doc = "Possible values of the field `INT_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_ONR {
    #[doc = "No NVM ready interrupts are generated."]
    VALUE1,
    #[doc = "NVM ready interrupts are generated."]
    VALUE2,
}
impl INT_ONR {
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
            INT_ONR::VALUE1 => false,
            INT_ONR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT_ONR {
        match value {
            false => INT_ONR::VALUE1,
            true => INT_ONR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INT_ONR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INT_ONR::VALUE2
    }
}
#[doc = "Possible values of the field `WS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSR {
    #[doc = "0 fixed wait states."]
    VALUE1,
    #[doc = "1 fixed wait state."]
    VALUE2,
}
impl WSR {
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
            WSR::VALUE1 => false,
            WSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WSR {
        match value {
            false => WSR::VALUE1,
            true => WSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WSR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct SECPROTR {
    bits: u8,
}
impl SECPROTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `HRLEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRLEVR {
    #[doc = "Normal read"]
    VALUE1,
    #[doc = "Hardread written"]
    VALUE2,
    #[doc = "Hardread erased"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HRLEVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HRLEVR::VALUE1 => 0,
            HRLEVR::VALUE2 => 1,
            HRLEVR::VALUE3 => 2,
            HRLEVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HRLEVR {
        match value {
            0 => HRLEVR::VALUE1,
            1 => HRLEVR::VALUE2,
            2 => HRLEVR::VALUE3,
            i => HRLEVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HRLEVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HRLEVR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == HRLEVR::VALUE3
    }
}
#[doc = "Values that can be written to the field `NVM_ON`"]
pub enum NVM_ONW {
    #[doc = "NVM is switched to or stays in sleep mode."]
    VALUE1,
    #[doc = "NVM is switched to or stays in normal mode."]
    VALUE2,
}
impl NVM_ONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NVM_ONW::VALUE1 => false,
            NVM_ONW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NVM_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _NVM_ONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NVM_ONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "NVM is switched to or stays in sleep mode."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(NVM_ONW::VALUE1)
    }
    #[doc = "NVM is switched to or stays in normal mode."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(NVM_ONW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INT_ON`"]
pub enum INT_ONW {
    #[doc = "No NVM ready interrupts are generated."]
    VALUE1,
    #[doc = "NVM ready interrupts are generated."]
    VALUE2,
}
impl INT_ONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT_ONW::VALUE1 => false,
            INT_ONW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_ONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT_ONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No NVM ready interrupts are generated."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(INT_ONW::VALUE1)
    }
    #[doc = "NVM ready interrupts are generated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(INT_ONW::VALUE2)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WS`"]
pub enum WSW {
    #[doc = "0 fixed wait states."]
    VALUE1,
    #[doc = "1 fixed wait state."]
    VALUE2,
}
impl WSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WSW::VALUE1 => false,
            WSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WSW<'a> {
    w: &'a mut W,
}
impl<'a> _WSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "0 fixed wait states."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WSW::VALUE1)
    }
    #[doc = "1 fixed wait state."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WSW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _SECPROTW<'a> {
    w: &'a mut W,
}
impl<'a> _SECPROTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HRLEV`"]
pub enum HRLEVW {
    #[doc = "Normal read"]
    VALUE1,
    #[doc = "Hardread written"]
    VALUE2,
    #[doc = "Hardread erased"]
    VALUE3,
}
impl HRLEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HRLEVW::VALUE1 => 0,
            HRLEVW::VALUE2 => 1,
            HRLEVW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRLEVW<'a> {
    w: &'a mut W,
}
impl<'a> _HRLEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRLEVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal read"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HRLEVW::VALUE1)
    }
    #[doc = "Hardread written"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HRLEVW::VALUE2)
    }
    #[doc = "Hardread erased"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(HRLEVW::VALUE3)
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
    #[doc = "Bit 15 - NVM On"]
    #[inline]
    pub fn nvm_on(&self) -> NVM_ONR {
        NVM_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 14 - Interrupt On"]
    #[inline]
    pub fn int_on(&self) -> INT_ONR {
        INT_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 12 - Number of fixed Wait States"]
    #[inline]
    pub fn ws(&self) -> WSR {
        WSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 4:11 - Sector Protection"]
    #[inline]
    pub fn secprot(&self) -> SECPROTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        SECPROTR { bits }
    }
    #[doc = "Bits 1:2 - Hardread Level"]
    #[inline]
    pub fn hrlev(&self) -> HRLEVR {
        HRLEVR::_from({
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
        W { bits: 36864 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 15 - NVM On"]
    #[inline]
    pub fn nvm_on(&mut self) -> _NVM_ONW {
        _NVM_ONW { w: self }
    }
    #[doc = "Bit 14 - Interrupt On"]
    #[inline]
    pub fn int_on(&mut self) -> _INT_ONW {
        _INT_ONW { w: self }
    }
    #[doc = "Bit 12 - Number of fixed Wait States"]
    #[inline]
    pub fn ws(&mut self) -> _WSW {
        _WSW { w: self }
    }
    #[doc = "Bits 4:11 - Sector Protection"]
    #[inline]
    pub fn secprot(&mut self) -> _SECPROTW {
        _SECPROTW { w: self }
    }
    #[doc = "Bits 1:2 - Hardread Level"]
    #[inline]
    pub fn hrlev(&mut self) -> _HRLEVW {
        _HRLEVW { w: self }
    }
}
