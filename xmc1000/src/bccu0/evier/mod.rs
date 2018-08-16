#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVIER {
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
#[doc = "Possible values of the field `T0IEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T0IENR {
    #[doc = "Trigger 0 interrupt generation is disabled"]
    VALUE1,
    #[doc = "BCCU trigger 0 (BCCU_TRIGOUT0) generates an interrupt on SR0"]
    VALUE2,
}
impl T0IENR {
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
            T0IENR::VALUE1 => false,
            T0IENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> T0IENR {
        match value {
            false => T0IENR::VALUE1,
            true => T0IENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == T0IENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == T0IENR::VALUE2
    }
}
#[doc = "Possible values of the field `T1IEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T1IENR {
    #[doc = "Trigger 1 interrupt generation is disabled"]
    VALUE1,
    #[doc = "BCCU trigger 1 (BCCU_TRIGOUT1) generates an interrupt on SR0"]
    VALUE2,
}
impl T1IENR {
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
            T1IENR::VALUE1 => false,
            T1IENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> T1IENR {
        match value {
            false => T1IENR::VALUE1,
            true => T1IENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == T1IENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == T1IENR::VALUE2
    }
}
#[doc = "Possible values of the field `FIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIENR {
    #[doc = "FIFO-full interrupt generation is disabled"]
    VALUE1,
    #[doc = "An interrupt is generated on SR0 if any of the packer FIFOs is full when there is a write attempt by the on-time or off-time counter"]
    VALUE2,
}
impl FIENR {
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
            FIENR::VALUE1 => false,
            FIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIENR {
        match value {
            false => FIENR::VALUE1,
            true => FIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FIENR::VALUE2
    }
}
#[doc = "Possible values of the field `EIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIENR {
    #[doc = "FIFO-full interrupt generation is disabled"]
    VALUE1,
    #[doc = "An interrupt is generated on SR0 if any of the packer FIFOs is empty when there is a read attempt by the output generator"]
    VALUE2,
}
impl EIENR {
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
            EIENR::VALUE1 => false,
            EIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EIENR {
        match value {
            false => EIENR::VALUE1,
            true => EIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EIENR::VALUE2
    }
}
#[doc = "Possible values of the field `TPIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPIENR {
    #[doc = "Trap interrupt generation is disabled"]
    VALUE1,
    #[doc = "An interrupt is generated on SR0 if a trap occurs"]
    VALUE2,
}
impl TPIENR {
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
            TPIENR::VALUE1 => false,
            TPIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPIENR {
        match value {
            false => TPIENR::VALUE1,
            true => TPIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TPIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TPIENR::VALUE2
    }
}
#[doc = "Values that can be written to the field `T0IEN`"]
pub enum T0IENW {
    #[doc = "Trigger 0 interrupt generation is disabled"]
    VALUE1,
    #[doc = "BCCU trigger 0 (BCCU_TRIGOUT0) generates an interrupt on SR0"]
    VALUE2,
}
impl T0IENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            T0IENW::VALUE1 => false,
            T0IENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _T0IENW<'a> {
    w: &'a mut W,
}
impl<'a> _T0IENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: T0IENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger 0 interrupt generation is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(T0IENW::VALUE1)
    }
    #[doc = "BCCU trigger 0 (BCCU_TRIGOUT0) generates an interrupt on SR0"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(T0IENW::VALUE2)
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
#[doc = "Values that can be written to the field `T1IEN`"]
pub enum T1IENW {
    #[doc = "Trigger 1 interrupt generation is disabled"]
    VALUE1,
    #[doc = "BCCU trigger 1 (BCCU_TRIGOUT1) generates an interrupt on SR0"]
    VALUE2,
}
impl T1IENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            T1IENW::VALUE1 => false,
            T1IENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _T1IENW<'a> {
    w: &'a mut W,
}
impl<'a> _T1IENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: T1IENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger 1 interrupt generation is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(T1IENW::VALUE1)
    }
    #[doc = "BCCU trigger 1 (BCCU_TRIGOUT1) generates an interrupt on SR0"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(T1IENW::VALUE2)
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
#[doc = "Values that can be written to the field `FIEN`"]
pub enum FIENW {
    #[doc = "FIFO-full interrupt generation is disabled"]
    VALUE1,
    #[doc = "An interrupt is generated on SR0 if any of the packer FIFOs is full when there is a write attempt by the on-time or off-time counter"]
    VALUE2,
}
impl FIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIENW::VALUE1 => false,
            FIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIENW<'a> {
    w: &'a mut W,
}
impl<'a> _FIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO-full interrupt generation is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FIENW::VALUE1)
    }
    #[doc = "An interrupt is generated on SR0 if any of the packer FIFOs is full when there is a write attempt by the on-time or off-time counter"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FIENW::VALUE2)
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
#[doc = "Values that can be written to the field `EIEN`"]
pub enum EIENW {
    #[doc = "FIFO-full interrupt generation is disabled"]
    VALUE1,
    #[doc = "An interrupt is generated on SR0 if any of the packer FIFOs is empty when there is a read attempt by the output generator"]
    VALUE2,
}
impl EIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EIENW::VALUE1 => false,
            EIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EIENW<'a> {
    w: &'a mut W,
}
impl<'a> _EIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO-full interrupt generation is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EIENW::VALUE1)
    }
    #[doc = "An interrupt is generated on SR0 if any of the packer FIFOs is empty when there is a read attempt by the output generator"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EIENW::VALUE2)
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
#[doc = "Values that can be written to the field `TPIEN`"]
pub enum TPIENW {
    #[doc = "Trap interrupt generation is disabled"]
    VALUE1,
    #[doc = "An interrupt is generated on SR0 if a trap occurs"]
    VALUE2,
}
impl TPIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPIENW::VALUE1 => false,
            TPIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPIENW<'a> {
    w: &'a mut W,
}
impl<'a> _TPIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap interrupt generation is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TPIENW::VALUE1)
    }
    #[doc = "An interrupt is generated on SR0 if a trap occurs"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TPIENW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Trigger 0 Interrupt Enable"]
    #[inline]
    pub fn t0ien(&self) -> T0IENR {
        T0IENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Trigger 1 Interrupt Enable"]
    #[inline]
    pub fn t1ien(&self) -> T1IENR {
        T1IENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - FIFO Full Interrupt Enable"]
    #[inline]
    pub fn fien(&self) -> FIENR {
        FIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - FIFO Empty Interrupt Enable"]
    #[inline]
    pub fn eien(&self) -> EIENR {
        EIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Trap Interrupt Enable"]
    #[inline]
    pub fn tpien(&self) -> TPIENR {
        TPIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Trigger 0 Interrupt Enable"]
    #[inline]
    pub fn t0ien(&mut self) -> _T0IENW {
        _T0IENW { w: self }
    }
    #[doc = "Bit 1 - Trigger 1 Interrupt Enable"]
    #[inline]
    pub fn t1ien(&mut self) -> _T1IENW {
        _T1IENW { w: self }
    }
    #[doc = "Bit 2 - FIFO Full Interrupt Enable"]
    #[inline]
    pub fn fien(&mut self) -> _FIENW {
        _FIENW { w: self }
    }
    #[doc = "Bit 3 - FIFO Empty Interrupt Enable"]
    #[inline]
    pub fn eien(&mut self) -> _EIENW {
        _EIENW { w: self }
    }
    #[doc = "Bit 4 - Trap Interrupt Enable"]
    #[inline]
    pub fn tpien(&mut self) -> _TPIENW {
        _TPIENW { w: self }
    }
}
