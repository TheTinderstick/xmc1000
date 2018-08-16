#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GLOBCON {
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
#[doc = "Possible values of the field `TM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR {
    #[doc = "Mode 0: BCCU trigger occurs if there is any channel trigger (OR logic)"]
    VALUE1,
    #[doc = "Mode 1: BCCU trigger occurs if there is a channel trigger event on the active channel. When this happens, the next trigger-enabled channel will be active following the round robin."]
    VALUE2,
}
impl TMR {
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
            TMR::VALUE1 => false,
            TMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMR {
        match value {
            false => TMR::VALUE1,
            true => TMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TMR::VALUE2
    }
}
#[doc = "Possible values of the field `TRDEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRDELR {
    #[doc = "No delay"]
    VALUE1,
    #[doc = "BCCU trigger occurs a quarter bit time after the channel trigger that caused it; only to be used if BCCU_GLOBCLK.BCS is 0"]
    VALUE2,
    #[doc = "BCCU trigger occurs half a bit time after the channel trigger that caused it; only to be used if BCCU_GLOBCLK.BCS is 0"]
    VALUE3,
    #[doc = "No delay"]
    VALUE4,
}
impl TRDELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRDELR::VALUE1 => 0,
            TRDELR::VALUE2 => 1,
            TRDELR::VALUE3 => 2,
            TRDELR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRDELR {
        match value {
            0 => TRDELR::VALUE1,
            1 => TRDELR::VALUE2,
            2 => TRDELR::VALUE3,
            3 => TRDELR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TRDELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TRDELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TRDELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == TRDELR::VALUE4
    }
}
#[doc = "Possible values of the field `SUSCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSCFGR {
    #[doc = "Suspend request is ignored and the module cannot get suspended"]
    VALUE1,
    #[doc = "All channels stop running immediately and freeze in the last state without any safe stop"]
    VALUE2,
    #[doc = "All channels stop running immediately and freeze in the last state; all outputs go to passive state to achieve safe stop"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SUSCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SUSCFGR::VALUE1 => 0,
            SUSCFGR::VALUE2 => 1,
            SUSCFGR::VALUE3 => 2,
            SUSCFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SUSCFGR {
        match value {
            0 => SUSCFGR::VALUE1,
            1 => SUSCFGR::VALUE2,
            2 => SUSCFGR::VALUE3,
            i => SUSCFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SUSCFGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SUSCFGR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SUSCFGR::VALUE3
    }
}
#[doc = "Possible values of the field `TRAPIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRAPISR {
    #[doc = "BCCU.TRAPINA"]
    VALUE1,
    #[doc = "BCCU.TRAPINB"]
    VALUE2,
    #[doc = "BCCU.TRAPINC"]
    VALUE3,
    #[doc = "BCCU.TRAPIND"]
    VALUE4,
    #[doc = "BCCU.TRAPINE"]
    VALUE5,
    #[doc = "BCCU.TRAPINF"]
    VALUE6,
    #[doc = "BCCU.TRAPING"]
    VALUE7,
    #[doc = "BCCU.TRAPINH"]
    VALUE8,
    #[doc = "BCCU.TRAPINI"]
    VALUE9,
    #[doc = "BCCU.TRAPING"]
    VALUE10,
    #[doc = "BCCU.TRAPINK"]
    VALUE11,
    #[doc = "BCCU.TRAPINL"]
    VALUE12,
    #[doc = "BCCU.TRAPINM"]
    VALUE13,
    #[doc = "BCCU.TRAPINN"]
    VALUE14,
    #[doc = "BCCU.TRAPINO"]
    VALUE15,
    #[doc = "BCCU.TRAPINP"]
    VALUE16,
}
impl TRAPISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRAPISR::VALUE1 => 0,
            TRAPISR::VALUE2 => 1,
            TRAPISR::VALUE3 => 2,
            TRAPISR::VALUE4 => 3,
            TRAPISR::VALUE5 => 4,
            TRAPISR::VALUE6 => 5,
            TRAPISR::VALUE7 => 6,
            TRAPISR::VALUE8 => 7,
            TRAPISR::VALUE9 => 8,
            TRAPISR::VALUE10 => 9,
            TRAPISR::VALUE11 => 10,
            TRAPISR::VALUE12 => 11,
            TRAPISR::VALUE13 => 12,
            TRAPISR::VALUE14 => 13,
            TRAPISR::VALUE15 => 14,
            TRAPISR::VALUE16 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRAPISR {
        match value {
            0 => TRAPISR::VALUE1,
            1 => TRAPISR::VALUE2,
            2 => TRAPISR::VALUE3,
            3 => TRAPISR::VALUE4,
            4 => TRAPISR::VALUE5,
            5 => TRAPISR::VALUE6,
            6 => TRAPISR::VALUE7,
            7 => TRAPISR::VALUE8,
            8 => TRAPISR::VALUE9,
            9 => TRAPISR::VALUE10,
            10 => TRAPISR::VALUE11,
            11 => TRAPISR::VALUE12,
            12 => TRAPISR::VALUE13,
            13 => TRAPISR::VALUE14,
            14 => TRAPISR::VALUE15,
            15 => TRAPISR::VALUE16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TRAPISR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TRAPISR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TRAPISR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == TRAPISR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == TRAPISR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == TRAPISR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == TRAPISR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == TRAPISR::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == TRAPISR::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == TRAPISR::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == TRAPISR::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == TRAPISR::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == TRAPISR::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == TRAPISR::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == TRAPISR::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == TRAPISR::VALUE16
    }
}
#[doc = "Possible values of the field `TRAPED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRAPEDR {
    #[doc = "Trap occurs (trap flag is set) on rising edge of the BCCU.TRAPL signal"]
    VALUE1,
    #[doc = "Trap occurs (trap flag is set) on falling edge of the BCCU.TRAPL signal"]
    VALUE2,
}
impl TRAPEDR {
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
            TRAPEDR::VALUE1 => false,
            TRAPEDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRAPEDR {
        match value {
            false => TRAPEDR::VALUE1,
            true => TRAPEDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TRAPEDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TRAPEDR::VALUE2
    }
}
#[doc = "Possible values of the field `LTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTRSR {
    #[doc = "The last trigger occurred in channel turn 0"]
    VALUE1,
    #[doc = "The last trigger occurred in channel turn 1"]
    VALUE2,
    #[doc = "The last trigger occurred in channel turn 2"]
    VALUE3,
    #[doc = "The last trigger occurred in channel turn 3"]
    VALUE4,
    #[doc = "The last trigger occurred in channel turn 4"]
    VALUE5,
    #[doc = "The last trigger occurred in channel turn 5"]
    VALUE6,
    #[doc = "The last trigger occurred in channel turn 6"]
    VALUE7,
    #[doc = "The last trigger occurred in channel turn 7"]
    VALUE8,
    #[doc = "The last trigger occurred in channel turn 8"]
    VALUE9,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LTRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LTRSR::VALUE1 => 0,
            LTRSR::VALUE2 => 1,
            LTRSR::VALUE3 => 2,
            LTRSR::VALUE4 => 3,
            LTRSR::VALUE5 => 4,
            LTRSR::VALUE6 => 5,
            LTRSR::VALUE7 => 6,
            LTRSR::VALUE8 => 7,
            LTRSR::VALUE9 => 8,
            LTRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LTRSR {
        match value {
            0 => LTRSR::VALUE1,
            1 => LTRSR::VALUE2,
            2 => LTRSR::VALUE3,
            3 => LTRSR::VALUE4,
            4 => LTRSR::VALUE5,
            5 => LTRSR::VALUE6,
            6 => LTRSR::VALUE7,
            7 => LTRSR::VALUE8,
            8 => LTRSR::VALUE9,
            i => LTRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LTRSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LTRSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == LTRSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == LTRSR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == LTRSR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == LTRSR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == LTRSR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == LTRSR::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == LTRSR::VALUE9
    }
}
#[doc = r" Value of the field"]
pub struct WDMBNR {
    bits: u16,
}
impl WDMBNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TM`"]
pub enum TMW {
    #[doc = "Mode 0: BCCU trigger occurs if there is any channel trigger (OR logic)"]
    VALUE1,
    #[doc = "Mode 1: BCCU trigger occurs if there is a channel trigger event on the active channel. When this happens, the next trigger-enabled channel will be active following the round robin."]
    VALUE2,
}
impl TMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMW::VALUE1 => false,
            TMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMW<'a> {
    w: &'a mut W,
}
impl<'a> _TMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mode 0: BCCU trigger occurs if there is any channel trigger (OR logic)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TMW::VALUE1)
    }
    #[doc = "Mode 1: BCCU trigger occurs if there is a channel trigger event on the active channel. When this happens, the next trigger-enabled channel will be active following the round robin."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TMW::VALUE2)
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
#[doc = "Values that can be written to the field `TRDEL`"]
pub enum TRDELW {
    #[doc = "No delay"]
    VALUE1,
    #[doc = "BCCU trigger occurs a quarter bit time after the channel trigger that caused it; only to be used if BCCU_GLOBCLK.BCS is 0"]
    VALUE2,
    #[doc = "BCCU trigger occurs half a bit time after the channel trigger that caused it; only to be used if BCCU_GLOBCLK.BCS is 0"]
    VALUE3,
    #[doc = "No delay"]
    VALUE4,
}
impl TRDELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRDELW::VALUE1 => 0,
            TRDELW::VALUE2 => 1,
            TRDELW::VALUE3 => 2,
            TRDELW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRDELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRDELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRDELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No delay"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRDELW::VALUE1)
    }
    #[doc = "BCCU trigger occurs a quarter bit time after the channel trigger that caused it; only to be used if BCCU_GLOBCLK.BCS is 0"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRDELW::VALUE2)
    }
    #[doc = "BCCU trigger occurs half a bit time after the channel trigger that caused it; only to be used if BCCU_GLOBCLK.BCS is 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRDELW::VALUE3)
    }
    #[doc = "No delay"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(TRDELW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SUSCFG`"]
pub enum SUSCFGW {
    #[doc = "Suspend request is ignored and the module cannot get suspended"]
    VALUE1,
    #[doc = "All channels stop running immediately and freeze in the last state without any safe stop"]
    VALUE2,
    #[doc = "All channels stop running immediately and freeze in the last state; all outputs go to passive state to achieve safe stop"]
    VALUE3,
}
impl SUSCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SUSCFGW::VALUE1 => 0,
            SUSCFGW::VALUE2 => 1,
            SUSCFGW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUSCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUSCFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Suspend request is ignored and the module cannot get suspended"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUSCFGW::VALUE1)
    }
    #[doc = "All channels stop running immediately and freeze in the last state without any safe stop"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUSCFGW::VALUE2)
    }
    #[doc = "All channels stop running immediately and freeze in the last state; all outputs go to passive state to achieve safe stop"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SUSCFGW::VALUE3)
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
#[doc = "Values that can be written to the field `TRAPIS`"]
pub enum TRAPISW {
    #[doc = "BCCU.TRAPINA"]
    VALUE1,
    #[doc = "BCCU.TRAPINB"]
    VALUE2,
    #[doc = "BCCU.TRAPINC"]
    VALUE3,
    #[doc = "BCCU.TRAPIND"]
    VALUE4,
    #[doc = "BCCU.TRAPINE"]
    VALUE5,
    #[doc = "BCCU.TRAPINF"]
    VALUE6,
    #[doc = "BCCU.TRAPING"]
    VALUE7,
    #[doc = "BCCU.TRAPINH"]
    VALUE8,
    #[doc = "BCCU.TRAPINI"]
    VALUE9,
    #[doc = "BCCU.TRAPING"]
    VALUE10,
    #[doc = "BCCU.TRAPINK"]
    VALUE11,
    #[doc = "BCCU.TRAPINL"]
    VALUE12,
    #[doc = "BCCU.TRAPINM"]
    VALUE13,
    #[doc = "BCCU.TRAPINN"]
    VALUE14,
    #[doc = "BCCU.TRAPINO"]
    VALUE15,
    #[doc = "BCCU.TRAPINP"]
    VALUE16,
}
impl TRAPISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRAPISW::VALUE1 => 0,
            TRAPISW::VALUE2 => 1,
            TRAPISW::VALUE3 => 2,
            TRAPISW::VALUE4 => 3,
            TRAPISW::VALUE5 => 4,
            TRAPISW::VALUE6 => 5,
            TRAPISW::VALUE7 => 6,
            TRAPISW::VALUE8 => 7,
            TRAPISW::VALUE9 => 8,
            TRAPISW::VALUE10 => 9,
            TRAPISW::VALUE11 => 10,
            TRAPISW::VALUE12 => 11,
            TRAPISW::VALUE13 => 12,
            TRAPISW::VALUE14 => 13,
            TRAPISW::VALUE15 => 14,
            TRAPISW::VALUE16 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRAPISW<'a> {
    w: &'a mut W,
}
impl<'a> _TRAPISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRAPISW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "BCCU.TRAPINA"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRAPISW::VALUE1)
    }
    #[doc = "BCCU.TRAPINB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRAPISW::VALUE2)
    }
    #[doc = "BCCU.TRAPINC"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRAPISW::VALUE3)
    }
    #[doc = "BCCU.TRAPIND"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(TRAPISW::VALUE4)
    }
    #[doc = "BCCU.TRAPINE"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(TRAPISW::VALUE5)
    }
    #[doc = "BCCU.TRAPINF"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(TRAPISW::VALUE6)
    }
    #[doc = "BCCU.TRAPING"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(TRAPISW::VALUE7)
    }
    #[doc = "BCCU.TRAPINH"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(TRAPISW::VALUE8)
    }
    #[doc = "BCCU.TRAPINI"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(TRAPISW::VALUE9)
    }
    #[doc = "BCCU.TRAPING"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(TRAPISW::VALUE10)
    }
    #[doc = "BCCU.TRAPINK"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(TRAPISW::VALUE11)
    }
    #[doc = "BCCU.TRAPINL"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(TRAPISW::VALUE12)
    }
    #[doc = "BCCU.TRAPINM"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(TRAPISW::VALUE13)
    }
    #[doc = "BCCU.TRAPINN"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(TRAPISW::VALUE14)
    }
    #[doc = "BCCU.TRAPINO"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(TRAPISW::VALUE15)
    }
    #[doc = "BCCU.TRAPINP"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(TRAPISW::VALUE16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRAPED`"]
pub enum TRAPEDW {
    #[doc = "Trap occurs (trap flag is set) on rising edge of the BCCU.TRAPL signal"]
    VALUE1,
    #[doc = "Trap occurs (trap flag is set) on falling edge of the BCCU.TRAPL signal"]
    VALUE2,
}
impl TRAPEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRAPEDW::VALUE1 => false,
            TRAPEDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRAPEDW<'a> {
    w: &'a mut W,
}
impl<'a> _TRAPEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRAPEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap occurs (trap flag is set) on rising edge of the BCCU.TRAPL signal"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRAPEDW::VALUE1)
    }
    #[doc = "Trap occurs (trap flag is set) on falling edge of the BCCU.TRAPL signal"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRAPEDW::VALUE2)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WDMBNW<'a> {
    w: &'a mut W,
}
impl<'a> _WDMBNW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
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
    #[doc = "Bit 0 - Trigger Mode"]
    #[inline]
    pub fn tm(&self) -> TMR {
        TMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - Trigger Delay"]
    #[inline]
    pub fn trdel(&self) -> TRDELR {
        TRDELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Suspend Mode Configuration"]
    #[inline]
    pub fn suscfg(&self) -> SUSCFGR {
        SUSCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:9 - Trap Input Pin Selector"]
    #[inline]
    pub fn trapis(&self) -> TRAPISR {
        TRAPISR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Trap Edge"]
    #[inline]
    pub fn traped(&self) -> TRAPEDR {
        TRAPEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:15 - Last Trigger Source"]
    #[inline]
    pub fn ltrs(&self) -> LTRSR {
        LTRSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:27 - Watchdog Maximum Bitnumber"]
    #[inline]
    pub fn wdmbn(&self) -> WDMBNR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        WDMBNR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 52428800 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Trigger Mode"]
    #[inline]
    pub fn tm(&mut self) -> _TMW {
        _TMW { w: self }
    }
    #[doc = "Bits 2:3 - Trigger Delay"]
    #[inline]
    pub fn trdel(&mut self) -> _TRDELW {
        _TRDELW { w: self }
    }
    #[doc = "Bits 4:5 - Suspend Mode Configuration"]
    #[inline]
    pub fn suscfg(&mut self) -> _SUSCFGW {
        _SUSCFGW { w: self }
    }
    #[doc = "Bits 6:9 - Trap Input Pin Selector"]
    #[inline]
    pub fn trapis(&mut self) -> _TRAPISW {
        _TRAPISW { w: self }
    }
    #[doc = "Bit 10 - Trap Edge"]
    #[inline]
    pub fn traped(&mut self) -> _TRAPEDW {
        _TRAPEDW { w: self }
    }
    #[doc = "Bits 16:27 - Watchdog Maximum Bitnumber"]
    #[inline]
    pub fn wdmbn(&mut self) -> _WDMBNW {
        _WDMBNW { w: self }
    }
}
