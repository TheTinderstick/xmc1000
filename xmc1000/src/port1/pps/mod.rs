#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PPS {
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
#[doc = "Possible values of the field `PPS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS0R {
    #[doc = "Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl PPS0R {
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
            PPS0R::VALUE1 => false,
            PPS0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS0R {
        match value {
            false => PPS0R::VALUE1,
            true => PPS0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PPS0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PPS0R::VALUE2
    }
}
#[doc = "Possible values of the field `PPS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS1R {
    #[doc = "Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl PPS1R {
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
            PPS1R::VALUE1 => false,
            PPS1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS1R {
        match value {
            false => PPS1R::VALUE1,
            true => PPS1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PPS1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PPS1R::VALUE2
    }
}
#[doc = "Possible values of the field `PPS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS2R {
    #[doc = "Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl PPS2R {
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
            PPS2R::VALUE1 => false,
            PPS2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS2R {
        match value {
            false => PPS2R::VALUE1,
            true => PPS2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PPS2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PPS2R::VALUE2
    }
}
#[doc = "Possible values of the field `PPS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS3R {
    #[doc = "Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl PPS3R {
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
            PPS3R::VALUE1 => false,
            PPS3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS3R {
        match value {
            false => PPS3R::VALUE1,
            true => PPS3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PPS3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PPS3R::VALUE2
    }
}
#[doc = "Possible values of the field `PPS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS4R {
    #[doc = "Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl PPS4R {
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
            PPS4R::VALUE1 => false,
            PPS4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS4R {
        match value {
            false => PPS4R::VALUE1,
            true => PPS4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PPS4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PPS4R::VALUE2
    }
}
#[doc = "Possible values of the field `PPS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS5R {
    #[doc = "Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl PPS5R {
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
            PPS5R::VALUE1 => false,
            PPS5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS5R {
        match value {
            false => PPS5R::VALUE1,
            true => PPS5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PPS5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PPS5R::VALUE2
    }
}
#[doc = "Possible values of the field `PPS6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS6R {
    #[doc = "Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl PPS6R {
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
            PPS6R::VALUE1 => false,
            PPS6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPS6R {
        match value {
            false => PPS6R::VALUE1,
            true => PPS6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PPS6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PPS6R::VALUE2
    }
}
#[doc = "Values that can be written to the field `PPS0`"]
pub enum PPS0W {
    #[doc = "Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl PPS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS0W::VALUE1 => false,
            PPS0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS0W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of P1.x is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPS0W::VALUE1)
    }
    #[doc = "Pin Power Save of P1.x is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPS0W::VALUE2)
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
#[doc = "Values that can be written to the field `PPS1`"]
pub enum PPS1W {
    #[doc = "Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl PPS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS1W::VALUE1 => false,
            PPS1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of P1.x is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPS1W::VALUE1)
    }
    #[doc = "Pin Power Save of P1.x is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPS1W::VALUE2)
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
#[doc = "Values that can be written to the field `PPS2`"]
pub enum PPS2W {
    #[doc = "Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl PPS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS2W::VALUE1 => false,
            PPS2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS2W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of P1.x is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPS2W::VALUE1)
    }
    #[doc = "Pin Power Save of P1.x is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPS2W::VALUE2)
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
#[doc = "Values that can be written to the field `PPS3`"]
pub enum PPS3W {
    #[doc = "Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl PPS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS3W::VALUE1 => false,
            PPS3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS3W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of P1.x is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPS3W::VALUE1)
    }
    #[doc = "Pin Power Save of P1.x is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPS3W::VALUE2)
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
#[doc = "Values that can be written to the field `PPS4`"]
pub enum PPS4W {
    #[doc = "Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl PPS4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS4W::VALUE1 => false,
            PPS4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS4W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of P1.x is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPS4W::VALUE1)
    }
    #[doc = "Pin Power Save of P1.x is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPS4W::VALUE2)
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
#[doc = "Values that can be written to the field `PPS5`"]
pub enum PPS5W {
    #[doc = "Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl PPS5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS5W::VALUE1 => false,
            PPS5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS5W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of P1.x is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPS5W::VALUE1)
    }
    #[doc = "Pin Power Save of P1.x is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPS5W::VALUE2)
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
#[doc = "Values that can be written to the field `PPS6`"]
pub enum PPS6W {
    #[doc = "Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl PPS6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPS6W::VALUE1 => false,
            PPS6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPS6W<'a> {
    w: &'a mut W,
}
impl<'a> _PPS6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPS6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Power Save of P1.x is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPS6W::VALUE1)
    }
    #[doc = "Pin Power Save of P1.x is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPS6W::VALUE2)
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
    #[doc = "Bit 0 - Port 1 Pin Power Save Bit 0"]
    #[inline]
    pub fn pps0(&self) -> PPS0R {
        PPS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Port 1 Pin Power Save Bit 1"]
    #[inline]
    pub fn pps1(&self) -> PPS1R {
        PPS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port 1 Pin Power Save Bit 2"]
    #[inline]
    pub fn pps2(&self) -> PPS2R {
        PPS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Port 1 Pin Power Save Bit 3"]
    #[inline]
    pub fn pps3(&self) -> PPS3R {
        PPS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Port 1 Pin Power Save Bit 4"]
    #[inline]
    pub fn pps4(&self) -> PPS4R {
        PPS4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Port 1 Pin Power Save Bit 5"]
    #[inline]
    pub fn pps5(&self) -> PPS5R {
        PPS5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Port 1 Pin Power Save Bit 6"]
    #[inline]
    pub fn pps6(&self) -> PPS6R {
        PPS6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - Port 1 Pin Power Save Bit 0"]
    #[inline]
    pub fn pps0(&mut self) -> _PPS0W {
        _PPS0W { w: self }
    }
    #[doc = "Bit 1 - Port 1 Pin Power Save Bit 1"]
    #[inline]
    pub fn pps1(&mut self) -> _PPS1W {
        _PPS1W { w: self }
    }
    #[doc = "Bit 2 - Port 1 Pin Power Save Bit 2"]
    #[inline]
    pub fn pps2(&mut self) -> _PPS2W {
        _PPS2W { w: self }
    }
    #[doc = "Bit 3 - Port 1 Pin Power Save Bit 3"]
    #[inline]
    pub fn pps3(&mut self) -> _PPS3W {
        _PPS3W { w: self }
    }
    #[doc = "Bit 4 - Port 1 Pin Power Save Bit 4"]
    #[inline]
    pub fn pps4(&mut self) -> _PPS4W {
        _PPS4W { w: self }
    }
    #[doc = "Bit 5 - Port 1 Pin Power Save Bit 5"]
    #[inline]
    pub fn pps5(&mut self) -> _PPS5W {
        _PPS5W { w: self }
    }
    #[doc = "Bit 6 - Port 1 Pin Power Save Bit 6"]
    #[inline]
    pub fn pps6(&mut self) -> _PPS6W {
        _PPS6W { w: self }
    }
}
