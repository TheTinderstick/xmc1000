#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACCPROT1 {
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
#[doc = "Possible values of the field `APS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APS0R {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to service request registers is blocked"]
    VALUE2,
}
impl APS0R {
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
            APS0R::VALUE1 => false,
            APS0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> APS0R {
        match value {
            false => APS0R::VALUE1,
            true => APS0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == APS0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == APS0R::VALUE2
    }
}
#[doc = "Possible values of the field `APS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APS1R {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to service request registers is blocked"]
    VALUE2,
}
impl APS1R {
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
            APS1R::VALUE1 => false,
            APS1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> APS1R {
        match value {
            false => APS1R::VALUE1,
            true => APS1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == APS1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == APS1R::VALUE2
    }
}
#[doc = "Possible values of the field `APTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APTFR {
    #[doc = "Full access to register"]
    VALUE1,
    #[doc = "Write access to test function register is blocked"]
    VALUE2,
}
impl APTFR {
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
            APTFR::VALUE1 => false,
            APTFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> APTFR {
        match value {
            false => APTFR::VALUE1,
            true => APTFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == APTFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == APTFR::VALUE2
    }
}
#[doc = "Possible values of the field `APR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APR0R {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to result registers is blocked"]
    VALUE2,
}
impl APR0R {
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
            APR0R::VALUE1 => false,
            APR0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> APR0R {
        match value {
            false => APR0R::VALUE1,
            true => APR0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == APR0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == APR0R::VALUE2
    }
}
#[doc = "Possible values of the field `APR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APR1R {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to result registers is blocked"]
    VALUE2,
}
impl APR1R {
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
            APR1R::VALUE1 => false,
            APR1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> APR1R {
        match value {
            false => APR1R::VALUE1,
            true => APR1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == APR1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == APR1R::VALUE2
    }
}
#[doc = "Values that can be written to the field `APS0`"]
pub enum APS0W {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to service request registers is blocked"]
    VALUE2,
}
impl APS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            APS0W::VALUE1 => false,
            APS0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APS0W<'a> {
    w: &'a mut W,
}
impl<'a> _APS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Full access to registers"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(APS0W::VALUE1)
    }
    #[doc = "Write access to service request registers is blocked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(APS0W::VALUE2)
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
#[doc = "Values that can be written to the field `APS1`"]
pub enum APS1W {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to service request registers is blocked"]
    VALUE2,
}
impl APS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            APS1W::VALUE1 => false,
            APS1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APS1W<'a> {
    w: &'a mut W,
}
impl<'a> _APS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Full access to registers"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(APS1W::VALUE1)
    }
    #[doc = "Write access to service request registers is blocked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(APS1W::VALUE2)
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
#[doc = "Values that can be written to the field `APTF`"]
pub enum APTFW {
    #[doc = "Full access to register"]
    VALUE1,
    #[doc = "Write access to test function register is blocked"]
    VALUE2,
}
impl APTFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            APTFW::VALUE1 => false,
            APTFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APTFW<'a> {
    w: &'a mut W,
}
impl<'a> _APTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Full access to register"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(APTFW::VALUE1)
    }
    #[doc = "Write access to test function register is blocked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(APTFW::VALUE2)
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
#[doc = "Values that can be written to the field `APR0`"]
pub enum APR0W {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to result registers is blocked"]
    VALUE2,
}
impl APR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            APR0W::VALUE1 => false,
            APR0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APR0W<'a> {
    w: &'a mut W,
}
impl<'a> _APR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Full access to registers"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(APR0W::VALUE1)
    }
    #[doc = "Write access to result registers is blocked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(APR0W::VALUE2)
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
#[doc = "Values that can be written to the field `APR1`"]
pub enum APR1W {
    #[doc = "Full access to registers"]
    VALUE1,
    #[doc = "Write access to result registers is blocked"]
    VALUE2,
}
impl APR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            APR1W::VALUE1 => false,
            APR1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APR1W<'a> {
    w: &'a mut W,
}
impl<'a> _APR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Full access to registers"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(APR1W::VALUE1)
    }
    #[doc = "Write access to result registers is blocked"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(APR1W::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Access Protection Service Request, Group 0 - 1"]
    #[inline]
    pub fn aps0(&self) -> APS0R {
        APS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Access Protection Service Request, Group 0 - 1"]
    #[inline]
    pub fn aps1(&self) -> APS1R {
        APS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Access Protection Test Function"]
    #[inline]
    pub fn aptf(&self) -> APTFR {
        APTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Access Protection Result Registers, Group 0 - 1"]
    #[inline]
    pub fn apr0(&self) -> APR0R {
        APR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Access Protection Result Registers, Group 0 - 1"]
    #[inline]
    pub fn apr1(&self) -> APR1R {
        APR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
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
    #[doc = "Bit 0 - Access Protection Service Request, Group 0 - 1"]
    #[inline]
    pub fn aps0(&mut self) -> _APS0W {
        _APS0W { w: self }
    }
    #[doc = "Bit 1 - Access Protection Service Request, Group 0 - 1"]
    #[inline]
    pub fn aps1(&mut self) -> _APS1W {
        _APS1W { w: self }
    }
    #[doc = "Bit 15 - Access Protection Test Function"]
    #[inline]
    pub fn aptf(&mut self) -> _APTFW {
        _APTFW { w: self }
    }
    #[doc = "Bit 16 - Access Protection Result Registers, Group 0 - 1"]
    #[inline]
    pub fn apr0(&mut self) -> _APR0W {
        _APR0W { w: self }
    }
    #[doc = "Bit 17 - Access Protection Result Registers, Group 0 - 1"]
    #[inline]
    pub fn apr1(&mut self) -> _APR1W {
        _APR1W { w: self }
    }
}
