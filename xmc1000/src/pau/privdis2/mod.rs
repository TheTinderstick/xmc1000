#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRIVDIS2 {
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
#[doc = "Possible values of the field `PDIS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS0R {
    #[doc = "CC80 and CCU80 Kernel SFRs are accessible."]
    VALUE1,
    #[doc = "CC80 and CCU80 Kernel SFRs are not accessible."]
    VALUE2,
}
impl PDIS0R {
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
            PDIS0R::VALUE1 => false,
            PDIS0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS0R {
        match value {
            false => PDIS0R::VALUE1,
            true => PDIS0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS0R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS1R {
    #[doc = "CC81 is accessible."]
    VALUE1,
    #[doc = "CC81 is not accessible."]
    VALUE2,
}
impl PDIS1R {
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
            PDIS1R::VALUE1 => false,
            PDIS1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS1R {
        match value {
            false => PDIS1R::VALUE1,
            true => PDIS1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS1R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS2R {
    #[doc = "CC82 is accessible."]
    VALUE1,
    #[doc = "CC82 is not accessible."]
    VALUE2,
}
impl PDIS2R {
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
            PDIS2R::VALUE1 => false,
            PDIS2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS2R {
        match value {
            false => PDIS2R::VALUE1,
            true => PDIS2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS2R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS3R {
    #[doc = "CC83 is accessible."]
    VALUE1,
    #[doc = "CC83 is not accessible."]
    VALUE2,
}
impl PDIS3R {
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
            PDIS3R::VALUE1 => false,
            PDIS3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS3R {
        match value {
            false => PDIS3R::VALUE1,
            true => PDIS3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS3R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS12R {
    #[doc = "POSIF0 is accessible."]
    VALUE1,
    #[doc = "POSIF0 is not accessible."]
    VALUE2,
}
impl PDIS12R {
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
            PDIS12R::VALUE1 => false,
            PDIS12R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS12R {
        match value {
            false => PDIS12R::VALUE1,
            true => PDIS12R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS12R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS12R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS15R {
    #[doc = "BCCU0 is accessible."]
    VALUE1,
    #[doc = "BCCU0 is not accessible."]
    VALUE2,
}
impl PDIS15R {
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
            PDIS15R::VALUE1 => false,
            PDIS15R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS15R {
        match value {
            false => PDIS15R::VALUE1,
            true => PDIS15R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS15R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS15R::VALUE2
    }
}
#[doc = "Values that can be written to the field `PDIS0`"]
pub enum PDIS0W {
    #[doc = "CC80 and CCU80 Kernel SFRs are accessible."]
    VALUE1,
    #[doc = "CC80 and CCU80 Kernel SFRs are not accessible."]
    VALUE2,
}
impl PDIS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS0W::VALUE1 => false,
            PDIS0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS0W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CC80 and CCU80 Kernel SFRs are accessible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS0W::VALUE1)
    }
    #[doc = "CC80 and CCU80 Kernel SFRs are not accessible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS0W::VALUE2)
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
#[doc = "Values that can be written to the field `PDIS1`"]
pub enum PDIS1W {
    #[doc = "CC81 is accessible."]
    VALUE1,
    #[doc = "CC81 is not accessible."]
    VALUE2,
}
impl PDIS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS1W::VALUE1 => false,
            PDIS1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CC81 is accessible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS1W::VALUE1)
    }
    #[doc = "CC81 is not accessible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS1W::VALUE2)
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
#[doc = "Values that can be written to the field `PDIS2`"]
pub enum PDIS2W {
    #[doc = "CC82 is accessible."]
    VALUE1,
    #[doc = "CC82 is not accessible."]
    VALUE2,
}
impl PDIS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS2W::VALUE1 => false,
            PDIS2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS2W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CC82 is accessible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS2W::VALUE1)
    }
    #[doc = "CC82 is not accessible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS2W::VALUE2)
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
#[doc = "Values that can be written to the field `PDIS3`"]
pub enum PDIS3W {
    #[doc = "CC83 is accessible."]
    VALUE1,
    #[doc = "CC83 is not accessible."]
    VALUE2,
}
impl PDIS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS3W::VALUE1 => false,
            PDIS3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS3W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CC83 is accessible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS3W::VALUE1)
    }
    #[doc = "CC83 is not accessible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS3W::VALUE2)
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
#[doc = "Values that can be written to the field `PDIS12`"]
pub enum PDIS12W {
    #[doc = "POSIF0 is accessible."]
    VALUE1,
    #[doc = "POSIF0 is not accessible."]
    VALUE2,
}
impl PDIS12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS12W::VALUE1 => false,
            PDIS12W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS12W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "POSIF0 is accessible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS12W::VALUE1)
    }
    #[doc = "POSIF0 is not accessible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS12W::VALUE2)
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
#[doc = "Values that can be written to the field `PDIS15`"]
pub enum PDIS15W {
    #[doc = "BCCU0 is accessible."]
    VALUE1,
    #[doc = "BCCU0 is not accessible."]
    VALUE2,
}
impl PDIS15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS15W::VALUE1 => false,
            PDIS15W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS15W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BCCU0 is accessible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS15W::VALUE1)
    }
    #[doc = "BCCU0 is not accessible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS15W::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - CC80 and CCU80 Kernel SFRs Privilege Disable Flag"]
    #[inline]
    pub fn pdis0(&self) -> PDIS0R {
        PDIS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - CC81 Privilege Disable Flag"]
    #[inline]
    pub fn pdis1(&self) -> PDIS1R {
        PDIS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - CC82 Privilege Disable Flag"]
    #[inline]
    pub fn pdis2(&self) -> PDIS2R {
        PDIS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - CC83 Privilege Disable Flag"]
    #[inline]
    pub fn pdis3(&self) -> PDIS3R {
        PDIS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - POSIF0 Privilege Disable Flag"]
    #[inline]
    pub fn pdis12(&self) -> PDIS12R {
        PDIS12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - BCCU0 Privilege Disable Flag"]
    #[inline]
    pub fn pdis15(&self) -> PDIS15R {
        PDIS15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - CC80 and CCU80 Kernel SFRs Privilege Disable Flag"]
    #[inline]
    pub fn pdis0(&mut self) -> _PDIS0W {
        _PDIS0W { w: self }
    }
    #[doc = "Bit 1 - CC81 Privilege Disable Flag"]
    #[inline]
    pub fn pdis1(&mut self) -> _PDIS1W {
        _PDIS1W { w: self }
    }
    #[doc = "Bit 2 - CC82 Privilege Disable Flag"]
    #[inline]
    pub fn pdis2(&mut self) -> _PDIS2W {
        _PDIS2W { w: self }
    }
    #[doc = "Bit 3 - CC83 Privilege Disable Flag"]
    #[inline]
    pub fn pdis3(&mut self) -> _PDIS3W {
        _PDIS3W { w: self }
    }
    #[doc = "Bit 12 - POSIF0 Privilege Disable Flag"]
    #[inline]
    pub fn pdis12(&mut self) -> _PDIS12W {
        _PDIS12W { w: self }
    }
    #[doc = "Bit 15 - BCCU0 Privilege Disable Flag"]
    #[inline]
    pub fn pdis15(&mut self) -> _PDIS15W {
        _PDIS15W { w: self }
    }
}
