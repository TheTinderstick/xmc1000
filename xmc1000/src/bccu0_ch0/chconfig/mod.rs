#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHCONFIG {
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
pub struct PKTHR {
    bits: u8,
}
impl PKTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENR {
    #[doc = "The packer is not used"]
    VALUE1,
    #[doc = "On-time and off-time counters are running and the packed output bitstream with the packer trigger are generated."]
    VALUE2,
}
impl PENR {
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
            PENR::VALUE1 => false,
            PENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PENR {
        match value {
            false => PENR::VALUE1,
            true => PENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PENR::VALUE2
    }
}
#[doc = "Possible values of the field `DSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSELR {
    #[doc = "Dimming Engine 0"]
    VALUE1,
    #[doc = "Dimming Engine 1"]
    VALUE2,
    #[doc = "Dimming Engine 2"]
    VALUE3,
    #[doc = "Global Dimming Level"]
    VALUE8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSELR::VALUE1 => 0,
            DSELR::VALUE2 => 1,
            DSELR::VALUE3 => 2,
            DSELR::VALUE8 => 7,
            DSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSELR {
        match value {
            0 => DSELR::VALUE1,
            1 => DSELR::VALUE2,
            2 => DSELR::VALUE3,
            7 => DSELR::VALUE8,
            i => DSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DSELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DSELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == DSELR::VALUE8
    }
}
#[doc = "Possible values of the field `DBP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBPR {
    #[doc = "Channel brightness is the product of the selected dimming input and the channel intensity"]
    VALUE1,
    #[doc = "No dimming input is used, channel brightness is only determined by the channel intensity level"]
    VALUE2,
}
impl DBPR {
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
            DBPR::VALUE1 => false,
            DBPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBPR {
        match value {
            false => DBPR::VALUE1,
            true => DBPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DBPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DBPR::VALUE2
    }
}
#[doc = "Possible values of the field `GEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENR {
    #[doc = "Gating function is disabled, the input signal (BCCU.INy) has no effect"]
    VALUE1,
    #[doc = "Gating function is enabled, the output gating signal is BCCU.INy"]
    VALUE2,
}
impl GENR {
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
            GENR::VALUE1 => false,
            GENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GENR {
        match value {
            false => GENR::VALUE1,
            true => GENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GENR::VALUE2
    }
}
#[doc = "Possible values of the field `WEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WENR {
    #[doc = "The flicker watchdog is not used"]
    VALUE1,
    #[doc = "The flicker watchdog is active and limits the number of consecutive zeroes at the sigma-delta modulator output according to GLOBCON.WDMBN"]
    VALUE2,
}
impl WENR {
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
            WENR::VALUE1 => false,
            WENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WENR {
        match value {
            false => WENR::VALUE1,
            true => WENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WENR::VALUE2
    }
}
#[doc = "Possible values of the field `TRED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TREDR {
    #[doc = "Channel triggers occur on channel output transition from passive to active level"]
    VALUE1,
    #[doc = "Channel triggers occur on channel output transition from active to passive level"]
    VALUE2,
}
impl TREDR {
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
            TREDR::VALUE1 => false,
            TREDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TREDR {
        match value {
            false => TREDR::VALUE1,
            true => TREDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TREDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TREDR::VALUE2
    }
}
#[doc = "Possible values of the field `ENFT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENFTR {
    #[doc = "No forced trigger is generated"]
    VALUE1,
    #[doc = "The trigger generator generates a trigger if the output of the sigma-delta modulator hasn't changed state for 256 bit times; only takes effect if the packer is disabled (PEN=0)"]
    VALUE2,
}
impl ENFTR {
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
            ENFTR::VALUE1 => false,
            ENFTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENFTR {
        match value {
            false => ENFTR::VALUE1,
            true => ENFTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENFTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENFTR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct LINPRESR {
    bits: u16,
}
impl LINPRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PKTHW<'a> {
    w: &'a mut W,
}
impl<'a> _PKTHW<'a> {
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
#[doc = "Values that can be written to the field `PEN`"]
pub enum PENW {
    #[doc = "The packer is not used"]
    VALUE1,
    #[doc = "On-time and off-time counters are running and the packed output bitstream with the packer trigger are generated."]
    VALUE2,
}
impl PENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENW::VALUE1 => false,
            PENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENW<'a> {
    w: &'a mut W,
}
impl<'a> _PENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The packer is not used"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PENW::VALUE1)
    }
    #[doc = "On-time and off-time counters are running and the packed output bitstream with the packer trigger are generated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PENW::VALUE2)
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
#[doc = "Values that can be written to the field `DSEL`"]
pub enum DSELW {
    #[doc = "Dimming Engine 0"]
    VALUE1,
    #[doc = "Dimming Engine 1"]
    VALUE2,
    #[doc = "Dimming Engine 2"]
    VALUE3,
    #[doc = "Global Dimming Level"]
    VALUE8,
}
impl DSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSELW::VALUE1 => 0,
            DSELW::VALUE2 => 1,
            DSELW::VALUE3 => 2,
            DSELW::VALUE8 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Dimming Engine 0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSELW::VALUE1)
    }
    #[doc = "Dimming Engine 1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DSELW::VALUE2)
    }
    #[doc = "Dimming Engine 2"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DSELW::VALUE3)
    }
    #[doc = "Global Dimming Level"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(DSELW::VALUE8)
    }
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
#[doc = "Values that can be written to the field `DBP`"]
pub enum DBPW {
    #[doc = "Channel brightness is the product of the selected dimming input and the channel intensity"]
    VALUE1,
    #[doc = "No dimming input is used, channel brightness is only determined by the channel intensity level"]
    VALUE2,
}
impl DBPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBPW::VALUE1 => false,
            DBPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel brightness is the product of the selected dimming input and the channel intensity"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DBPW::VALUE1)
    }
    #[doc = "No dimming input is used, channel brightness is only determined by the channel intensity level"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DBPW::VALUE2)
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
#[doc = "Values that can be written to the field `GEN`"]
pub enum GENW {
    #[doc = "Gating function is disabled, the input signal (BCCU.INy) has no effect"]
    VALUE1,
    #[doc = "Gating function is enabled, the output gating signal is BCCU.INy"]
    VALUE2,
}
impl GENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GENW::VALUE1 => false,
            GENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GENW<'a> {
    w: &'a mut W,
}
impl<'a> _GENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Gating function is disabled, the input signal (BCCU.INy) has no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GENW::VALUE1)
    }
    #[doc = "Gating function is enabled, the output gating signal is BCCU.INy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GENW::VALUE2)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WEN`"]
pub enum WENW {
    #[doc = "The flicker watchdog is not used"]
    VALUE1,
    #[doc = "The flicker watchdog is active and limits the number of consecutive zeroes at the sigma-delta modulator output according to GLOBCON.WDMBN"]
    VALUE2,
}
impl WENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WENW::VALUE1 => false,
            WENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WENW<'a> {
    w: &'a mut W,
}
impl<'a> _WENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The flicker watchdog is not used"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WENW::VALUE1)
    }
    #[doc = "The flicker watchdog is active and limits the number of consecutive zeroes at the sigma-delta modulator output according to GLOBCON.WDMBN"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WENW::VALUE2)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRED`"]
pub enum TREDW {
    #[doc = "Channel triggers occur on channel output transition from passive to active level"]
    VALUE1,
    #[doc = "Channel triggers occur on channel output transition from active to passive level"]
    VALUE2,
}
impl TREDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TREDW::VALUE1 => false,
            TREDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TREDW<'a> {
    w: &'a mut W,
}
impl<'a> _TREDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TREDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel triggers occur on channel output transition from passive to active level"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TREDW::VALUE1)
    }
    #[doc = "Channel triggers occur on channel output transition from active to passive level"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TREDW::VALUE2)
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
#[doc = "Values that can be written to the field `ENFT`"]
pub enum ENFTW {
    #[doc = "No forced trigger is generated"]
    VALUE1,
    #[doc = "The trigger generator generates a trigger if the output of the sigma-delta modulator hasn't changed state for 256 bit times; only takes effect if the packer is disabled (PEN=0)"]
    VALUE2,
}
impl ENFTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENFTW::VALUE1 => false,
            ENFTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENFTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENFTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENFTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No forced trigger is generated"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENFTW::VALUE1)
    }
    #[doc = "The trigger generator generates a trigger if the output of the sigma-delta modulator hasn't changed state for 256 bit times; only takes effect if the packer is disabled (PEN=0)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENFTW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _LINPRESW<'a> {
    w: &'a mut W,
}
impl<'a> _LINPRESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
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
    #[doc = "Bits 0:2 - Packer Threshold"]
    #[inline]
    pub fn pkth(&self) -> PKTHR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PKTHR { bits }
    }
    #[doc = "Bit 3 - Packer Enable"]
    #[inline]
    pub fn pen(&self) -> PENR {
        PENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Dimming Select"]
    #[inline]
    pub fn dsel(&self) -> DSELR {
        DSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Dimming Input Bypass"]
    #[inline]
    pub fn dbp(&self) -> DBPR {
        DBPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Gating Enable"]
    #[inline]
    pub fn gen(&self) -> GENR {
        GENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Flicker Watchdog Enable"]
    #[inline]
    pub fn wen(&self) -> WENR {
        WENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Trigger Edge"]
    #[inline]
    pub fn tred(&self) -> TREDR {
        TREDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Forced Trigger Enable"]
    #[inline]
    pub fn enft(&self) -> ENFTR {
        ENFTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:25 - Linear Walker Clock Prescaler"]
    #[inline]
    pub fn linpres(&self) -> LINPRESR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        LINPRESR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Packer Threshold"]
    #[inline]
    pub fn pkth(&mut self) -> _PKTHW {
        _PKTHW { w: self }
    }
    #[doc = "Bit 3 - Packer Enable"]
    #[inline]
    pub fn pen(&mut self) -> _PENW {
        _PENW { w: self }
    }
    #[doc = "Bits 4:6 - Dimming Select"]
    #[inline]
    pub fn dsel(&mut self) -> _DSELW {
        _DSELW { w: self }
    }
    #[doc = "Bit 7 - Dimming Input Bypass"]
    #[inline]
    pub fn dbp(&mut self) -> _DBPW {
        _DBPW { w: self }
    }
    #[doc = "Bit 8 - Gating Enable"]
    #[inline]
    pub fn gen(&mut self) -> _GENW {
        _GENW { w: self }
    }
    #[doc = "Bit 9 - Flicker Watchdog Enable"]
    #[inline]
    pub fn wen(&mut self) -> _WENW {
        _WENW { w: self }
    }
    #[doc = "Bit 10 - Trigger Edge"]
    #[inline]
    pub fn tred(&mut self) -> _TREDW {
        _TREDW { w: self }
    }
    #[doc = "Bit 11 - Forced Trigger Enable"]
    #[inline]
    pub fn enft(&mut self) -> _ENFTW {
        _ENFTW { w: self }
    }
    #[doc = "Bits 16:25 - Linear Walker Clock Prescaler"]
    #[inline]
    pub fn linpres(&mut self) -> _LINPRESW {
        _LINPRESW { w: self }
    }
}
