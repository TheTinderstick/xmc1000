#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DESTRCON {
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
#[doc = "Possible values of the field `DE0S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DE0SR {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate target dimming level shadow transfer. The dimming process will start and the dimming level will change towards the target. Cleared by hardware when the dimming process is complete and the target has been reached."]
    VALUE2,
}
impl DE0SR {
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
            DE0SR::VALUE1 => false,
            DE0SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DE0SR {
        match value {
            false => DE0SR::VALUE1,
            true => DE0SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DE0SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DE0SR::VALUE2
    }
}
#[doc = "Possible values of the field `DE1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DE1SR {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate target dimming level shadow transfer. The dimming process will start and the dimming level will change towards the target. Cleared by hardware when the dimming process is complete and the target has been reached."]
    VALUE2,
}
impl DE1SR {
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
            DE1SR::VALUE1 => false,
            DE1SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DE1SR {
        match value {
            false => DE1SR::VALUE1,
            true => DE1SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DE1SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DE1SR::VALUE2
    }
}
#[doc = "Possible values of the field `DE2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DE2SR {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate target dimming level shadow transfer. The dimming process will start and the dimming level will change towards the target. Cleared by hardware when the dimming process is complete and the target has been reached."]
    VALUE2,
}
impl DE2SR {
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
            DE2SR::VALUE1 => false,
            DE2SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DE2SR {
        match value {
            false => DE2SR::VALUE1,
            true => DE2SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DE2SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DE2SR::VALUE2
    }
}
#[doc = "Values that can be written to the field `DE0S`"]
pub enum DE0SW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate target dimming level shadow transfer. The dimming process will start and the dimming level will change towards the target. Cleared by hardware when the dimming process is complete and the target has been reached."]
    VALUE2,
}
impl DE0SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DE0SW::VALUE1 => false,
            DE0SW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DE0SW<'a> {
    w: &'a mut W,
}
impl<'a> _DE0SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DE0SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DE0SW::VALUE1)
    }
    #[doc = "Initiate target dimming level shadow transfer. The dimming process will start and the dimming level will change towards the target. Cleared by hardware when the dimming process is complete and the target has been reached."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DE0SW::VALUE2)
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
#[doc = "Values that can be written to the field `DE1S`"]
pub enum DE1SW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate target dimming level shadow transfer. The dimming process will start and the dimming level will change towards the target. Cleared by hardware when the dimming process is complete and the target has been reached."]
    VALUE2,
}
impl DE1SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DE1SW::VALUE1 => false,
            DE1SW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DE1SW<'a> {
    w: &'a mut W,
}
impl<'a> _DE1SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DE1SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DE1SW::VALUE1)
    }
    #[doc = "Initiate target dimming level shadow transfer. The dimming process will start and the dimming level will change towards the target. Cleared by hardware when the dimming process is complete and the target has been reached."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DE1SW::VALUE2)
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
#[doc = "Values that can be written to the field `DE2S`"]
pub enum DE2SW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate target dimming level shadow transfer. The dimming process will start and the dimming level will change towards the target. Cleared by hardware when the dimming process is complete and the target has been reached."]
    VALUE2,
}
impl DE2SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DE2SW::VALUE1 => false,
            DE2SW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DE2SW<'a> {
    w: &'a mut W,
}
impl<'a> _DE2SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DE2SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DE2SW::VALUE1)
    }
    #[doc = "Initiate target dimming level shadow transfer. The dimming process will start and the dimming level will change towards the target. Cleared by hardware when the dimming process is complete and the target has been reached."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DE2SW::VALUE2)
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
#[doc = "Values that can be written to the field `DE0A`"]
pub enum DE0AW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Abort dimming; DEzS is cleared, BCCU_DLz.DLEV stops changing"]
    VALUE2,
}
impl DE0AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DE0AW::VALUE1 => false,
            DE0AW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DE0AW<'a> {
    w: &'a mut W,
}
impl<'a> _DE0AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DE0AW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DE0AW::VALUE1)
    }
    #[doc = "Abort dimming; DEzS is cleared, BCCU_DLz.DLEV stops changing"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DE0AW::VALUE2)
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
#[doc = "Values that can be written to the field `DE1A`"]
pub enum DE1AW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Abort dimming; DEzS is cleared, BCCU_DLz.DLEV stops changing"]
    VALUE2,
}
impl DE1AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DE1AW::VALUE1 => false,
            DE1AW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DE1AW<'a> {
    w: &'a mut W,
}
impl<'a> _DE1AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DE1AW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DE1AW::VALUE1)
    }
    #[doc = "Abort dimming; DEzS is cleared, BCCU_DLz.DLEV stops changing"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DE1AW::VALUE2)
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
#[doc = "Values that can be written to the field `DE2A`"]
pub enum DE2AW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Abort dimming; DEzS is cleared, BCCU_DLz.DLEV stops changing"]
    VALUE2,
}
impl DE2AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DE2AW::VALUE1 => false,
            DE2AW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DE2AW<'a> {
    w: &'a mut W,
}
impl<'a> _DE2AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DE2AW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DE2AW::VALUE1)
    }
    #[doc = "Abort dimming; DEzS is cleared, BCCU_DLz.DLEV stops changing"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DE2AW::VALUE2)
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
        const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - Dimming Engine 0 Shadow Transfer"]
    #[inline]
    pub fn de0s(&self) -> DE0SR {
        DE0SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Dimming Engine 1 Shadow Transfer"]
    #[inline]
    pub fn de1s(&self) -> DE1SR {
        DE1SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Dimming Engine 2 Shadow Transfer"]
    #[inline]
    pub fn de2s(&self) -> DE2SR {
        DE2SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Dimming Engine 0 Shadow Transfer"]
    #[inline]
    pub fn de0s(&mut self) -> _DE0SW {
        _DE0SW { w: self }
    }
    #[doc = "Bit 1 - Dimming Engine 1 Shadow Transfer"]
    #[inline]
    pub fn de1s(&mut self) -> _DE1SW {
        _DE1SW { w: self }
    }
    #[doc = "Bit 2 - Dimming Engine 2 Shadow Transfer"]
    #[inline]
    pub fn de2s(&mut self) -> _DE2SW {
        _DE2SW { w: self }
    }
    #[doc = "Bit 16 - Dimming Engine 0 Dimming Abort"]
    #[inline]
    pub fn de0a(&mut self) -> _DE0AW {
        _DE0AW { w: self }
    }
    #[doc = "Bit 17 - Dimming Engine 1 Dimming Abort"]
    #[inline]
    pub fn de1a(&mut self) -> _DE1AW {
        _DE1AW { w: self }
    }
    #[doc = "Bit 18 - Dimming Engine 2 Dimming Abort"]
    #[inline]
    pub fn de2a(&mut self) -> _DE2AW {
        _DE2AW { w: self }
    }
}
