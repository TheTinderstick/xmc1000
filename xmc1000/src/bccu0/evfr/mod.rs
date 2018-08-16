#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EVFR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `T0F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T0FR {
    #[doc = "No trigger event has been detected on BCCU trigger line 0 (BCCU_TRIGOUT0)"]
    VALUE1,
    #[doc = "A trigger event has been detected on BCCU trigger line 0 (BCCU_TRIGOUT0)"]
    VALUE2,
}
impl T0FR {
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
            T0FR::VALUE1 => false,
            T0FR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> T0FR {
        match value {
            false => T0FR::VALUE1,
            true => T0FR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == T0FR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == T0FR::VALUE2
    }
}
#[doc = "Possible values of the field `T1F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T1FR {
    #[doc = "No trigger event has been detected on BCCU trigger line 1 (BCCU_TRIGOUT1)"]
    VALUE1,
    #[doc = "A trigger event has been detected on BCCU trigger line 1 (BCCU_TRIGOUT1)"]
    VALUE2,
}
impl T1FR {
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
            T1FR::VALUE1 => false,
            T1FR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> T1FR {
        match value {
            false => T1FR::VALUE1,
            true => T1FR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == T1FR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == T1FR::VALUE2
    }
}
#[doc = "Possible values of the field `FF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFR {
    #[doc = "No FIFO full event has been detected"]
    VALUE1,
    #[doc = "A FIFO full event has been detected because one of the packer FIFOs is full and there has been a write attempt by the on-time or off-time counter"]
    VALUE2,
}
impl FFR {
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
            FFR::VALUE1 => false,
            FFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FFR {
        match value {
            false => FFR::VALUE1,
            true => FFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FFR::VALUE2
    }
}
#[doc = "Possible values of the field `EF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EFR {
    #[doc = "No FIFO full event has been detected"]
    VALUE1,
    #[doc = "A FIFO full event has been detected because one of the packer FIFOs is empty and there has been a read attempt by the output generator"]
    VALUE2,
}
impl EFR {
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
            EFR::VALUE1 => false,
            EFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EFR {
        match value {
            false => EFR::VALUE1,
            true => EFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EFR::VALUE2
    }
}
#[doc = "Possible values of the field `TPF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPFR {
    #[doc = "No trap event has been detected"]
    VALUE1,
    #[doc = "A trap event has been detected"]
    VALUE2,
}
impl TPFR {
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
            TPFR::VALUE1 => false,
            TPFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPFR {
        match value {
            false => TPFR::VALUE1,
            true => TPFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TPFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TPFR::VALUE2
    }
}
#[doc = "Possible values of the field `TPSF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPSFR {
    #[doc = "BCCU is not in a trap state"]
    VALUE1,
    #[doc = "BCCU is in a trap state, the affected channel outputs are at their passive levels"]
    VALUE2,
}
impl TPSFR {
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
            TPSFR::VALUE1 => false,
            TPSFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPSFR {
        match value {
            false => TPSFR::VALUE1,
            true => TPSFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TPSFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TPSFR::VALUE2
    }
}
#[doc = "Possible values of the field `TPINL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPINLR {
    #[doc = "The current level of BCCU.TRAPL is low"]
    VALUE1,
    #[doc = "The current level of BCCU.TRAPL is high"]
    VALUE2,
}
impl TPINLR {
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
            TPINLR::VALUE1 => false,
            TPINLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPINLR {
        match value {
            false => TPINLR::VALUE1,
            true => TPINLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TPINLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TPINLR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Trigger 0 Flag"]
    #[inline]
    pub fn t0f(&self) -> T0FR {
        T0FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Trigger 1 Flag"]
    #[inline]
    pub fn t1f(&self) -> T1FR {
        T1FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - FIFO Full Flag"]
    #[inline]
    pub fn ff(&self) -> FFR {
        FFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - FIFO Empty Flag"]
    #[inline]
    pub fn ef(&self) -> EFR {
        EFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Trap Flag"]
    #[inline]
    pub fn tpf(&self) -> TPFR {
        TPFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Trap State Flag"]
    #[inline]
    pub fn tpsf(&self) -> TPSFR {
        TPSFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Trap Input Level"]
    #[inline]
    pub fn tpinl(&self) -> TPINLR {
        TPINLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
