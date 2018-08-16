#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HWSEL {
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
#[doc = "Possible values of the field `HW0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW0R {
    #[doc = "Software control only."]
    VALUE1,
    #[doc = "HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "HW1 control path can override the software configuration."]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW0R::VALUE1 => 0,
            HW0R::VALUE2 => 1,
            HW0R::VALUE3 => 2,
            HW0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW0R {
        match value {
            0 => HW0R::VALUE1,
            1 => HW0R::VALUE2,
            2 => HW0R::VALUE3,
            i => HW0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HW0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HW0R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == HW0R::VALUE3
    }
}
#[doc = "Possible values of the field `HW1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW1R {
    #[doc = "Software control only."]
    VALUE1,
    #[doc = "HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "HW1 control path can override the software configuration."]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW1R::VALUE1 => 0,
            HW1R::VALUE2 => 1,
            HW1R::VALUE3 => 2,
            HW1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW1R {
        match value {
            0 => HW1R::VALUE1,
            1 => HW1R::VALUE2,
            2 => HW1R::VALUE3,
            i => HW1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HW1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HW1R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == HW1R::VALUE3
    }
}
#[doc = "Possible values of the field `HW2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW2R {
    #[doc = "Software control only."]
    VALUE1,
    #[doc = "HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "HW1 control path can override the software configuration."]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW2R::VALUE1 => 0,
            HW2R::VALUE2 => 1,
            HW2R::VALUE3 => 2,
            HW2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW2R {
        match value {
            0 => HW2R::VALUE1,
            1 => HW2R::VALUE2,
            2 => HW2R::VALUE3,
            i => HW2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HW2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HW2R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == HW2R::VALUE3
    }
}
#[doc = "Possible values of the field `HW3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW3R {
    #[doc = "Software control only."]
    VALUE1,
    #[doc = "HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "HW1 control path can override the software configuration."]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW3R::VALUE1 => 0,
            HW3R::VALUE2 => 1,
            HW3R::VALUE3 => 2,
            HW3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW3R {
        match value {
            0 => HW3R::VALUE1,
            1 => HW3R::VALUE2,
            2 => HW3R::VALUE3,
            i => HW3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HW3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HW3R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == HW3R::VALUE3
    }
}
#[doc = "Possible values of the field `HW4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW4R {
    #[doc = "Software control only."]
    VALUE1,
    #[doc = "HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "HW1 control path can override the software configuration."]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW4R::VALUE1 => 0,
            HW4R::VALUE2 => 1,
            HW4R::VALUE3 => 2,
            HW4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW4R {
        match value {
            0 => HW4R::VALUE1,
            1 => HW4R::VALUE2,
            2 => HW4R::VALUE3,
            i => HW4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HW4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HW4R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == HW4R::VALUE3
    }
}
#[doc = "Possible values of the field `HW5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW5R {
    #[doc = "Software control only."]
    VALUE1,
    #[doc = "HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "HW1 control path can override the software configuration."]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW5R::VALUE1 => 0,
            HW5R::VALUE2 => 1,
            HW5R::VALUE3 => 2,
            HW5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW5R {
        match value {
            0 => HW5R::VALUE1,
            1 => HW5R::VALUE2,
            2 => HW5R::VALUE3,
            i => HW5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HW5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HW5R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == HW5R::VALUE3
    }
}
#[doc = "Possible values of the field `HW6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW6R {
    #[doc = "Software control only."]
    VALUE1,
    #[doc = "HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "HW1 control path can override the software configuration."]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HW6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HW6R::VALUE1 => 0,
            HW6R::VALUE2 => 1,
            HW6R::VALUE3 => 2,
            HW6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HW6R {
        match value {
            0 => HW6R::VALUE1,
            1 => HW6R::VALUE2,
            2 => HW6R::VALUE3,
            i => HW6R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HW6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HW6R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == HW6R::VALUE3
    }
}
#[doc = "Values that can be written to the field `HW0`"]
pub enum HW0W {
    #[doc = "Software control only."]
    VALUE1,
    #[doc = "HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "HW1 control path can override the software configuration."]
    VALUE3,
}
impl HW0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW0W::VALUE1 => 0,
            HW0W::VALUE2 => 1,
            HW0W::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW0W<'a> {
    w: &'a mut W,
}
impl<'a> _HW0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HW0W::VALUE1)
    }
    #[doc = "HW0 control path can override the software configuration."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HW0W::VALUE2)
    }
    #[doc = "HW1 control path can override the software configuration."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(HW0W::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW1`"]
pub enum HW1W {
    #[doc = "Software control only."]
    VALUE1,
    #[doc = "HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "HW1 control path can override the software configuration."]
    VALUE3,
}
impl HW1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW1W::VALUE1 => 0,
            HW1W::VALUE2 => 1,
            HW1W::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW1W<'a> {
    w: &'a mut W,
}
impl<'a> _HW1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HW1W::VALUE1)
    }
    #[doc = "HW0 control path can override the software configuration."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HW1W::VALUE2)
    }
    #[doc = "HW1 control path can override the software configuration."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(HW1W::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW2`"]
pub enum HW2W {
    #[doc = "Software control only."]
    VALUE1,
    #[doc = "HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "HW1 control path can override the software configuration."]
    VALUE3,
}
impl HW2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW2W::VALUE1 => 0,
            HW2W::VALUE2 => 1,
            HW2W::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW2W<'a> {
    w: &'a mut W,
}
impl<'a> _HW2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HW2W::VALUE1)
    }
    #[doc = "HW0 control path can override the software configuration."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HW2W::VALUE2)
    }
    #[doc = "HW1 control path can override the software configuration."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(HW2W::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW3`"]
pub enum HW3W {
    #[doc = "Software control only."]
    VALUE1,
    #[doc = "HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "HW1 control path can override the software configuration."]
    VALUE3,
}
impl HW3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW3W::VALUE1 => 0,
            HW3W::VALUE2 => 1,
            HW3W::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW3W<'a> {
    w: &'a mut W,
}
impl<'a> _HW3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HW3W::VALUE1)
    }
    #[doc = "HW0 control path can override the software configuration."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HW3W::VALUE2)
    }
    #[doc = "HW1 control path can override the software configuration."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(HW3W::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW4`"]
pub enum HW4W {
    #[doc = "Software control only."]
    VALUE1,
    #[doc = "HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "HW1 control path can override the software configuration."]
    VALUE3,
}
impl HW4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW4W::VALUE1 => 0,
            HW4W::VALUE2 => 1,
            HW4W::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW4W<'a> {
    w: &'a mut W,
}
impl<'a> _HW4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HW4W::VALUE1)
    }
    #[doc = "HW0 control path can override the software configuration."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HW4W::VALUE2)
    }
    #[doc = "HW1 control path can override the software configuration."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(HW4W::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HW5`"]
pub enum HW5W {
    #[doc = "Software control only."]
    VALUE1,
    #[doc = "HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "HW1 control path can override the software configuration."]
    VALUE3,
}
impl HW5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW5W::VALUE1 => 0,
            HW5W::VALUE2 => 1,
            HW5W::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW5W<'a> {
    w: &'a mut W,
}
impl<'a> _HW5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HW5W::VALUE1)
    }
    #[doc = "HW0 control path can override the software configuration."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HW5W::VALUE2)
    }
    #[doc = "HW1 control path can override the software configuration."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(HW5W::VALUE3)
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
#[doc = "Values that can be written to the field `HW6`"]
pub enum HW6W {
    #[doc = "Software control only."]
    VALUE1,
    #[doc = "HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "HW1 control path can override the software configuration."]
    VALUE3,
}
impl HW6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HW6W::VALUE1 => 0,
            HW6W::VALUE2 => 1,
            HW6W::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW6W<'a> {
    w: &'a mut W,
}
impl<'a> _HW6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software control only."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HW6W::VALUE1)
    }
    #[doc = "HW0 control path can override the software configuration."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HW6W::VALUE2)
    }
    #[doc = "HW1 control path can override the software configuration."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(HW6W::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:1 - Port 1 Pin Hardware Select Bit 0"]
    #[inline]
    pub fn hw0(&self) -> HW0R {
        HW0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Port 1 Pin Hardware Select Bit 1"]
    #[inline]
    pub fn hw1(&self) -> HW1R {
        HW1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Port 1 Pin Hardware Select Bit 2"]
    #[inline]
    pub fn hw2(&self) -> HW2R {
        HW2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Port 1 Pin Hardware Select Bit 3"]
    #[inline]
    pub fn hw3(&self) -> HW3R {
        HW3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Port 1 Pin Hardware Select Bit 4"]
    #[inline]
    pub fn hw4(&self) -> HW4R {
        HW4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Port 1 Pin Hardware Select Bit 5"]
    #[inline]
    pub fn hw5(&self) -> HW5R {
        HW5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Port 1 Pin Hardware Select Bit 6"]
    #[inline]
    pub fn hw6(&self) -> HW6R {
        HW6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:1 - Port 1 Pin Hardware Select Bit 0"]
    #[inline]
    pub fn hw0(&mut self) -> _HW0W {
        _HW0W { w: self }
    }
    #[doc = "Bits 2:3 - Port 1 Pin Hardware Select Bit 1"]
    #[inline]
    pub fn hw1(&mut self) -> _HW1W {
        _HW1W { w: self }
    }
    #[doc = "Bits 4:5 - Port 1 Pin Hardware Select Bit 2"]
    #[inline]
    pub fn hw2(&mut self) -> _HW2W {
        _HW2W { w: self }
    }
    #[doc = "Bits 6:7 - Port 1 Pin Hardware Select Bit 3"]
    #[inline]
    pub fn hw3(&mut self) -> _HW3W {
        _HW3W { w: self }
    }
    #[doc = "Bits 8:9 - Port 1 Pin Hardware Select Bit 4"]
    #[inline]
    pub fn hw4(&mut self) -> _HW4W {
        _HW4W { w: self }
    }
    #[doc = "Bits 10:11 - Port 1 Pin Hardware Select Bit 5"]
    #[inline]
    pub fn hw5(&mut self) -> _HW5W {
        _HW5W { w: self }
    }
    #[doc = "Bits 12:13 - Port 1 Pin Hardware Select Bit 6"]
    #[inline]
    pub fn hw6(&mut self) -> _HW6W {
        _HW6W { w: self }
    }
}
