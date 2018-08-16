#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GNCTR00 {
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
#[doc = "Possible values of the field `GAIN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN0R {
    #[doc = "Gain factor = 1"]
    VALUE1,
    #[doc = "Gain factor = 3"]
    VALUE2,
    #[doc = "Gain factor = 6"]
    VALUE3,
    #[doc = "Gain factor = 12"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GAIN0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GAIN0R::VALUE1 => 0,
            GAIN0R::VALUE2 => 1,
            GAIN0R::VALUE3 => 2,
            GAIN0R::VALUE4 => 3,
            GAIN0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GAIN0R {
        match value {
            0 => GAIN0R::VALUE1,
            1 => GAIN0R::VALUE2,
            2 => GAIN0R::VALUE3,
            3 => GAIN0R::VALUE4,
            i => GAIN0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GAIN0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GAIN0R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == GAIN0R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == GAIN0R::VALUE4
    }
}
#[doc = "Possible values of the field `GAIN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN1R {
    #[doc = "Gain factor = 1"]
    VALUE1,
    #[doc = "Gain factor = 3"]
    VALUE2,
    #[doc = "Gain factor = 6"]
    VALUE3,
    #[doc = "Gain factor = 12"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GAIN1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GAIN1R::VALUE1 => 0,
            GAIN1R::VALUE2 => 1,
            GAIN1R::VALUE3 => 2,
            GAIN1R::VALUE4 => 3,
            GAIN1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GAIN1R {
        match value {
            0 => GAIN1R::VALUE1,
            1 => GAIN1R::VALUE2,
            2 => GAIN1R::VALUE3,
            3 => GAIN1R::VALUE4,
            i => GAIN1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GAIN1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GAIN1R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == GAIN1R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == GAIN1R::VALUE4
    }
}
#[doc = "Possible values of the field `GAIN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN2R {
    #[doc = "Gain factor = 1"]
    VALUE1,
    #[doc = "Gain factor = 3"]
    VALUE2,
    #[doc = "Gain factor = 6"]
    VALUE3,
    #[doc = "Gain factor = 12"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GAIN2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GAIN2R::VALUE1 => 0,
            GAIN2R::VALUE2 => 1,
            GAIN2R::VALUE3 => 2,
            GAIN2R::VALUE4 => 3,
            GAIN2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GAIN2R {
        match value {
            0 => GAIN2R::VALUE1,
            1 => GAIN2R::VALUE2,
            2 => GAIN2R::VALUE3,
            3 => GAIN2R::VALUE4,
            i => GAIN2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GAIN2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GAIN2R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == GAIN2R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == GAIN2R::VALUE4
    }
}
#[doc = "Possible values of the field `GAIN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN3R {
    #[doc = "Gain factor = 1"]
    VALUE1,
    #[doc = "Gain factor = 3"]
    VALUE2,
    #[doc = "Gain factor = 6"]
    VALUE3,
    #[doc = "Gain factor = 12"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GAIN3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GAIN3R::VALUE1 => 0,
            GAIN3R::VALUE2 => 1,
            GAIN3R::VALUE3 => 2,
            GAIN3R::VALUE4 => 3,
            GAIN3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GAIN3R {
        match value {
            0 => GAIN3R::VALUE1,
            1 => GAIN3R::VALUE2,
            2 => GAIN3R::VALUE3,
            3 => GAIN3R::VALUE4,
            i => GAIN3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GAIN3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GAIN3R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == GAIN3R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == GAIN3R::VALUE4
    }
}
#[doc = "Possible values of the field `GAIN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN4R {
    #[doc = "Gain factor = 1"]
    VALUE1,
    #[doc = "Gain factor = 3"]
    VALUE2,
    #[doc = "Gain factor = 6"]
    VALUE3,
    #[doc = "Gain factor = 12"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GAIN4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GAIN4R::VALUE1 => 0,
            GAIN4R::VALUE2 => 1,
            GAIN4R::VALUE3 => 2,
            GAIN4R::VALUE4 => 3,
            GAIN4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GAIN4R {
        match value {
            0 => GAIN4R::VALUE1,
            1 => GAIN4R::VALUE2,
            2 => GAIN4R::VALUE3,
            3 => GAIN4R::VALUE4,
            i => GAIN4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GAIN4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GAIN4R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == GAIN4R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == GAIN4R::VALUE4
    }
}
#[doc = "Possible values of the field `GAIN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN5R {
    #[doc = "Gain factor = 1"]
    VALUE1,
    #[doc = "Gain factor = 3"]
    VALUE2,
    #[doc = "Gain factor = 6"]
    VALUE3,
    #[doc = "Gain factor = 12"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GAIN5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GAIN5R::VALUE1 => 0,
            GAIN5R::VALUE2 => 1,
            GAIN5R::VALUE3 => 2,
            GAIN5R::VALUE4 => 3,
            GAIN5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GAIN5R {
        match value {
            0 => GAIN5R::VALUE1,
            1 => GAIN5R::VALUE2,
            2 => GAIN5R::VALUE3,
            3 => GAIN5R::VALUE4,
            i => GAIN5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GAIN5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GAIN5R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == GAIN5R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == GAIN5R::VALUE4
    }
}
#[doc = "Possible values of the field `GAIN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN6R {
    #[doc = "Gain factor = 1"]
    VALUE1,
    #[doc = "Gain factor = 3"]
    VALUE2,
    #[doc = "Gain factor = 6"]
    VALUE3,
    #[doc = "Gain factor = 12"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GAIN6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GAIN6R::VALUE1 => 0,
            GAIN6R::VALUE2 => 1,
            GAIN6R::VALUE3 => 2,
            GAIN6R::VALUE4 => 3,
            GAIN6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GAIN6R {
        match value {
            0 => GAIN6R::VALUE1,
            1 => GAIN6R::VALUE2,
            2 => GAIN6R::VALUE3,
            3 => GAIN6R::VALUE4,
            i => GAIN6R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GAIN6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GAIN6R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == GAIN6R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == GAIN6R::VALUE4
    }
}
#[doc = "Possible values of the field `GAIN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN7R {
    #[doc = "Gain factor = 1"]
    VALUE1,
    #[doc = "Gain factor = 3"]
    VALUE2,
    #[doc = "Gain factor = 6"]
    VALUE3,
    #[doc = "Gain factor = 12"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GAIN7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GAIN7R::VALUE1 => 0,
            GAIN7R::VALUE2 => 1,
            GAIN7R::VALUE3 => 2,
            GAIN7R::VALUE4 => 3,
            GAIN7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GAIN7R {
        match value {
            0 => GAIN7R::VALUE1,
            1 => GAIN7R::VALUE2,
            2 => GAIN7R::VALUE3,
            3 => GAIN7R::VALUE4,
            i => GAIN7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GAIN7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GAIN7R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == GAIN7R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == GAIN7R::VALUE4
    }
}
#[doc = "Values that can be written to the field `GAIN0`"]
pub enum GAIN0W {
    #[doc = "Gain factor = 1"]
    VALUE1,
    #[doc = "Gain factor = 3"]
    VALUE2,
    #[doc = "Gain factor = 6"]
    VALUE3,
    #[doc = "Gain factor = 12"]
    VALUE4,
}
impl GAIN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GAIN0W::VALUE1 => 0,
            GAIN0W::VALUE2 => 1,
            GAIN0W::VALUE3 => 2,
            GAIN0W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GAIN0W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GAIN0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Gain factor = 1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAIN0W::VALUE1)
    }
    #[doc = "Gain factor = 3"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAIN0W::VALUE2)
    }
    #[doc = "Gain factor = 6"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAIN0W::VALUE3)
    }
    #[doc = "Gain factor = 12"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAIN0W::VALUE4)
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
#[doc = "Values that can be written to the field `GAIN1`"]
pub enum GAIN1W {
    #[doc = "Gain factor = 1"]
    VALUE1,
    #[doc = "Gain factor = 3"]
    VALUE2,
    #[doc = "Gain factor = 6"]
    VALUE3,
    #[doc = "Gain factor = 12"]
    VALUE4,
}
impl GAIN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GAIN1W::VALUE1 => 0,
            GAIN1W::VALUE2 => 1,
            GAIN1W::VALUE3 => 2,
            GAIN1W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GAIN1W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GAIN1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Gain factor = 1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAIN1W::VALUE1)
    }
    #[doc = "Gain factor = 3"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAIN1W::VALUE2)
    }
    #[doc = "Gain factor = 6"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAIN1W::VALUE3)
    }
    #[doc = "Gain factor = 12"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAIN1W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GAIN2`"]
pub enum GAIN2W {
    #[doc = "Gain factor = 1"]
    VALUE1,
    #[doc = "Gain factor = 3"]
    VALUE2,
    #[doc = "Gain factor = 6"]
    VALUE3,
    #[doc = "Gain factor = 12"]
    VALUE4,
}
impl GAIN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GAIN2W::VALUE1 => 0,
            GAIN2W::VALUE2 => 1,
            GAIN2W::VALUE3 => 2,
            GAIN2W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GAIN2W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GAIN2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Gain factor = 1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAIN2W::VALUE1)
    }
    #[doc = "Gain factor = 3"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAIN2W::VALUE2)
    }
    #[doc = "Gain factor = 6"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAIN2W::VALUE3)
    }
    #[doc = "Gain factor = 12"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAIN2W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GAIN3`"]
pub enum GAIN3W {
    #[doc = "Gain factor = 1"]
    VALUE1,
    #[doc = "Gain factor = 3"]
    VALUE2,
    #[doc = "Gain factor = 6"]
    VALUE3,
    #[doc = "Gain factor = 12"]
    VALUE4,
}
impl GAIN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GAIN3W::VALUE1 => 0,
            GAIN3W::VALUE2 => 1,
            GAIN3W::VALUE3 => 2,
            GAIN3W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GAIN3W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GAIN3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Gain factor = 1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAIN3W::VALUE1)
    }
    #[doc = "Gain factor = 3"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAIN3W::VALUE2)
    }
    #[doc = "Gain factor = 6"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAIN3W::VALUE3)
    }
    #[doc = "Gain factor = 12"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAIN3W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GAIN4`"]
pub enum GAIN4W {
    #[doc = "Gain factor = 1"]
    VALUE1,
    #[doc = "Gain factor = 3"]
    VALUE2,
    #[doc = "Gain factor = 6"]
    VALUE3,
    #[doc = "Gain factor = 12"]
    VALUE4,
}
impl GAIN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GAIN4W::VALUE1 => 0,
            GAIN4W::VALUE2 => 1,
            GAIN4W::VALUE3 => 2,
            GAIN4W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GAIN4W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GAIN4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Gain factor = 1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAIN4W::VALUE1)
    }
    #[doc = "Gain factor = 3"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAIN4W::VALUE2)
    }
    #[doc = "Gain factor = 6"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAIN4W::VALUE3)
    }
    #[doc = "Gain factor = 12"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAIN4W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GAIN5`"]
pub enum GAIN5W {
    #[doc = "Gain factor = 1"]
    VALUE1,
    #[doc = "Gain factor = 3"]
    VALUE2,
    #[doc = "Gain factor = 6"]
    VALUE3,
    #[doc = "Gain factor = 12"]
    VALUE4,
}
impl GAIN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GAIN5W::VALUE1 => 0,
            GAIN5W::VALUE2 => 1,
            GAIN5W::VALUE3 => 2,
            GAIN5W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GAIN5W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GAIN5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Gain factor = 1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAIN5W::VALUE1)
    }
    #[doc = "Gain factor = 3"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAIN5W::VALUE2)
    }
    #[doc = "Gain factor = 6"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAIN5W::VALUE3)
    }
    #[doc = "Gain factor = 12"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAIN5W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GAIN6`"]
pub enum GAIN6W {
    #[doc = "Gain factor = 1"]
    VALUE1,
    #[doc = "Gain factor = 3"]
    VALUE2,
    #[doc = "Gain factor = 6"]
    VALUE3,
    #[doc = "Gain factor = 12"]
    VALUE4,
}
impl GAIN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GAIN6W::VALUE1 => 0,
            GAIN6W::VALUE2 => 1,
            GAIN6W::VALUE3 => 2,
            GAIN6W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GAIN6W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GAIN6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Gain factor = 1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAIN6W::VALUE1)
    }
    #[doc = "Gain factor = 3"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAIN6W::VALUE2)
    }
    #[doc = "Gain factor = 6"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAIN6W::VALUE3)
    }
    #[doc = "Gain factor = 12"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAIN6W::VALUE4)
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
#[doc = "Values that can be written to the field `GAIN7`"]
pub enum GAIN7W {
    #[doc = "Gain factor = 1"]
    VALUE1,
    #[doc = "Gain factor = 3"]
    VALUE2,
    #[doc = "Gain factor = 6"]
    VALUE3,
    #[doc = "Gain factor = 12"]
    VALUE4,
}
impl GAIN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GAIN7W::VALUE1 => 0,
            GAIN7W::VALUE2 => 1,
            GAIN7W::VALUE3 => 2,
            GAIN7W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GAIN7W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GAIN7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Gain factor = 1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAIN7W::VALUE1)
    }
    #[doc = "Gain factor = 3"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAIN7W::VALUE2)
    }
    #[doc = "Gain factor = 6"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAIN7W::VALUE3)
    }
    #[doc = "Gain factor = 12"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAIN7W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - Gain Control 0"]
    #[inline]
    pub fn gain0(&self) -> GAIN0R {
        GAIN0R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Gain Control 1"]
    #[inline]
    pub fn gain1(&self) -> GAIN1R {
        GAIN1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Gain Control 2"]
    #[inline]
    pub fn gain2(&self) -> GAIN2R {
        GAIN2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Gain Control 3"]
    #[inline]
    pub fn gain3(&self) -> GAIN3R {
        GAIN3R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Gain Control 4"]
    #[inline]
    pub fn gain4(&self) -> GAIN4R {
        GAIN4R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Gain Control 5"]
    #[inline]
    pub fn gain5(&self) -> GAIN5R {
        GAIN5R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Gain Control 6"]
    #[inline]
    pub fn gain6(&self) -> GAIN6R {
        GAIN6R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Gain Control 7"]
    #[inline]
    pub fn gain7(&self) -> GAIN7R {
        GAIN7R::_from({
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
    #[doc = "Bits 0:3 - Gain Control 0"]
    #[inline]
    pub fn gain0(&mut self) -> _GAIN0W {
        _GAIN0W { w: self }
    }
    #[doc = "Bits 4:7 - Gain Control 1"]
    #[inline]
    pub fn gain1(&mut self) -> _GAIN1W {
        _GAIN1W { w: self }
    }
    #[doc = "Bits 8:11 - Gain Control 2"]
    #[inline]
    pub fn gain2(&mut self) -> _GAIN2W {
        _GAIN2W { w: self }
    }
    #[doc = "Bits 12:15 - Gain Control 3"]
    #[inline]
    pub fn gain3(&mut self) -> _GAIN3W {
        _GAIN3W { w: self }
    }
    #[doc = "Bits 16:19 - Gain Control 4"]
    #[inline]
    pub fn gain4(&mut self) -> _GAIN4W {
        _GAIN4W { w: self }
    }
    #[doc = "Bits 20:23 - Gain Control 5"]
    #[inline]
    pub fn gain5(&mut self) -> _GAIN5W {
        _GAIN5W { w: self }
    }
    #[doc = "Bits 24:27 - Gain Control 6"]
    #[inline]
    pub fn gain6(&mut self) -> _GAIN6W {
        _GAIN6W { w: self }
    }
    #[doc = "Bits 28:31 - Gain Control 7"]
    #[inline]
    pub fn gain7(&mut self) -> _GAIN7W {
        _GAIN7W { w: self }
    }
}
