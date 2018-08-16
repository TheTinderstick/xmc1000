#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RSTCON {
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
#[doc = "Possible values of the field `ECCRSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCRSTENR {
    #[doc = "No reset when ECC double bit error occur"]
    VALUE1,
    #[doc = "Reset when ECC double bit error occur"]
    VALUE2,
}
impl ECCRSTENR {
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
            ECCRSTENR::VALUE1 => false,
            ECCRSTENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECCRSTENR {
        match value {
            false => ECCRSTENR::VALUE1,
            true => ECCRSTENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECCRSTENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ECCRSTENR::VALUE2
    }
}
#[doc = "Possible values of the field `LOCRSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCRSTENR {
    #[doc = "No reset when loss of clock occur"]
    VALUE1,
    #[doc = "Reset when loss of clock occur"]
    VALUE2,
}
impl LOCRSTENR {
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
            LOCRSTENR::VALUE1 => false,
            LOCRSTENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCRSTENR {
        match value {
            false => LOCRSTENR::VALUE1,
            true => LOCRSTENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LOCRSTENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LOCRSTENR::VALUE2
    }
}
#[doc = "Possible values of the field `SPERSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPERSTENR {
    #[doc = "No reset when SRAM parity error occur"]
    VALUE1,
    #[doc = "Reset when SRAM parity error occur"]
    VALUE2,
}
impl SPERSTENR {
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
            SPERSTENR::VALUE1 => false,
            SPERSTENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPERSTENR {
        match value {
            false => SPERSTENR::VALUE1,
            true => SPERSTENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SPERSTENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SPERSTENR::VALUE2
    }
}
#[doc = "Possible values of the field `U0PERSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum U0PERSTENR {
    #[doc = "No reset when USIC0 memory parity error occur"]
    VALUE1,
    #[doc = "Reset when USIC0 memory parity error occur"]
    VALUE2,
}
impl U0PERSTENR {
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
            U0PERSTENR::VALUE1 => false,
            U0PERSTENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> U0PERSTENR {
        match value {
            false => U0PERSTENR::VALUE1,
            true => U0PERSTENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == U0PERSTENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == U0PERSTENR::VALUE2
    }
}
#[doc = "Values that can be written to the field `ECCRSTEN`"]
pub enum ECCRSTENW {
    #[doc = "No reset when ECC double bit error occur"]
    VALUE1,
    #[doc = "Reset when ECC double bit error occur"]
    VALUE2,
}
impl ECCRSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECCRSTENW::VALUE1 => false,
            ECCRSTENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECCRSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ECCRSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECCRSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset when ECC double bit error occur"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECCRSTENW::VALUE1)
    }
    #[doc = "Reset when ECC double bit error occur"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECCRSTENW::VALUE2)
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
#[doc = "Values that can be written to the field `LOCRSTEN`"]
pub enum LOCRSTENW {
    #[doc = "No reset when loss of clock occur"]
    VALUE1,
    #[doc = "Reset when loss of clock occur"]
    VALUE2,
}
impl LOCRSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCRSTENW::VALUE1 => false,
            LOCRSTENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCRSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCRSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCRSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset when loss of clock occur"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOCRSTENW::VALUE1)
    }
    #[doc = "Reset when loss of clock occur"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOCRSTENW::VALUE2)
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
#[doc = "Values that can be written to the field `SPERSTEN`"]
pub enum SPERSTENW {
    #[doc = "No reset when SRAM parity error occur"]
    VALUE1,
    #[doc = "Reset when SRAM parity error occur"]
    VALUE2,
}
impl SPERSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPERSTENW::VALUE1 => false,
            SPERSTENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPERSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPERSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPERSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset when SRAM parity error occur"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SPERSTENW::VALUE1)
    }
    #[doc = "Reset when SRAM parity error occur"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SPERSTENW::VALUE2)
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
#[doc = "Values that can be written to the field `U0PERSTEN`"]
pub enum U0PERSTENW {
    #[doc = "No reset when USIC0 memory parity error occur"]
    VALUE1,
    #[doc = "Reset when USIC0 memory parity error occur"]
    VALUE2,
}
impl U0PERSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            U0PERSTENW::VALUE1 => false,
            U0PERSTENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _U0PERSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _U0PERSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: U0PERSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset when USIC0 memory parity error occur"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(U0PERSTENW::VALUE1)
    }
    #[doc = "Reset when USIC0 memory parity error occur"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(U0PERSTENW::VALUE2)
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
#[doc = "Values that can be written to the field `MRSTEN`"]
pub enum MRSTENW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Triggered Master reset"]
    VALUE2,
}
impl MRSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MRSTENW::VALUE1 => false,
            MRSTENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MRSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _MRSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MRSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MRSTENW::VALUE1)
    }
    #[doc = "Triggered Master reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MRSTENW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable ECC Error Reset"]
    #[inline]
    pub fn eccrsten(&self) -> ECCRSTENR {
        ECCRSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable Loss of Clock Reset"]
    #[inline]
    pub fn locrsten(&self) -> LOCRSTENR {
        LOCRSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable 16kbytes SRAM Parity Error Reset"]
    #[inline]
    pub fn spersten(&self) -> SPERSTENR {
        SPERSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable USIC0 SRAM Parity Error Reset"]
    #[inline]
    pub fn u0persten(&self) -> U0PERSTENR {
        U0PERSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Enable ECC Error Reset"]
    #[inline]
    pub fn eccrsten(&mut self) -> _ECCRSTENW {
        _ECCRSTENW { w: self }
    }
    #[doc = "Bit 1 - Enable Loss of Clock Reset"]
    #[inline]
    pub fn locrsten(&mut self) -> _LOCRSTENW {
        _LOCRSTENW { w: self }
    }
    #[doc = "Bit 2 - Enable 16kbytes SRAM Parity Error Reset"]
    #[inline]
    pub fn spersten(&mut self) -> _SPERSTENW {
        _SPERSTENW { w: self }
    }
    #[doc = "Bit 3 - Enable USIC0 SRAM Parity Error Reset"]
    #[inline]
    pub fn u0persten(&mut self) -> _U0PERSTENW {
        _U0PERSTENW { w: self }
    }
    #[doc = "Bit 16 - Enable Master Reset"]
    #[inline]
    pub fn mrsten(&mut self) -> _MRSTENW {
        _MRSTENW { w: self }
    }
}
