#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACCPROT0 {
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
#[doc = "Possible values of the field `APC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APC0R {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to channel control registers is blocked"]
    VALUE2,
}
impl APC0R {
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
            APC0R::VALUE1 => false,
            APC0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> APC0R {
        match value {
            false => APC0R::VALUE1,
            true => APC0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == APC0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == APC0R::VALUE2
    }
}
#[doc = "Possible values of the field `APC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APC1R {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to channel control registers is blocked"]
    VALUE2,
}
impl APC1R {
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
            APC1R::VALUE1 => false,
            APC1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> APC1R {
        match value {
            false => APC1R::VALUE1,
            true => APC1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == APC1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == APC1R::VALUE2
    }
}
#[doc = "Possible values of the field `APEM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APEMR {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to external multiplexer registers is blocked"]
    VALUE2,
}
impl APEMR {
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
            APEMR::VALUE1 => false,
            APEMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> APEMR {
        match value {
            false => APEMR::VALUE1,
            true => APEMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == APEMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == APEMR::VALUE2
    }
}
#[doc = "Possible values of the field `API0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum API0R {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to initialization registers is blocked"]
    VALUE2,
}
impl API0R {
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
            API0R::VALUE1 => false,
            API0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> API0R {
        match value {
            false => API0R::VALUE1,
            true => API0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == API0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == API0R::VALUE2
    }
}
#[doc = "Possible values of the field `API1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum API1R {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to initialization registers is blocked"]
    VALUE2,
}
impl API1R {
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
            API1R::VALUE1 => false,
            API1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> API1R {
        match value {
            false => API1R::VALUE1,
            true => API1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == API1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == API1R::VALUE2
    }
}
#[doc = "Possible values of the field `APGC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APGCR {
    #[doc = "Full access to register"]
    VALUE1,
    #[doc = "Write access to global configuration register is blocked"]
    VALUE2,
}
impl APGCR {
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
            APGCR::VALUE1 => false,
            APGCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> APGCR {
        match value {
            false => APGCR::VALUE1,
            true => APGCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == APGCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == APGCR::VALUE2
    }
}
#[doc = "Values that can be written to the field `APC0`"]
pub enum APC0W {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to channel control registers is blocked"]
    VALUE2,
}
impl APC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            APC0W::VALUE1 => false,
            APC0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APC0W<'a> {
    w: &'a mut W,
}
impl<'a> _APC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APC0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Full access to registers"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(APC0W::VALUE1)
    }
    #[doc = "Write access to channel control registers is blocked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(APC0W::VALUE2)
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
#[doc = "Values that can be written to the field `APC1`"]
pub enum APC1W {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to channel control registers is blocked"]
    VALUE2,
}
impl APC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            APC1W::VALUE1 => false,
            APC1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APC1W<'a> {
    w: &'a mut W,
}
impl<'a> _APC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APC1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Full access to registers"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(APC1W::VALUE1)
    }
    #[doc = "Write access to channel control registers is blocked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(APC1W::VALUE2)
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
#[doc = "Values that can be written to the field `APEM`"]
pub enum APEMW {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to external multiplexer registers is blocked"]
    VALUE2,
}
impl APEMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            APEMW::VALUE1 => false,
            APEMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APEMW<'a> {
    w: &'a mut W,
}
impl<'a> _APEMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APEMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Full access to registers"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(APEMW::VALUE1)
    }
    #[doc = "Write access to external multiplexer registers is blocked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(APEMW::VALUE2)
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
#[doc = "Values that can be written to the field `API0`"]
pub enum API0W {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to initialization registers is blocked"]
    VALUE2,
}
impl API0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            API0W::VALUE1 => false,
            API0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _API0W<'a> {
    w: &'a mut W,
}
impl<'a> _API0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: API0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Full access to registers"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(API0W::VALUE1)
    }
    #[doc = "Write access to initialization registers is blocked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(API0W::VALUE2)
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
#[doc = "Values that can be written to the field `API1`"]
pub enum API1W {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to initialization registers is blocked"]
    VALUE2,
}
impl API1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            API1W::VALUE1 => false,
            API1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _API1W<'a> {
    w: &'a mut W,
}
impl<'a> _API1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: API1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Full access to registers"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(API1W::VALUE1)
    }
    #[doc = "Write access to initialization registers is blocked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(API1W::VALUE2)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `APGC`"]
pub enum APGCW {
    #[doc = "Full access to register"]
    VALUE1,
    #[doc = "Write access to global configuration register is blocked"]
    VALUE2,
}
impl APGCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            APGCW::VALUE1 => false,
            APGCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APGCW<'a> {
    w: &'a mut W,
}
impl<'a> _APGCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APGCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Full access to register"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(APGCW::VALUE1)
    }
    #[doc = "Write access to global configuration register is blocked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(APGCW::VALUE2)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Access Protection Channel Control, Group 0 - 1"]
    #[inline]
    pub fn apc0(&self) -> APC0R {
        APC0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Access Protection Channel Control, Group 0 - 1"]
    #[inline]
    pub fn apc1(&self) -> APC1R {
        APC1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Access Protection External Multiplexer"]
    #[inline]
    pub fn apem(&self) -> APEMR {
        APEMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Access Protection Initialization, Group 0 - 1"]
    #[inline]
    pub fn api0(&self) -> API0R {
        API0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Access Protection Initialization, Group 0 - 1"]
    #[inline]
    pub fn api1(&self) -> API1R {
        API1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Access Protection Global Configuration"]
    #[inline]
    pub fn apgc(&self) -> APGCR {
        APGCR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Access Protection Channel Control, Group 0 - 1"]
    #[inline]
    pub fn apc0(&mut self) -> _APC0W {
        _APC0W { w: self }
    }
    #[doc = "Bit 1 - Access Protection Channel Control, Group 0 - 1"]
    #[inline]
    pub fn apc1(&mut self) -> _APC1W {
        _APC1W { w: self }
    }
    #[doc = "Bit 15 - Access Protection External Multiplexer"]
    #[inline]
    pub fn apem(&mut self) -> _APEMW {
        _APEMW { w: self }
    }
    #[doc = "Bit 16 - Access Protection Initialization, Group 0 - 1"]
    #[inline]
    pub fn api0(&mut self) -> _API0W {
        _API0W { w: self }
    }
    #[doc = "Bit 17 - Access Protection Initialization, Group 0 - 1"]
    #[inline]
    pub fn api1(&mut self) -> _API1W {
        _API1W { w: self }
    }
    #[doc = "Bit 31 - Access Protection Global Configuration"]
    #[inline]
    pub fn apgc(&mut self) -> _APGCW {
        _APGCW { w: self }
    }
}
