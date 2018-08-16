#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHSCFG {
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
#[doc = "Possible values of the field `DIVS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVSR {
    #[doc = "fSH = fCONV / 1"]
    VALUE1,
    #[doc = "fSH = fCONV / 2"]
    VALUE2,
    #[doc = "fSH = fCONV / 16"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DIVSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVSR::VALUE1 => 0,
            DIVSR::VALUE2 => 1,
            DIVSR::VALUE3 => 15,
            DIVSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVSR {
        match value {
            0 => DIVSR::VALUE1,
            1 => DIVSR::VALUE2,
            15 => DIVSR::VALUE3,
            i => DIVSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIVSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIVSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DIVSR::VALUE3
    }
}
#[doc = "Possible values of the field `AREF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AREFR {
    #[doc = "External reference, upper supply range"]
    VALUE1,
    #[doc = "Internal reference, upper supply range"]
    VALUE3,
    #[doc = "Internal reference, lower supply range"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AREFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AREFR::VALUE1 => 0,
            AREFR::VALUE3 => 2,
            AREFR::VALUE4 => 3,
            AREFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AREFR {
        match value {
            0 => AREFR::VALUE1,
            2 => AREFR::VALUE3,
            3 => AREFR::VALUE4,
            i => AREFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AREFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == AREFR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == AREFR::VALUE4
    }
}
#[doc = "Possible values of the field `ANOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANOFFR {
    #[doc = "Converter controlled by bitfields ANONS (digital control block)"]
    VALUE1,
    #[doc = "Converter is permanently off"]
    VALUE2,
}
impl ANOFFR {
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
            ANOFFR::VALUE1 => false,
            ANOFFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ANOFFR {
        match value {
            false => ANOFFR::VALUE1,
            true => ANOFFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ANOFFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ANOFFR::VALUE2
    }
}
#[doc = "Possible values of the field `ANRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANRDYR {
    #[doc = "Converter is in power-down mode"]
    VALUE1,
    #[doc = "Converter is operable"]
    VALUE2,
}
impl ANRDYR {
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
            ANRDYR::VALUE1 => false,
            ANRDYR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ANRDYR {
        match value {
            false => ANRDYR::VALUE1,
            true => ANRDYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ANRDYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ANRDYR::VALUE2
    }
}
#[doc = "Possible values of the field `SP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP0R {
    #[doc = "No sample pending"]
    VALUE1,
    #[doc = "S&H unit x has finished the sample phase"]
    VALUE2,
}
impl SP0R {
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
            SP0R::VALUE1 => false,
            SP0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SP0R {
        match value {
            false => SP0R::VALUE1,
            true => SP0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SP0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SP0R::VALUE2
    }
}
#[doc = "Possible values of the field `SP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP1R {
    #[doc = "No sample pending"]
    VALUE1,
    #[doc = "S&H unit x has finished the sample phase"]
    VALUE2,
}
impl SP1R {
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
            SP1R::VALUE1 => false,
            SP1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SP1R {
        match value {
            false => SP1R::VALUE1,
            true => SP1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SP1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SP1R::VALUE2
    }
}
#[doc = "Possible values of the field `TC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCR {
    #[doc = "Internal test functions enabled"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TCR::VALUE1 => 11,
            TCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TCR {
        match value {
            11 => TCR::VALUE1,
            i => TCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TCR::VALUE1
    }
}
#[doc = "Possible values of the field `STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATER {
    #[doc = "Idle"]
    VALUE1,
    #[doc = "Offset calibration active"]
    VALUE2,
    #[doc = "Gain calibration active"]
    VALUE3,
    #[doc = "Startup calibration active"]
    VALUE4,
    #[doc = "Stepper process active for S&H unit 0"]
    VALUE5,
    #[doc = "Stepper process active for S&H unit 1"]
    VALUE6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATER::VALUE1 => 0,
            STATER::VALUE2 => 1,
            STATER::VALUE3 => 2,
            STATER::VALUE4 => 3,
            STATER::VALUE5 => 8,
            STATER::VALUE6 => 9,
            STATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATER {
        match value {
            0 => STATER::VALUE1,
            1 => STATER::VALUE2,
            2 => STATER::VALUE3,
            3 => STATER::VALUE4,
            8 => STATER::VALUE5,
            9 => STATER::VALUE6,
            i => STATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STATER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STATER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == STATER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == STATER::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == STATER::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == STATER::VALUE6
    }
}
#[doc = "Values that can be written to the field `DIVS`"]
pub enum DIVSW {
    #[doc = "fSH = fCONV / 1"]
    VALUE1,
    #[doc = "fSH = fCONV / 2"]
    VALUE2,
    #[doc = "fSH = fCONV / 16"]
    VALUE3,
}
impl DIVSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVSW::VALUE1 => 0,
            DIVSW::VALUE2 => 1,
            DIVSW::VALUE3 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVSW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "fSH = fCONV / 1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVSW::VALUE1)
    }
    #[doc = "fSH = fCONV / 2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVSW::VALUE2)
    }
    #[doc = "fSH = fCONV / 16"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DIVSW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AREF`"]
pub enum AREFW {
    #[doc = "External reference, upper supply range"]
    VALUE1,
    #[doc = "Internal reference, upper supply range"]
    VALUE3,
    #[doc = "Internal reference, lower supply range"]
    VALUE4,
}
impl AREFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AREFW::VALUE1 => 0,
            AREFW::VALUE3 => 2,
            AREFW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AREFW<'a> {
    w: &'a mut W,
}
impl<'a> _AREFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AREFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "External reference, upper supply range"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AREFW::VALUE1)
    }
    #[doc = "Internal reference, upper supply range"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(AREFW::VALUE3)
    }
    #[doc = "Internal reference, lower supply range"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(AREFW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ANOFF`"]
pub enum ANOFFW {
    #[doc = "Converter controlled by bitfields ANONS (digital control block)"]
    VALUE1,
    #[doc = "Converter is permanently off"]
    VALUE2,
}
impl ANOFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ANOFFW::VALUE1 => false,
            ANOFFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ANOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _ANOFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ANOFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Converter controlled by bitfields ANONS (digital control block)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ANOFFW::VALUE1)
    }
    #[doc = "Converter is permanently off"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ANOFFW::VALUE2)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCWC`"]
pub enum SCWCW {
    #[doc = "No write access to SHS configuration"]
    VALUE1,
    #[doc = "Bitfields ANOFF, AREF, DIVS can be written"]
    VALUE2,
}
impl SCWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCWCW::VALUE1 => false,
            SCWCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCWCW<'a> {
    w: &'a mut W,
}
impl<'a> _SCWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCWCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No write access to SHS configuration"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCWCW::VALUE1)
    }
    #[doc = "Bitfields ANOFF, AREF, DIVS can be written"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCWCW::VALUE2)
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
#[doc = "Values that can be written to the field `TC`"]
pub enum TCW {
    #[doc = "Internal test functions enabled"]
    VALUE1,
}
impl TCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TCW::VALUE1 => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCW<'a> {
    w: &'a mut W,
}
impl<'a> _TCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Internal test functions enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TCW::VALUE1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Divider Factor for the SHS Clock"]
    #[inline]
    pub fn divs(&self) -> DIVSR {
        DIVSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Analog Reference Voltage Selection"]
    #[inline]
    pub fn aref(&self) -> AREFR {
        AREFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Analog Converter Power Down Force"]
    #[inline]
    pub fn anoff(&self) -> ANOFFR {
        ANOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Analog Converter Ready"]
    #[inline]
    pub fn anrdy(&self) -> ANRDYR {
        ANRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Sample Pending on S&H Unit x"]
    #[inline]
    pub fn sp0(&self) -> SP0R {
        SP0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Sample Pending on S&H Unit x"]
    #[inline]
    pub fn sp1(&self) -> SP1R {
        SP1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:27 - Test Control"]
    #[inline]
    pub fn tc(&self) -> TCR {
        TCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Current State of Sequencer"]
    #[inline]
    pub fn state(&self) -> STATER {
        STATER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:3 - Divider Factor for the SHS Clock"]
    #[inline]
    pub fn divs(&mut self) -> _DIVSW {
        _DIVSW { w: self }
    }
    #[doc = "Bits 10:11 - Analog Reference Voltage Selection"]
    #[inline]
    pub fn aref(&mut self) -> _AREFW {
        _AREFW { w: self }
    }
    #[doc = "Bit 12 - Analog Converter Power Down Force"]
    #[inline]
    pub fn anoff(&mut self) -> _ANOFFW {
        _ANOFFW { w: self }
    }
    #[doc = "Bit 15 - Write Control for SHS Configuration"]
    #[inline]
    pub fn scwc(&mut self) -> _SCWCW {
        _SCWCW { w: self }
    }
    #[doc = "Bits 24:27 - Test Control"]
    #[inline]
    pub fn tc(&mut self) -> _TCW {
        _TCW { w: self }
    }
}
