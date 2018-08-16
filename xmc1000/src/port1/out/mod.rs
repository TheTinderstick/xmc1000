#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUT {
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
#[doc = "Possible values of the field `P0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0R {
    #[doc = "The output level of P1.x is 0."]
    VALUE1,
    #[doc = "The output level of P1.x is 1."]
    VALUE2,
}
impl P0R {
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
            P0R::VALUE1 => false,
            P0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P0R {
        match value {
            false => P0R::VALUE1,
            true => P0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == P0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == P0R::VALUE2
    }
}
#[doc = "Possible values of the field `P1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1R {
    #[doc = "The output level of P1.x is 0."]
    VALUE1,
    #[doc = "The output level of P1.x is 1."]
    VALUE2,
}
impl P1R {
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
            P1R::VALUE1 => false,
            P1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1R {
        match value {
            false => P1R::VALUE1,
            true => P1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == P1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == P1R::VALUE2
    }
}
#[doc = "Possible values of the field `P2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2R {
    #[doc = "The output level of P1.x is 0."]
    VALUE1,
    #[doc = "The output level of P1.x is 1."]
    VALUE2,
}
impl P2R {
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
            P2R::VALUE1 => false,
            P2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2R {
        match value {
            false => P2R::VALUE1,
            true => P2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == P2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == P2R::VALUE2
    }
}
#[doc = "Possible values of the field `P3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3R {
    #[doc = "The output level of P1.x is 0."]
    VALUE1,
    #[doc = "The output level of P1.x is 1."]
    VALUE2,
}
impl P3R {
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
            P3R::VALUE1 => false,
            P3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P3R {
        match value {
            false => P3R::VALUE1,
            true => P3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == P3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == P3R::VALUE2
    }
}
#[doc = "Possible values of the field `P4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4R {
    #[doc = "The output level of P1.x is 0."]
    VALUE1,
    #[doc = "The output level of P1.x is 1."]
    VALUE2,
}
impl P4R {
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
            P4R::VALUE1 => false,
            P4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P4R {
        match value {
            false => P4R::VALUE1,
            true => P4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == P4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == P4R::VALUE2
    }
}
#[doc = "Possible values of the field `P5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P5R {
    #[doc = "The output level of P1.x is 0."]
    VALUE1,
    #[doc = "The output level of P1.x is 1."]
    VALUE2,
}
impl P5R {
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
            P5R::VALUE1 => false,
            P5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P5R {
        match value {
            false => P5R::VALUE1,
            true => P5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == P5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == P5R::VALUE2
    }
}
#[doc = "Possible values of the field `P6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P6R {
    #[doc = "The output level of P1.x is 0."]
    VALUE1,
    #[doc = "The output level of P1.x is 1."]
    VALUE2,
}
impl P6R {
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
            P6R::VALUE1 => false,
            P6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P6R {
        match value {
            false => P6R::VALUE1,
            true => P6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == P6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == P6R::VALUE2
    }
}
#[doc = "Values that can be written to the field `P0`"]
pub enum P0W {
    #[doc = "The output level of P1.x is 0."]
    VALUE1,
    #[doc = "The output level of P1.x is 1."]
    VALUE2,
}
impl P0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P0W::VALUE1 => false,
            P0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0W<'a> {
    w: &'a mut W,
}
impl<'a> _P0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of P1.x is 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(P0W::VALUE1)
    }
    #[doc = "The output level of P1.x is 1."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(P0W::VALUE2)
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
#[doc = "Values that can be written to the field `P1`"]
pub enum P1W {
    #[doc = "The output level of P1.x is 0."]
    VALUE1,
    #[doc = "The output level of P1.x is 1."]
    VALUE2,
}
impl P1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1W::VALUE1 => false,
            P1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1W<'a> {
    w: &'a mut W,
}
impl<'a> _P1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of P1.x is 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(P1W::VALUE1)
    }
    #[doc = "The output level of P1.x is 1."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(P1W::VALUE2)
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
#[doc = "Values that can be written to the field `P2`"]
pub enum P2W {
    #[doc = "The output level of P1.x is 0."]
    VALUE1,
    #[doc = "The output level of P1.x is 1."]
    VALUE2,
}
impl P2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P2W::VALUE1 => false,
            P2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2W<'a> {
    w: &'a mut W,
}
impl<'a> _P2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of P1.x is 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(P2W::VALUE1)
    }
    #[doc = "The output level of P1.x is 1."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(P2W::VALUE2)
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
#[doc = "Values that can be written to the field `P3`"]
pub enum P3W {
    #[doc = "The output level of P1.x is 0."]
    VALUE1,
    #[doc = "The output level of P1.x is 1."]
    VALUE2,
}
impl P3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P3W::VALUE1 => false,
            P3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P3W<'a> {
    w: &'a mut W,
}
impl<'a> _P3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of P1.x is 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(P3W::VALUE1)
    }
    #[doc = "The output level of P1.x is 1."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(P3W::VALUE2)
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
#[doc = "Values that can be written to the field `P4`"]
pub enum P4W {
    #[doc = "The output level of P1.x is 0."]
    VALUE1,
    #[doc = "The output level of P1.x is 1."]
    VALUE2,
}
impl P4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P4W::VALUE1 => false,
            P4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P4W<'a> {
    w: &'a mut W,
}
impl<'a> _P4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of P1.x is 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(P4W::VALUE1)
    }
    #[doc = "The output level of P1.x is 1."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(P4W::VALUE2)
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
#[doc = "Values that can be written to the field `P5`"]
pub enum P5W {
    #[doc = "The output level of P1.x is 0."]
    VALUE1,
    #[doc = "The output level of P1.x is 1."]
    VALUE2,
}
impl P5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P5W::VALUE1 => false,
            P5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P5W<'a> {
    w: &'a mut W,
}
impl<'a> _P5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of P1.x is 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(P5W::VALUE1)
    }
    #[doc = "The output level of P1.x is 1."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(P5W::VALUE2)
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
#[doc = "Values that can be written to the field `P6`"]
pub enum P6W {
    #[doc = "The output level of P1.x is 0."]
    VALUE1,
    #[doc = "The output level of P1.x is 1."]
    VALUE2,
}
impl P6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P6W::VALUE1 => false,
            P6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P6W<'a> {
    w: &'a mut W,
}
impl<'a> _P6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The output level of P1.x is 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(P6W::VALUE1)
    }
    #[doc = "The output level of P1.x is 1."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(P6W::VALUE2)
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
    #[doc = "Bit 0 - Port 1 Output Bit 0"]
    #[inline]
    pub fn p0(&self) -> P0R {
        P0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Port 1 Output Bit 1"]
    #[inline]
    pub fn p1(&self) -> P1R {
        P1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port 1 Output Bit 2"]
    #[inline]
    pub fn p2(&self) -> P2R {
        P2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Port 1 Output Bit 3"]
    #[inline]
    pub fn p3(&self) -> P3R {
        P3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Port 1 Output Bit 4"]
    #[inline]
    pub fn p4(&self) -> P4R {
        P4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Port 1 Output Bit 5"]
    #[inline]
    pub fn p5(&self) -> P5R {
        P5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Port 1 Output Bit 6"]
    #[inline]
    pub fn p6(&self) -> P6R {
        P6R::_from({
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
    #[doc = "Bit 0 - Port 1 Output Bit 0"]
    #[inline]
    pub fn p0(&mut self) -> _P0W {
        _P0W { w: self }
    }
    #[doc = "Bit 1 - Port 1 Output Bit 1"]
    #[inline]
    pub fn p1(&mut self) -> _P1W {
        _P1W { w: self }
    }
    #[doc = "Bit 2 - Port 1 Output Bit 2"]
    #[inline]
    pub fn p2(&mut self) -> _P2W {
        _P2W { w: self }
    }
    #[doc = "Bit 3 - Port 1 Output Bit 3"]
    #[inline]
    pub fn p3(&mut self) -> _P3W {
        _P3W { w: self }
    }
    #[doc = "Bit 4 - Port 1 Output Bit 4"]
    #[inline]
    pub fn p4(&mut self) -> _P4W {
        _P4W { w: self }
    }
    #[doc = "Bit 5 - Port 1 Output Bit 5"]
    #[inline]
    pub fn p5(&mut self) -> _P5W {
        _P5W { w: self }
    }
    #[doc = "Bit 6 - Port 1 Output Bit 6"]
    #[inline]
    pub fn p6(&mut self) -> _P6W {
        _P6W { w: self }
    }
}
