#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::CHK {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RDV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDVR {
    #[doc = "New random data block is not yet ready to be read. In ) this flag is set to #0 while loading is in progress."]
    VALUE1,
    #[doc = "Random data block is valid. In key loading mode this value indicates that the next partial key word can be written to PRNG_WORD."]
    VALUE2,
}
impl RDVR {
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
            RDVR::VALUE1 => false,
            RDVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDVR {
        match value {
            false => RDVR::VALUE1,
            true => RDVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RDVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RDVR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Random Data / Key Valid Flag"]
    #[inline]
    pub fn rdv(&self) -> RDVR {
        RDVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
