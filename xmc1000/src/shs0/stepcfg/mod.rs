#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STEPCFG {
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
pub struct KSEL0R {
    bits: u8,
}
impl KSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct KSEL1R {
    bits: u8,
}
impl KSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct KSEL2R {
    bits: u8,
}
impl KSEL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct KSEL3R {
    bits: u8,
}
impl KSEL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct KSEL4R {
    bits: u8,
}
impl KSEL4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct KSEL5R {
    bits: u8,
}
impl KSEL5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct KSEL6R {
    bits: u8,
}
impl KSEL6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct KSEL7R {
    bits: u8,
}
impl KSEL7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN0R {
    #[doc = "Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "Active: This step is executed during the sequence"]
    VALUE2,
}
impl SEN0R {
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
            SEN0R::VALUE1 => false,
            SEN0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEN0R {
        match value {
            false => SEN0R::VALUE1,
            true => SEN0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SEN0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SEN0R::VALUE2
    }
}
#[doc = "Possible values of the field `SEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN1R {
    #[doc = "Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "Active: This step is executed during the sequence"]
    VALUE2,
}
impl SEN1R {
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
            SEN1R::VALUE1 => false,
            SEN1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEN1R {
        match value {
            false => SEN1R::VALUE1,
            true => SEN1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SEN1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SEN1R::VALUE2
    }
}
#[doc = "Possible values of the field `SEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN2R {
    #[doc = "Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "Active: This step is executed during the sequence"]
    VALUE2,
}
impl SEN2R {
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
            SEN2R::VALUE1 => false,
            SEN2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEN2R {
        match value {
            false => SEN2R::VALUE1,
            true => SEN2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SEN2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SEN2R::VALUE2
    }
}
#[doc = "Possible values of the field `SEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN3R {
    #[doc = "Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "Active: This step is executed during the sequence"]
    VALUE2,
}
impl SEN3R {
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
            SEN3R::VALUE1 => false,
            SEN3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEN3R {
        match value {
            false => SEN3R::VALUE1,
            true => SEN3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SEN3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SEN3R::VALUE2
    }
}
#[doc = "Possible values of the field `SEN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN4R {
    #[doc = "Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "Active: This step is executed during the sequence"]
    VALUE2,
}
impl SEN4R {
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
            SEN4R::VALUE1 => false,
            SEN4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEN4R {
        match value {
            false => SEN4R::VALUE1,
            true => SEN4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SEN4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SEN4R::VALUE2
    }
}
#[doc = "Possible values of the field `SEN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN5R {
    #[doc = "Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "Active: This step is executed during the sequence"]
    VALUE2,
}
impl SEN5R {
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
            SEN5R::VALUE1 => false,
            SEN5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEN5R {
        match value {
            false => SEN5R::VALUE1,
            true => SEN5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SEN5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SEN5R::VALUE2
    }
}
#[doc = "Possible values of the field `SEN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN6R {
    #[doc = "Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "Active: This step is executed during the sequence"]
    VALUE2,
}
impl SEN6R {
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
            SEN6R::VALUE1 => false,
            SEN6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEN6R {
        match value {
            false => SEN6R::VALUE1,
            true => SEN6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SEN6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SEN6R::VALUE2
    }
}
#[doc = "Possible values of the field `SEN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN7R {
    #[doc = "Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "Active: This step is executed during the sequence"]
    VALUE2,
}
impl SEN7R {
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
            SEN7R::VALUE1 => false,
            SEN7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEN7R {
        match value {
            false => SEN7R::VALUE1,
            true => SEN7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SEN7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SEN7R::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _KSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _KSEL0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _KSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _KSEL1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _KSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _KSEL2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _KSEL3W<'a> {
    w: &'a mut W,
}
impl<'a> _KSEL3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _KSEL4W<'a> {
    w: &'a mut W,
}
impl<'a> _KSEL4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _KSEL5W<'a> {
    w: &'a mut W,
}
impl<'a> _KSEL5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _KSEL6W<'a> {
    w: &'a mut W,
}
impl<'a> _KSEL6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _KSEL7W<'a> {
    w: &'a mut W,
}
impl<'a> _KSEL7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SEN0`"]
pub enum SEN0W {
    #[doc = "Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "Active: This step is executed during the sequence"]
    VALUE2,
}
impl SEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEN0W::VALUE1 => false,
            SEN0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _SEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off: This step is not part of the stepper sequence"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEN0W::VALUE1)
    }
    #[doc = "Active: This step is executed during the sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEN0W::VALUE2)
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
#[doc = "Values that can be written to the field `SEN1`"]
pub enum SEN1W {
    #[doc = "Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "Active: This step is executed during the sequence"]
    VALUE2,
}
impl SEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEN1W::VALUE1 => false,
            SEN1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _SEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off: This step is not part of the stepper sequence"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEN1W::VALUE1)
    }
    #[doc = "Active: This step is executed during the sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEN1W::VALUE2)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SEN2`"]
pub enum SEN2W {
    #[doc = "Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "Active: This step is executed during the sequence"]
    VALUE2,
}
impl SEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEN2W::VALUE1 => false,
            SEN2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _SEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off: This step is not part of the stepper sequence"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEN2W::VALUE1)
    }
    #[doc = "Active: This step is executed during the sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEN2W::VALUE2)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SEN3`"]
pub enum SEN3W {
    #[doc = "Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "Active: This step is executed during the sequence"]
    VALUE2,
}
impl SEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEN3W::VALUE1 => false,
            SEN3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _SEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off: This step is not part of the stepper sequence"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEN3W::VALUE1)
    }
    #[doc = "Active: This step is executed during the sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEN3W::VALUE2)
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
#[doc = "Values that can be written to the field `SEN4`"]
pub enum SEN4W {
    #[doc = "Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "Active: This step is executed during the sequence"]
    VALUE2,
}
impl SEN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEN4W::VALUE1 => false,
            SEN4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _SEN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off: This step is not part of the stepper sequence"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEN4W::VALUE1)
    }
    #[doc = "Active: This step is executed during the sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEN4W::VALUE2)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SEN5`"]
pub enum SEN5W {
    #[doc = "Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "Active: This step is executed during the sequence"]
    VALUE2,
}
impl SEN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEN5W::VALUE1 => false,
            SEN5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _SEN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off: This step is not part of the stepper sequence"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEN5W::VALUE1)
    }
    #[doc = "Active: This step is executed during the sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEN5W::VALUE2)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SEN6`"]
pub enum SEN6W {
    #[doc = "Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "Active: This step is executed during the sequence"]
    VALUE2,
}
impl SEN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEN6W::VALUE1 => false,
            SEN6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _SEN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off: This step is not part of the stepper sequence"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEN6W::VALUE1)
    }
    #[doc = "Active: This step is executed during the sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEN6W::VALUE2)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SEN7`"]
pub enum SEN7W {
    #[doc = "Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "Active: This step is executed during the sequence"]
    VALUE2,
}
impl SEN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEN7W::VALUE1 => false,
            SEN7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _SEN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off: This step is not part of the stepper sequence"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEN7W::VALUE1)
    }
    #[doc = "Active: This step is executed during the sequence"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEN7W::VALUE2)
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
    #[doc = "Bits 0:2 - Kernel Select"]
    #[inline]
    pub fn ksel0(&self) -> KSEL0R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        KSEL0R { bits }
    }
    #[doc = "Bits 4:6 - Kernel Select"]
    #[inline]
    pub fn ksel1(&self) -> KSEL1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        KSEL1R { bits }
    }
    #[doc = "Bits 8:10 - Kernel Select"]
    #[inline]
    pub fn ksel2(&self) -> KSEL2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        KSEL2R { bits }
    }
    #[doc = "Bits 12:14 - Kernel Select"]
    #[inline]
    pub fn ksel3(&self) -> KSEL3R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        KSEL3R { bits }
    }
    #[doc = "Bits 16:18 - Kernel Select"]
    #[inline]
    pub fn ksel4(&self) -> KSEL4R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        KSEL4R { bits }
    }
    #[doc = "Bits 20:22 - Kernel Select"]
    #[inline]
    pub fn ksel5(&self) -> KSEL5R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        KSEL5R { bits }
    }
    #[doc = "Bits 24:26 - Kernel Select"]
    #[inline]
    pub fn ksel6(&self) -> KSEL6R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        KSEL6R { bits }
    }
    #[doc = "Bits 28:30 - Kernel Select"]
    #[inline]
    pub fn ksel7(&self) -> KSEL7R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        KSEL7R { bits }
    }
    #[doc = "Bit 3 - Step x Enable"]
    #[inline]
    pub fn sen0(&self) -> SEN0R {
        SEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Step x Enable"]
    #[inline]
    pub fn sen1(&self) -> SEN1R {
        SEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Step x Enable"]
    #[inline]
    pub fn sen2(&self) -> SEN2R {
        SEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Step x Enable"]
    #[inline]
    pub fn sen3(&self) -> SEN3R {
        SEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Step x Enable"]
    #[inline]
    pub fn sen4(&self) -> SEN4R {
        SEN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Step x Enable"]
    #[inline]
    pub fn sen5(&self) -> SEN5R {
        SEN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Step x Enable"]
    #[inline]
    pub fn sen6(&self) -> SEN6R {
        SEN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Step x Enable"]
    #[inline]
    pub fn sen7(&self) -> SEN7R {
        SEN7R::_from({
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
        W { bits: 152 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Kernel Select"]
    #[inline]
    pub fn ksel0(&mut self) -> _KSEL0W {
        _KSEL0W { w: self }
    }
    #[doc = "Bits 4:6 - Kernel Select"]
    #[inline]
    pub fn ksel1(&mut self) -> _KSEL1W {
        _KSEL1W { w: self }
    }
    #[doc = "Bits 8:10 - Kernel Select"]
    #[inline]
    pub fn ksel2(&mut self) -> _KSEL2W {
        _KSEL2W { w: self }
    }
    #[doc = "Bits 12:14 - Kernel Select"]
    #[inline]
    pub fn ksel3(&mut self) -> _KSEL3W {
        _KSEL3W { w: self }
    }
    #[doc = "Bits 16:18 - Kernel Select"]
    #[inline]
    pub fn ksel4(&mut self) -> _KSEL4W {
        _KSEL4W { w: self }
    }
    #[doc = "Bits 20:22 - Kernel Select"]
    #[inline]
    pub fn ksel5(&mut self) -> _KSEL5W {
        _KSEL5W { w: self }
    }
    #[doc = "Bits 24:26 - Kernel Select"]
    #[inline]
    pub fn ksel6(&mut self) -> _KSEL6W {
        _KSEL6W { w: self }
    }
    #[doc = "Bits 28:30 - Kernel Select"]
    #[inline]
    pub fn ksel7(&mut self) -> _KSEL7W {
        _KSEL7W { w: self }
    }
    #[doc = "Bit 3 - Step x Enable"]
    #[inline]
    pub fn sen0(&mut self) -> _SEN0W {
        _SEN0W { w: self }
    }
    #[doc = "Bit 7 - Step x Enable"]
    #[inline]
    pub fn sen1(&mut self) -> _SEN1W {
        _SEN1W { w: self }
    }
    #[doc = "Bit 11 - Step x Enable"]
    #[inline]
    pub fn sen2(&mut self) -> _SEN2W {
        _SEN2W { w: self }
    }
    #[doc = "Bit 15 - Step x Enable"]
    #[inline]
    pub fn sen3(&mut self) -> _SEN3W {
        _SEN3W { w: self }
    }
    #[doc = "Bit 19 - Step x Enable"]
    #[inline]
    pub fn sen4(&mut self) -> _SEN4W {
        _SEN4W { w: self }
    }
    #[doc = "Bit 23 - Step x Enable"]
    #[inline]
    pub fn sen5(&mut self) -> _SEN5W {
        _SEN5W { w: self }
    }
    #[doc = "Bit 27 - Step x Enable"]
    #[inline]
    pub fn sen6(&mut self) -> _SEN6W {
        _SEN6W { w: self }
    }
    #[doc = "Bit 31 - Step x Enable"]
    #[inline]
    pub fn sen7(&mut self) -> _SEN7W {
        _SEN7W { w: self }
    }
}
