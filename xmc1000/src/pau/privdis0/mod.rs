#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRIVDIS0 {
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
#[doc = "Possible values of the field `PDIS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS2R {
    #[doc = "Flash SFRs are accessible."]
    VALUE1,
    #[doc = "Flash SFRs are not accessible."]
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
#[doc = "Possible values of the field `PDIS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS5R {
    #[doc = "RAM Block 1 is accessible."]
    VALUE1,
    #[doc = "RAM Block 1 is not accessible."]
    VALUE2,
}
impl PDIS5R {
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
            PDIS5R::VALUE1 => false,
            PDIS5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS5R {
        match value {
            false => PDIS5R::VALUE1,
            true => PDIS5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS5R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS6R {
    #[doc = "RAM Block 2 is accessible."]
    VALUE1,
    #[doc = "RAM Block 2 is not accessible."]
    VALUE2,
}
impl PDIS6R {
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
            PDIS6R::VALUE1 => false,
            PDIS6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS6R {
        match value {
            false => PDIS6R::VALUE1,
            true => PDIS6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS6R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS7R {
    #[doc = "RAM Block 3 is accessible."]
    VALUE1,
    #[doc = "RAM Block 3 is not accessible."]
    VALUE2,
}
impl PDIS7R {
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
            PDIS7R::VALUE1 => false,
            PDIS7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS7R {
        match value {
            false => PDIS7R::VALUE1,
            true => PDIS7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS7R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS19R {
    #[doc = "WDT is accessible."]
    VALUE1,
    #[doc = "WDT is not accessible."]
    VALUE2,
}
impl PDIS19R {
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
            PDIS19R::VALUE1 => false,
            PDIS19R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS19R {
        match value {
            false => PDIS19R::VALUE1,
            true => PDIS19R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS19R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS19R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS20R {
    #[doc = "MATH Global SFRs and Divider are accessible."]
    VALUE1,
    #[doc = "MATH Global SFRs and Divider are not accessible."]
    VALUE2,
}
impl PDIS20R {
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
            PDIS20R::VALUE1 => false,
            PDIS20R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS20R {
        match value {
            false => PDIS20R::VALUE1,
            true => PDIS20R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS20R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS20R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS21R {
    #[doc = "MATH CORDIC is accessible."]
    VALUE1,
    #[doc = "MATH CORDIC is not accessible."]
    VALUE2,
}
impl PDIS21R {
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
            PDIS21R::VALUE1 => false,
            PDIS21R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS21R {
        match value {
            false => PDIS21R::VALUE1,
            true => PDIS21R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS21R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS21R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS22R {
    #[doc = "Port 0 is accessible."]
    VALUE1,
    #[doc = "Port 0 is not accessible."]
    VALUE2,
}
impl PDIS22R {
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
            PDIS22R::VALUE1 => false,
            PDIS22R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS22R {
        match value {
            false => PDIS22R::VALUE1,
            true => PDIS22R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS22R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS22R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS23R {
    #[doc = "Port 1 is accessible."]
    VALUE1,
    #[doc = "Port 1 is not accessible."]
    VALUE2,
}
impl PDIS23R {
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
            PDIS23R::VALUE1 => false,
            PDIS23R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS23R {
        match value {
            false => PDIS23R::VALUE1,
            true => PDIS23R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS23R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS23R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS24R {
    #[doc = "Port 2 is accessible."]
    VALUE1,
    #[doc = "Port 2 is not accessible."]
    VALUE2,
}
impl PDIS24R {
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
            PDIS24R::VALUE1 => false,
            PDIS24R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS24R {
        match value {
            false => PDIS24R::VALUE1,
            true => PDIS24R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS24R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS24R::VALUE2
    }
}
#[doc = "Values that can be written to the field `PDIS2`"]
pub enum PDIS2W {
    #[doc = "Flash SFRs are accessible."]
    VALUE1,
    #[doc = "Flash SFRs are not accessible."]
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
    #[doc = "Flash SFRs are accessible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS2W::VALUE1)
    }
    #[doc = "Flash SFRs are not accessible."]
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
#[doc = "Values that can be written to the field `PDIS5`"]
pub enum PDIS5W {
    #[doc = "RAM Block 1 is accessible."]
    VALUE1,
    #[doc = "RAM Block 1 is not accessible."]
    VALUE2,
}
impl PDIS5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS5W::VALUE1 => false,
            PDIS5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS5W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RAM Block 1 is accessible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS5W::VALUE1)
    }
    #[doc = "RAM Block 1 is not accessible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS5W::VALUE2)
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
#[doc = "Values that can be written to the field `PDIS6`"]
pub enum PDIS6W {
    #[doc = "RAM Block 2 is accessible."]
    VALUE1,
    #[doc = "RAM Block 2 is not accessible."]
    VALUE2,
}
impl PDIS6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS6W::VALUE1 => false,
            PDIS6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS6W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RAM Block 2 is accessible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS6W::VALUE1)
    }
    #[doc = "RAM Block 2 is not accessible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS6W::VALUE2)
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
#[doc = "Values that can be written to the field `PDIS7`"]
pub enum PDIS7W {
    #[doc = "RAM Block 3 is accessible."]
    VALUE1,
    #[doc = "RAM Block 3 is not accessible."]
    VALUE2,
}
impl PDIS7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS7W::VALUE1 => false,
            PDIS7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS7W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RAM Block 3 is accessible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS7W::VALUE1)
    }
    #[doc = "RAM Block 3 is not accessible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS7W::VALUE2)
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
#[doc = "Values that can be written to the field `PDIS19`"]
pub enum PDIS19W {
    #[doc = "WDT is accessible."]
    VALUE1,
    #[doc = "WDT is not accessible."]
    VALUE2,
}
impl PDIS19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS19W::VALUE1 => false,
            PDIS19W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS19W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WDT is accessible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS19W::VALUE1)
    }
    #[doc = "WDT is not accessible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS19W::VALUE2)
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
#[doc = "Values that can be written to the field `PDIS20`"]
pub enum PDIS20W {
    #[doc = "MATH Global SFRs and Divider are accessible."]
    VALUE1,
    #[doc = "MATH Global SFRs and Divider are not accessible."]
    VALUE2,
}
impl PDIS20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS20W::VALUE1 => false,
            PDIS20W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS20W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MATH Global SFRs and Divider are accessible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS20W::VALUE1)
    }
    #[doc = "MATH Global SFRs and Divider are not accessible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS20W::VALUE2)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS21`"]
pub enum PDIS21W {
    #[doc = "MATH CORDIC is accessible."]
    VALUE1,
    #[doc = "MATH CORDIC is not accessible."]
    VALUE2,
}
impl PDIS21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS21W::VALUE1 => false,
            PDIS21W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS21W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MATH CORDIC is accessible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS21W::VALUE1)
    }
    #[doc = "MATH CORDIC is not accessible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS21W::VALUE2)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS22`"]
pub enum PDIS22W {
    #[doc = "Port 0 is accessible."]
    VALUE1,
    #[doc = "Port 0 is not accessible."]
    VALUE2,
}
impl PDIS22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS22W::VALUE1 => false,
            PDIS22W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS22W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port 0 is accessible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS22W::VALUE1)
    }
    #[doc = "Port 0 is not accessible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS22W::VALUE2)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS23`"]
pub enum PDIS23W {
    #[doc = "Port 1 is accessible."]
    VALUE1,
    #[doc = "Port 1 is not accessible."]
    VALUE2,
}
impl PDIS23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS23W::VALUE1 => false,
            PDIS23W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS23W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port 1 is accessible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS23W::VALUE1)
    }
    #[doc = "Port 1 is not accessible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS23W::VALUE2)
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
#[doc = "Values that can be written to the field `PDIS24`"]
pub enum PDIS24W {
    #[doc = "Port 2 is accessible."]
    VALUE1,
    #[doc = "Port 2 is not accessible."]
    VALUE2,
}
impl PDIS24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS24W::VALUE1 => false,
            PDIS24W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS24W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port 2 is accessible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS24W::VALUE1)
    }
    #[doc = "Port 2 is not accessible."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS24W::VALUE2)
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
    #[doc = "Bit 2 - Flash SFRs Privilege Disable Flag"]
    #[inline]
    pub fn pdis2(&self) -> PDIS2R {
        PDIS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - RAM Block 1 Privilege Disable Flag"]
    #[inline]
    pub fn pdis5(&self) -> PDIS5R {
        PDIS5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - RAM Block 2 Privilege Disable Flag"]
    #[inline]
    pub fn pdis6(&self) -> PDIS6R {
        PDIS6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - RAM Block 3 Privilege Disable Flag"]
    #[inline]
    pub fn pdis7(&self) -> PDIS7R {
        PDIS7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - WDT Privilege Disable Flag"]
    #[inline]
    pub fn pdis19(&self) -> PDIS19R {
        PDIS19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - MATH Global SFRs and Divider Privilege Disable Flag"]
    #[inline]
    pub fn pdis20(&self) -> PDIS20R {
        PDIS20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - MATH CORDIC Privilege Disable Flag"]
    #[inline]
    pub fn pdis21(&self) -> PDIS21R {
        PDIS21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Port 0 Privilege Disable Flag"]
    #[inline]
    pub fn pdis22(&self) -> PDIS22R {
        PDIS22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Port 1 Privilege Disable Flag"]
    #[inline]
    pub fn pdis23(&self) -> PDIS23R {
        PDIS23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Port 2 Privilege Disable Flag"]
    #[inline]
    pub fn pdis24(&self) -> PDIS24R {
        PDIS24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
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
    #[doc = "Bit 2 - Flash SFRs Privilege Disable Flag"]
    #[inline]
    pub fn pdis2(&mut self) -> _PDIS2W {
        _PDIS2W { w: self }
    }
    #[doc = "Bit 5 - RAM Block 1 Privilege Disable Flag"]
    #[inline]
    pub fn pdis5(&mut self) -> _PDIS5W {
        _PDIS5W { w: self }
    }
    #[doc = "Bit 6 - RAM Block 2 Privilege Disable Flag"]
    #[inline]
    pub fn pdis6(&mut self) -> _PDIS6W {
        _PDIS6W { w: self }
    }
    #[doc = "Bit 7 - RAM Block 3 Privilege Disable Flag"]
    #[inline]
    pub fn pdis7(&mut self) -> _PDIS7W {
        _PDIS7W { w: self }
    }
    #[doc = "Bit 19 - WDT Privilege Disable Flag"]
    #[inline]
    pub fn pdis19(&mut self) -> _PDIS19W {
        _PDIS19W { w: self }
    }
    #[doc = "Bit 20 - MATH Global SFRs and Divider Privilege Disable Flag"]
    #[inline]
    pub fn pdis20(&mut self) -> _PDIS20W {
        _PDIS20W { w: self }
    }
    #[doc = "Bit 21 - MATH CORDIC Privilege Disable Flag"]
    #[inline]
    pub fn pdis21(&mut self) -> _PDIS21W {
        _PDIS21W { w: self }
    }
    #[doc = "Bit 22 - Port 0 Privilege Disable Flag"]
    #[inline]
    pub fn pdis22(&mut self) -> _PDIS22W {
        _PDIS22W { w: self }
    }
    #[doc = "Bit 23 - Port 1 Privilege Disable Flag"]
    #[inline]
    pub fn pdis23(&mut self) -> _PDIS23W {
        _PDIS23W { w: self }
    }
    #[doc = "Bit 24 - Port 2 Privilege Disable Flag"]
    #[inline]
    pub fn pdis24(&mut self) -> _PDIS24W {
        _PDIS24W { w: self }
    }
}
