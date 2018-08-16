#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GLBCON {
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
#[doc = "Possible values of the field `DVDRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DVDRCR {
    #[doc = "No result chaining is selected"]
    VALUE1,
    #[doc = "QUOT register is the selected source"]
    VALUE2,
    #[doc = "RMD register is the selected source"]
    VALUE3,
    #[doc = "CORRX is the selected source"]
    VALUE4,
    #[doc = "CORRY is the selected source"]
    VALUE5,
    #[doc = "CORRZ is the selected source"]
    VALUE6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DVDRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DVDRCR::VALUE1 => 0,
            DVDRCR::VALUE2 => 1,
            DVDRCR::VALUE3 => 2,
            DVDRCR::VALUE4 => 3,
            DVDRCR::VALUE5 => 4,
            DVDRCR::VALUE6 => 5,
            DVDRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DVDRCR {
        match value {
            0 => DVDRCR::VALUE1,
            1 => DVDRCR::VALUE2,
            2 => DVDRCR::VALUE3,
            3 => DVDRCR::VALUE4,
            4 => DVDRCR::VALUE5,
            5 => DVDRCR::VALUE6,
            i => DVDRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DVDRCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DVDRCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DVDRCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == DVDRCR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == DVDRCR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == DVDRCR::VALUE6
    }
}
#[doc = "Possible values of the field `DVSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DVSRCR {
    #[doc = "No result chaining is selected"]
    VALUE1,
    #[doc = "QUOT register is the selected source"]
    VALUE2,
    #[doc = "RMD register is the selected source"]
    VALUE3,
    #[doc = "CORRX is the selected source"]
    VALUE4,
    #[doc = "CORRY is the selected source"]
    VALUE5,
    #[doc = "CORRZ is the selected source"]
    VALUE6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DVSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DVSRCR::VALUE1 => 0,
            DVSRCR::VALUE2 => 1,
            DVSRCR::VALUE3 => 2,
            DVSRCR::VALUE4 => 3,
            DVSRCR::VALUE5 => 4,
            DVSRCR::VALUE6 => 5,
            DVSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DVSRCR {
        match value {
            0 => DVSRCR::VALUE1,
            1 => DVSRCR::VALUE2,
            2 => DVSRCR::VALUE3,
            3 => DVSRCR::VALUE4,
            4 => DVSRCR::VALUE5,
            5 => DVSRCR::VALUE6,
            i => DVSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DVSRCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DVSRCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DVSRCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == DVSRCR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == DVSRCR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == DVSRCR::VALUE6
    }
}
#[doc = "Possible values of the field `CORDXRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORDXRCR {
    #[doc = "No result chaining is selected"]
    VALUE1,
    #[doc = "QUOT register is the selected source"]
    VALUE2,
    #[doc = "RMD register is the selected source"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CORDXRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CORDXRCR::VALUE1 => 0,
            CORDXRCR::VALUE2 => 1,
            CORDXRCR::VALUE3 => 2,
            CORDXRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CORDXRCR {
        match value {
            0 => CORDXRCR::VALUE1,
            1 => CORDXRCR::VALUE2,
            2 => CORDXRCR::VALUE3,
            i => CORDXRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CORDXRCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CORDXRCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CORDXRCR::VALUE3
    }
}
#[doc = "Possible values of the field `CORDYRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORDYRCR {
    #[doc = "No result chaining is selected"]
    VALUE1,
    #[doc = "QUOT register is the selected source"]
    VALUE2,
    #[doc = "RMD register is the selected source"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CORDYRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CORDYRCR::VALUE1 => 0,
            CORDYRCR::VALUE2 => 1,
            CORDYRCR::VALUE3 => 2,
            CORDYRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CORDYRCR {
        match value {
            0 => CORDYRCR::VALUE1,
            1 => CORDYRCR::VALUE2,
            2 => CORDYRCR::VALUE3,
            i => CORDYRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CORDYRCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CORDYRCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CORDYRCR::VALUE3
    }
}
#[doc = "Possible values of the field `CORDZRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORDZRCR {
    #[doc = "No result chaining is selected"]
    VALUE1,
    #[doc = "QUOT register is the selected source"]
    VALUE2,
    #[doc = "RMD register is the selected source"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CORDZRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CORDZRCR::VALUE1 => 0,
            CORDZRCR::VALUE2 => 1,
            CORDZRCR::VALUE3 => 2,
            CORDZRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CORDZRCR {
        match value {
            0 => CORDZRCR::VALUE1,
            1 => CORDZRCR::VALUE2,
            2 => CORDZRCR::VALUE3,
            i => CORDZRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CORDZRCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CORDZRCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CORDZRCR::VALUE3
    }
}
#[doc = "Possible values of the field `SUSCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSCFGR {
    #[doc = "Suspend mode is never entered."]
    VALUE1,
    #[doc = "Hard suspend mode will be entered when CPU is halted."]
    VALUE2,
    #[doc = "Soft suspend mode will be entered when CPU is halted."]
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
#[doc = "Values that can be written to the field `DVDRC`"]
pub enum DVDRCW {
    #[doc = "No result chaining is selected"]
    VALUE1,
    #[doc = "QUOT register is the selected source"]
    VALUE2,
    #[doc = "RMD register is the selected source"]
    VALUE3,
    #[doc = "CORRX is the selected source"]
    VALUE4,
    #[doc = "CORRY is the selected source"]
    VALUE5,
    #[doc = "CORRZ is the selected source"]
    VALUE6,
}
impl DVDRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DVDRCW::VALUE1 => 0,
            DVDRCW::VALUE2 => 1,
            DVDRCW::VALUE3 => 2,
            DVDRCW::VALUE4 => 3,
            DVDRCW::VALUE5 => 4,
            DVDRCW::VALUE6 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DVDRCW<'a> {
    w: &'a mut W,
}
impl<'a> _DVDRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DVDRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No result chaining is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DVDRCW::VALUE1)
    }
    #[doc = "QUOT register is the selected source"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DVDRCW::VALUE2)
    }
    #[doc = "RMD register is the selected source"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DVDRCW::VALUE3)
    }
    #[doc = "CORRX is the selected source"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(DVDRCW::VALUE4)
    }
    #[doc = "CORRY is the selected source"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(DVDRCW::VALUE5)
    }
    #[doc = "CORRZ is the selected source"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(DVDRCW::VALUE6)
    }
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
#[doc = "Values that can be written to the field `DVSRC`"]
pub enum DVSRCW {
    #[doc = "No result chaining is selected"]
    VALUE1,
    #[doc = "QUOT register is the selected source"]
    VALUE2,
    #[doc = "RMD register is the selected source"]
    VALUE3,
    #[doc = "CORRX is the selected source"]
    VALUE4,
    #[doc = "CORRY is the selected source"]
    VALUE5,
    #[doc = "CORRZ is the selected source"]
    VALUE6,
}
impl DVSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DVSRCW::VALUE1 => 0,
            DVSRCW::VALUE2 => 1,
            DVSRCW::VALUE3 => 2,
            DVSRCW::VALUE4 => 3,
            DVSRCW::VALUE5 => 4,
            DVSRCW::VALUE6 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DVSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _DVSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DVSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No result chaining is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DVSRCW::VALUE1)
    }
    #[doc = "QUOT register is the selected source"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DVSRCW::VALUE2)
    }
    #[doc = "RMD register is the selected source"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DVSRCW::VALUE3)
    }
    #[doc = "CORRX is the selected source"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(DVSRCW::VALUE4)
    }
    #[doc = "CORRY is the selected source"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(DVSRCW::VALUE5)
    }
    #[doc = "CORRZ is the selected source"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(DVSRCW::VALUE6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CORDXRC`"]
pub enum CORDXRCW {
    #[doc = "No result chaining is selected"]
    VALUE1,
    #[doc = "QUOT register is the selected source"]
    VALUE2,
    #[doc = "RMD register is the selected source"]
    VALUE3,
}
impl CORDXRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CORDXRCW::VALUE1 => 0,
            CORDXRCW::VALUE2 => 1,
            CORDXRCW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CORDXRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CORDXRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CORDXRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No result chaining is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CORDXRCW::VALUE1)
    }
    #[doc = "QUOT register is the selected source"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CORDXRCW::VALUE2)
    }
    #[doc = "RMD register is the selected source"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CORDXRCW::VALUE3)
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
#[doc = "Values that can be written to the field `CORDYRC`"]
pub enum CORDYRCW {
    #[doc = "No result chaining is selected"]
    VALUE1,
    #[doc = "QUOT register is the selected source"]
    VALUE2,
    #[doc = "RMD register is the selected source"]
    VALUE3,
}
impl CORDYRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CORDYRCW::VALUE1 => 0,
            CORDYRCW::VALUE2 => 1,
            CORDYRCW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CORDYRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CORDYRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CORDYRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No result chaining is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CORDYRCW::VALUE1)
    }
    #[doc = "QUOT register is the selected source"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CORDYRCW::VALUE2)
    }
    #[doc = "RMD register is the selected source"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CORDYRCW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CORDZRC`"]
pub enum CORDZRCW {
    #[doc = "No result chaining is selected"]
    VALUE1,
    #[doc = "QUOT register is the selected source"]
    VALUE2,
    #[doc = "RMD register is the selected source"]
    VALUE3,
}
impl CORDZRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CORDZRCW::VALUE1 => 0,
            CORDZRCW::VALUE2 => 1,
            CORDZRCW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CORDZRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CORDZRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CORDZRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No result chaining is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CORDZRCW::VALUE1)
    }
    #[doc = "QUOT register is the selected source"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CORDZRCW::VALUE2)
    }
    #[doc = "RMD register is the selected source"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CORDZRCW::VALUE3)
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
#[doc = "Values that can be written to the field `SUSCFG`"]
pub enum SUSCFGW {
    #[doc = "Suspend mode is never entered."]
    VALUE1,
    #[doc = "Hard suspend mode will be entered when CPU is halted."]
    VALUE2,
    #[doc = "Soft suspend mode will be entered when CPU is halted."]
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
    #[doc = "Suspend mode is never entered."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUSCFGW::VALUE1)
    }
    #[doc = "Hard suspend mode will be entered when CPU is halted."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUSCFGW::VALUE2)
    }
    #[doc = "Soft suspend mode will be entered when CPU is halted."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SUSCFGW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:2 - Dividend Register Result Chaining"]
    #[inline]
    pub fn dvdrc(&self) -> DVDRCR {
        DVDRCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - Divisor Register Result Chaining"]
    #[inline]
    pub fn dvsrc(&self) -> DVSRCR {
        DVSRCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - CORDX Register Result Chaining"]
    #[inline]
    pub fn cordxrc(&self) -> CORDXRCR {
        CORDXRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 9:10 - CORDY Register Result Chaining"]
    #[inline]
    pub fn cordyrc(&self) -> CORDYRCR {
        CORDYRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - CORDZ Register Result Chaining"]
    #[inline]
    pub fn cordzrc(&self) -> CORDZRCR {
        CORDZRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Suspend Mode Configuration"]
    #[inline]
    pub fn suscfg(&self) -> SUSCFGR {
        SUSCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:2 - Dividend Register Result Chaining"]
    #[inline]
    pub fn dvdrc(&mut self) -> _DVDRCW {
        _DVDRCW { w: self }
    }
    #[doc = "Bits 3:5 - Divisor Register Result Chaining"]
    #[inline]
    pub fn dvsrc(&mut self) -> _DVSRCW {
        _DVSRCW { w: self }
    }
    #[doc = "Bits 6:7 - CORDX Register Result Chaining"]
    #[inline]
    pub fn cordxrc(&mut self) -> _CORDXRCW {
        _CORDXRCW { w: self }
    }
    #[doc = "Bits 9:10 - CORDY Register Result Chaining"]
    #[inline]
    pub fn cordyrc(&mut self) -> _CORDYRCW {
        _CORDYRCW { w: self }
    }
    #[doc = "Bits 12:13 - CORDZ Register Result Chaining"]
    #[inline]
    pub fn cordzrc(&mut self) -> _CORDZRCW {
        _CORDZRCW { w: self }
    }
    #[doc = "Bits 16:17 - Suspend Mode Configuration"]
    #[inline]
    pub fn suscfg(&mut self) -> _SUSCFGW {
        _SUSCFGW { w: self }
    }
}
