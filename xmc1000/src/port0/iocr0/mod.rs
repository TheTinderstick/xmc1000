#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCR0 {
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
#[doc = "Possible values of the field `PC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC0R {
    #[doc = "Input - No internal pull device active"]
    INPUT,
    #[doc = "Input - Internal pull-down device active"]
    INPUT_PULL_DOWN,
    #[doc = "Input - Internal pull-up device active"]
    INPUT_PULL_UP,
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    INPUT_STORE,
    #[doc = "Input inverted - No internal pull device active"]
    INPUT_INV,
    #[doc = "Input inverted - Internal pull-down device active"]
    INPUT_INV_PULL_DOWN,
    #[doc = "Input inverted - Internal pull-up device active"]
    INPUT_INV_PULL_UP,
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    INPUT_INV_STORE,
    #[doc = "Output Push-Pull - General-purpose output"]
    OUTPUT_PUSH_PULL,
    #[doc = "Output Push-Pull - Alternate output function 1"]
    OUTPUT_PUSH_PULL_ALT1,
    #[doc = "Output Push-Pull - Alternate output function 2"]
    OUTPUT_PUSH_PULL_ALT2,
    #[doc = "Output Push-Pull - Alternate output function 3"]
    OUTPUT_PUSH_PULL_ALT3,
    #[doc = "Output Push-Pull - Alternate output function 4"]
    OUTPUT_PUSH_PULL_ALT4,
    #[doc = "Output Push-Pull - Alternate output function 5"]
    OUTPUT_PUSH_PULL_ALT5,
    #[doc = "Output Push-Pull - Alternate output function 6"]
    OUTPUT_PUSH_PULL_ALT6,
    #[doc = "Output Push-Pull - Alternate output function 7"]
    OUTPUT_PUSH_PULL_ALT7,
    #[doc = "Output Open Drain - General-purpose output"]
    OUTPUT_OPEN_DRAIN,
    #[doc = "Output Open Drain - Alternate output function 1"]
    OUTPUT_OPEN_DRAIN_ALT1,
    #[doc = "Output Open Drain - Alternate output function 2"]
    OUTPUT_OPEN_DRAIN_ALT2,
    #[doc = "Output Open Drain - Alternate output function 3"]
    OUTPUT_OPEN_DRAIN_ALT3,
    #[doc = "Output Open Drain - Alternate output function 4"]
    OUTPUT_OPEN_DRAIN_ALT4,
    #[doc = "Output Open Drain - Alternate output function 5"]
    OUTPUT_OPEN_DRAIN_ALT5,
    #[doc = "Output Open Drain - Alternate output function 6"]
    OUTPUT_OPEN_DRAIN_ALT6,
    #[doc = "Output Open Drain - Alternate output function 7"]
    OUTPUT_OPEN_DRAIN_ALT7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PC0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC0R::INPUT => 0,
            PC0R::INPUT_PULL_DOWN => 1,
            PC0R::INPUT_PULL_UP => 2,
            PC0R::INPUT_STORE => 3,
            PC0R::INPUT_INV => 4,
            PC0R::INPUT_INV_PULL_DOWN => 5,
            PC0R::INPUT_INV_PULL_UP => 6,
            PC0R::INPUT_INV_STORE => 7,
            PC0R::OUTPUT_PUSH_PULL => 16,
            PC0R::OUTPUT_PUSH_PULL_ALT1 => 17,
            PC0R::OUTPUT_PUSH_PULL_ALT2 => 18,
            PC0R::OUTPUT_PUSH_PULL_ALT3 => 19,
            PC0R::OUTPUT_PUSH_PULL_ALT4 => 20,
            PC0R::OUTPUT_PUSH_PULL_ALT5 => 21,
            PC0R::OUTPUT_PUSH_PULL_ALT6 => 22,
            PC0R::OUTPUT_PUSH_PULL_ALT7 => 23,
            PC0R::OUTPUT_OPEN_DRAIN => 24,
            PC0R::OUTPUT_OPEN_DRAIN_ALT1 => 25,
            PC0R::OUTPUT_OPEN_DRAIN_ALT2 => 26,
            PC0R::OUTPUT_OPEN_DRAIN_ALT3 => 27,
            PC0R::OUTPUT_OPEN_DRAIN_ALT4 => 28,
            PC0R::OUTPUT_OPEN_DRAIN_ALT5 => 29,
            PC0R::OUTPUT_OPEN_DRAIN_ALT6 => 30,
            PC0R::OUTPUT_OPEN_DRAIN_ALT7 => 31,
            PC0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC0R {
        match value {
            0 => PC0R::INPUT,
            1 => PC0R::INPUT_PULL_DOWN,
            2 => PC0R::INPUT_PULL_UP,
            3 => PC0R::INPUT_STORE,
            4 => PC0R::INPUT_INV,
            5 => PC0R::INPUT_INV_PULL_DOWN,
            6 => PC0R::INPUT_INV_PULL_UP,
            7 => PC0R::INPUT_INV_STORE,
            16 => PC0R::OUTPUT_PUSH_PULL,
            17 => PC0R::OUTPUT_PUSH_PULL_ALT1,
            18 => PC0R::OUTPUT_PUSH_PULL_ALT2,
            19 => PC0R::OUTPUT_PUSH_PULL_ALT3,
            20 => PC0R::OUTPUT_PUSH_PULL_ALT4,
            21 => PC0R::OUTPUT_PUSH_PULL_ALT5,
            22 => PC0R::OUTPUT_PUSH_PULL_ALT6,
            23 => PC0R::OUTPUT_PUSH_PULL_ALT7,
            24 => PC0R::OUTPUT_OPEN_DRAIN,
            25 => PC0R::OUTPUT_OPEN_DRAIN_ALT1,
            26 => PC0R::OUTPUT_OPEN_DRAIN_ALT2,
            27 => PC0R::OUTPUT_OPEN_DRAIN_ALT3,
            28 => PC0R::OUTPUT_OPEN_DRAIN_ALT4,
            29 => PC0R::OUTPUT_OPEN_DRAIN_ALT5,
            30 => PC0R::OUTPUT_OPEN_DRAIN_ALT6,
            31 => PC0R::OUTPUT_OPEN_DRAIN_ALT7,
            i => PC0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PC0R::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUT_PULL_DOWN`"]
    #[inline]
    pub fn is_input_pull_down(&self) -> bool {
        *self == PC0R::INPUT_PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `INPUT_PULL_UP`"]
    #[inline]
    pub fn is_input_pull_up(&self) -> bool {
        *self == PC0R::INPUT_PULL_UP
    }
    #[doc = "Checks if the value of the field is `INPUT_STORE`"]
    #[inline]
    pub fn is_input_store(&self) -> bool {
        *self == PC0R::INPUT_STORE
    }
    #[doc = "Checks if the value of the field is `INPUT_INV`"]
    #[inline]
    pub fn is_input_inv(&self) -> bool {
        *self == PC0R::INPUT_INV
    }
    #[doc = "Checks if the value of the field is `INPUT_INV_PULL_DOWN`"]
    #[inline]
    pub fn is_input_inv_pull_down(&self) -> bool {
        *self == PC0R::INPUT_INV_PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `INPUT_INV_PULL_UP`"]
    #[inline]
    pub fn is_input_inv_pull_up(&self) -> bool {
        *self == PC0R::INPUT_INV_PULL_UP
    }
    #[doc = "Checks if the value of the field is `INPUT_INV_STORE`"]
    #[inline]
    pub fn is_input_inv_store(&self) -> bool {
        *self == PC0R::INPUT_INV_STORE
    }
    #[doc = "Checks if the value of the field is `OUTPUT_PUSH_PULL`"]
    #[inline]
    pub fn is_output_push_pull(&self) -> bool {
        *self == PC0R::OUTPUT_PUSH_PULL
    }
    #[doc = "Checks if the value of the field is `OUTPUT_PUSH_PULL_ALT1`"]
    #[inline]
    pub fn is_output_push_pull_alt1(&self) -> bool {
        *self == PC0R::OUTPUT_PUSH_PULL_ALT1
    }
    #[doc = "Checks if the value of the field is `OUTPUT_PUSH_PULL_ALT2`"]
    #[inline]
    pub fn is_output_push_pull_alt2(&self) -> bool {
        *self == PC0R::OUTPUT_PUSH_PULL_ALT2
    }
    #[doc = "Checks if the value of the field is `OUTPUT_PUSH_PULL_ALT3`"]
    #[inline]
    pub fn is_output_push_pull_alt3(&self) -> bool {
        *self == PC0R::OUTPUT_PUSH_PULL_ALT3
    }
    #[doc = "Checks if the value of the field is `OUTPUT_PUSH_PULL_ALT4`"]
    #[inline]
    pub fn is_output_push_pull_alt4(&self) -> bool {
        *self == PC0R::OUTPUT_PUSH_PULL_ALT4
    }
    #[doc = "Checks if the value of the field is `OUTPUT_PUSH_PULL_ALT5`"]
    #[inline]
    pub fn is_output_push_pull_alt5(&self) -> bool {
        *self == PC0R::OUTPUT_PUSH_PULL_ALT5
    }
    #[doc = "Checks if the value of the field is `OUTPUT_PUSH_PULL_ALT6`"]
    #[inline]
    pub fn is_output_push_pull_alt6(&self) -> bool {
        *self == PC0R::OUTPUT_PUSH_PULL_ALT6
    }
    #[doc = "Checks if the value of the field is `OUTPUT_PUSH_PULL_ALT7`"]
    #[inline]
    pub fn is_output_push_pull_alt7(&self) -> bool {
        *self == PC0R::OUTPUT_PUSH_PULL_ALT7
    }
    #[doc = "Checks if the value of the field is `OUTPUT_OPEN_DRAIN`"]
    #[inline]
    pub fn is_output_open_drain(&self) -> bool {
        *self == PC0R::OUTPUT_OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `OUTPUT_OPEN_DRAIN_ALT1`"]
    #[inline]
    pub fn is_output_open_drain_alt1(&self) -> bool {
        *self == PC0R::OUTPUT_OPEN_DRAIN_ALT1
    }
    #[doc = "Checks if the value of the field is `OUTPUT_OPEN_DRAIN_ALT2`"]
    #[inline]
    pub fn is_output_open_drain_alt2(&self) -> bool {
        *self == PC0R::OUTPUT_OPEN_DRAIN_ALT2
    }
    #[doc = "Checks if the value of the field is `OUTPUT_OPEN_DRAIN_ALT3`"]
    #[inline]
    pub fn is_output_open_drain_alt3(&self) -> bool {
        *self == PC0R::OUTPUT_OPEN_DRAIN_ALT3
    }
    #[doc = "Checks if the value of the field is `OUTPUT_OPEN_DRAIN_ALT4`"]
    #[inline]
    pub fn is_output_open_drain_alt4(&self) -> bool {
        *self == PC0R::OUTPUT_OPEN_DRAIN_ALT4
    }
    #[doc = "Checks if the value of the field is `OUTPUT_OPEN_DRAIN_ALT5`"]
    #[inline]
    pub fn is_output_open_drain_alt5(&self) -> bool {
        *self == PC0R::OUTPUT_OPEN_DRAIN_ALT5
    }
    #[doc = "Checks if the value of the field is `OUTPUT_OPEN_DRAIN_ALT6`"]
    #[inline]
    pub fn is_output_open_drain_alt6(&self) -> bool {
        *self == PC0R::OUTPUT_OPEN_DRAIN_ALT6
    }
    #[doc = "Checks if the value of the field is `OUTPUT_OPEN_DRAIN_ALT7`"]
    #[inline]
    pub fn is_output_open_drain_alt7(&self) -> bool {
        *self == PC0R::OUTPUT_OPEN_DRAIN_ALT7
    }
}
#[doc = "Possible values of the field `PC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC1R {
    #[doc = "Input - No internal pull device active"]
    VALUE1,
    #[doc = "Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "Output Push-Pull - Alternate output function 5"]
    VALUE14,
    #[doc = "Output Push-Pull - Alternate output function 6"]
    VALUE15,
    #[doc = "Output Push-Pull - Alternate output function 7"]
    VALUE16,
    #[doc = "Output Open Drain - General-purpose output"]
    VALUE17,
    #[doc = "Output Open Drain - Alternate output function 1"]
    VALUE18,
    #[doc = "Output Open Drain - Alternate output function 2"]
    VALUE19,
    #[doc = "Output Open Drain - Alternate output function 3"]
    VALUE20,
    #[doc = "Output Open Drain - Alternate output function 4"]
    VALUE21,
    #[doc = "Output Open Drain - Alternate output function 5"]
    VALUE22,
    #[doc = "Output Open Drain - Alternate output function 6"]
    VALUE23,
    #[doc = "Output Open Drain - Alternate output function 7"]
    VALUE24,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PC1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC1R::VALUE1 => 0,
            PC1R::VALUE2 => 1,
            PC1R::VALUE3 => 2,
            PC1R::VALUE4 => 3,
            PC1R::VALUE5 => 4,
            PC1R::VALUE6 => 5,
            PC1R::VALUE7 => 6,
            PC1R::VALUE8 => 7,
            PC1R::VALUE9 => 16,
            PC1R::VALUE10 => 17,
            PC1R::VALUE11 => 18,
            PC1R::VALUE12 => 19,
            PC1R::VALUE13 => 20,
            PC1R::VALUE14 => 21,
            PC1R::VALUE15 => 22,
            PC1R::VALUE16 => 23,
            PC1R::VALUE17 => 24,
            PC1R::VALUE18 => 25,
            PC1R::VALUE19 => 26,
            PC1R::VALUE20 => 27,
            PC1R::VALUE21 => 28,
            PC1R::VALUE22 => 29,
            PC1R::VALUE23 => 30,
            PC1R::VALUE24 => 31,
            PC1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC1R {
        match value {
            0 => PC1R::VALUE1,
            1 => PC1R::VALUE2,
            2 => PC1R::VALUE3,
            3 => PC1R::VALUE4,
            4 => PC1R::VALUE5,
            5 => PC1R::VALUE6,
            6 => PC1R::VALUE7,
            7 => PC1R::VALUE8,
            16 => PC1R::VALUE9,
            17 => PC1R::VALUE10,
            18 => PC1R::VALUE11,
            19 => PC1R::VALUE12,
            20 => PC1R::VALUE13,
            21 => PC1R::VALUE14,
            22 => PC1R::VALUE15,
            23 => PC1R::VALUE16,
            24 => PC1R::VALUE17,
            25 => PC1R::VALUE18,
            26 => PC1R::VALUE19,
            27 => PC1R::VALUE20,
            28 => PC1R::VALUE21,
            29 => PC1R::VALUE22,
            30 => PC1R::VALUE23,
            31 => PC1R::VALUE24,
            i => PC1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC1R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC1R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC1R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC1R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC1R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC1R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC1R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC1R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC1R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC1R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC1R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC1R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC1R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC1R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC1R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC1R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC1R::VALUE18
    }
    #[doc = "Checks if the value of the field is `VALUE19`"]
    #[inline]
    pub fn is_value19(&self) -> bool {
        *self == PC1R::VALUE19
    }
    #[doc = "Checks if the value of the field is `VALUE20`"]
    #[inline]
    pub fn is_value20(&self) -> bool {
        *self == PC1R::VALUE20
    }
    #[doc = "Checks if the value of the field is `VALUE21`"]
    #[inline]
    pub fn is_value21(&self) -> bool {
        *self == PC1R::VALUE21
    }
    #[doc = "Checks if the value of the field is `VALUE22`"]
    #[inline]
    pub fn is_value22(&self) -> bool {
        *self == PC1R::VALUE22
    }
    #[doc = "Checks if the value of the field is `VALUE23`"]
    #[inline]
    pub fn is_value23(&self) -> bool {
        *self == PC1R::VALUE23
    }
    #[doc = "Checks if the value of the field is `VALUE24`"]
    #[inline]
    pub fn is_value24(&self) -> bool {
        *self == PC1R::VALUE24
    }
}
#[doc = "Possible values of the field `PC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC2R {
    #[doc = "Input - No internal pull device active"]
    VALUE1,
    #[doc = "Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "Output Push-Pull - Alternate output function 5"]
    VALUE14,
    #[doc = "Output Push-Pull - Alternate output function 6"]
    VALUE15,
    #[doc = "Output Push-Pull - Alternate output function 7"]
    VALUE16,
    #[doc = "Output Open Drain - General-purpose output"]
    VALUE17,
    #[doc = "Output Open Drain - Alternate output function 1"]
    VALUE18,
    #[doc = "Output Open Drain - Alternate output function 2"]
    VALUE19,
    #[doc = "Output Open Drain - Alternate output function 3"]
    VALUE20,
    #[doc = "Output Open Drain - Alternate output function 4"]
    VALUE21,
    #[doc = "Output Open Drain - Alternate output function 5"]
    VALUE22,
    #[doc = "Output Open Drain - Alternate output function 6"]
    VALUE23,
    #[doc = "Output Open Drain - Alternate output function 7"]
    VALUE24,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PC2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC2R::VALUE1 => 0,
            PC2R::VALUE2 => 1,
            PC2R::VALUE3 => 2,
            PC2R::VALUE4 => 3,
            PC2R::VALUE5 => 4,
            PC2R::VALUE6 => 5,
            PC2R::VALUE7 => 6,
            PC2R::VALUE8 => 7,
            PC2R::VALUE9 => 16,
            PC2R::VALUE10 => 17,
            PC2R::VALUE11 => 18,
            PC2R::VALUE12 => 19,
            PC2R::VALUE13 => 20,
            PC2R::VALUE14 => 21,
            PC2R::VALUE15 => 22,
            PC2R::VALUE16 => 23,
            PC2R::VALUE17 => 24,
            PC2R::VALUE18 => 25,
            PC2R::VALUE19 => 26,
            PC2R::VALUE20 => 27,
            PC2R::VALUE21 => 28,
            PC2R::VALUE22 => 29,
            PC2R::VALUE23 => 30,
            PC2R::VALUE24 => 31,
            PC2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC2R {
        match value {
            0 => PC2R::VALUE1,
            1 => PC2R::VALUE2,
            2 => PC2R::VALUE3,
            3 => PC2R::VALUE4,
            4 => PC2R::VALUE5,
            5 => PC2R::VALUE6,
            6 => PC2R::VALUE7,
            7 => PC2R::VALUE8,
            16 => PC2R::VALUE9,
            17 => PC2R::VALUE10,
            18 => PC2R::VALUE11,
            19 => PC2R::VALUE12,
            20 => PC2R::VALUE13,
            21 => PC2R::VALUE14,
            22 => PC2R::VALUE15,
            23 => PC2R::VALUE16,
            24 => PC2R::VALUE17,
            25 => PC2R::VALUE18,
            26 => PC2R::VALUE19,
            27 => PC2R::VALUE20,
            28 => PC2R::VALUE21,
            29 => PC2R::VALUE22,
            30 => PC2R::VALUE23,
            31 => PC2R::VALUE24,
            i => PC2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC2R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC2R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC2R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC2R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC2R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC2R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC2R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC2R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC2R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC2R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC2R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC2R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC2R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC2R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC2R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC2R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC2R::VALUE18
    }
    #[doc = "Checks if the value of the field is `VALUE19`"]
    #[inline]
    pub fn is_value19(&self) -> bool {
        *self == PC2R::VALUE19
    }
    #[doc = "Checks if the value of the field is `VALUE20`"]
    #[inline]
    pub fn is_value20(&self) -> bool {
        *self == PC2R::VALUE20
    }
    #[doc = "Checks if the value of the field is `VALUE21`"]
    #[inline]
    pub fn is_value21(&self) -> bool {
        *self == PC2R::VALUE21
    }
    #[doc = "Checks if the value of the field is `VALUE22`"]
    #[inline]
    pub fn is_value22(&self) -> bool {
        *self == PC2R::VALUE22
    }
    #[doc = "Checks if the value of the field is `VALUE23`"]
    #[inline]
    pub fn is_value23(&self) -> bool {
        *self == PC2R::VALUE23
    }
    #[doc = "Checks if the value of the field is `VALUE24`"]
    #[inline]
    pub fn is_value24(&self) -> bool {
        *self == PC2R::VALUE24
    }
}
#[doc = "Possible values of the field `PC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC3R {
    #[doc = "Input - No internal pull device active"]
    VALUE1,
    #[doc = "Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "Output Push-Pull - Alternate output function 5"]
    VALUE14,
    #[doc = "Output Push-Pull - Alternate output function 6"]
    VALUE15,
    #[doc = "Output Push-Pull - Alternate output function 7"]
    VALUE16,
    #[doc = "Output Open Drain - General-purpose output"]
    VALUE17,
    #[doc = "Output Open Drain - Alternate output function 1"]
    VALUE18,
    #[doc = "Output Open Drain - Alternate output function 2"]
    VALUE19,
    #[doc = "Output Open Drain - Alternate output function 3"]
    VALUE20,
    #[doc = "Output Open Drain - Alternate output function 4"]
    VALUE21,
    #[doc = "Output Open Drain - Alternate output function 5"]
    VALUE22,
    #[doc = "Output Open Drain - Alternate output function 6"]
    VALUE23,
    #[doc = "Output Open Drain - Alternate output function 7"]
    VALUE24,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PC3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC3R::VALUE1 => 0,
            PC3R::VALUE2 => 1,
            PC3R::VALUE3 => 2,
            PC3R::VALUE4 => 3,
            PC3R::VALUE5 => 4,
            PC3R::VALUE6 => 5,
            PC3R::VALUE7 => 6,
            PC3R::VALUE8 => 7,
            PC3R::VALUE9 => 16,
            PC3R::VALUE10 => 17,
            PC3R::VALUE11 => 18,
            PC3R::VALUE12 => 19,
            PC3R::VALUE13 => 20,
            PC3R::VALUE14 => 21,
            PC3R::VALUE15 => 22,
            PC3R::VALUE16 => 23,
            PC3R::VALUE17 => 24,
            PC3R::VALUE18 => 25,
            PC3R::VALUE19 => 26,
            PC3R::VALUE20 => 27,
            PC3R::VALUE21 => 28,
            PC3R::VALUE22 => 29,
            PC3R::VALUE23 => 30,
            PC3R::VALUE24 => 31,
            PC3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC3R {
        match value {
            0 => PC3R::VALUE1,
            1 => PC3R::VALUE2,
            2 => PC3R::VALUE3,
            3 => PC3R::VALUE4,
            4 => PC3R::VALUE5,
            5 => PC3R::VALUE6,
            6 => PC3R::VALUE7,
            7 => PC3R::VALUE8,
            16 => PC3R::VALUE9,
            17 => PC3R::VALUE10,
            18 => PC3R::VALUE11,
            19 => PC3R::VALUE12,
            20 => PC3R::VALUE13,
            21 => PC3R::VALUE14,
            22 => PC3R::VALUE15,
            23 => PC3R::VALUE16,
            24 => PC3R::VALUE17,
            25 => PC3R::VALUE18,
            26 => PC3R::VALUE19,
            27 => PC3R::VALUE20,
            28 => PC3R::VALUE21,
            29 => PC3R::VALUE22,
            30 => PC3R::VALUE23,
            31 => PC3R::VALUE24,
            i => PC3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC3R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC3R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC3R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC3R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC3R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC3R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC3R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC3R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC3R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC3R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC3R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC3R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC3R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC3R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC3R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC3R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC3R::VALUE18
    }
    #[doc = "Checks if the value of the field is `VALUE19`"]
    #[inline]
    pub fn is_value19(&self) -> bool {
        *self == PC3R::VALUE19
    }
    #[doc = "Checks if the value of the field is `VALUE20`"]
    #[inline]
    pub fn is_value20(&self) -> bool {
        *self == PC3R::VALUE20
    }
    #[doc = "Checks if the value of the field is `VALUE21`"]
    #[inline]
    pub fn is_value21(&self) -> bool {
        *self == PC3R::VALUE21
    }
    #[doc = "Checks if the value of the field is `VALUE22`"]
    #[inline]
    pub fn is_value22(&self) -> bool {
        *self == PC3R::VALUE22
    }
    #[doc = "Checks if the value of the field is `VALUE23`"]
    #[inline]
    pub fn is_value23(&self) -> bool {
        *self == PC3R::VALUE23
    }
    #[doc = "Checks if the value of the field is `VALUE24`"]
    #[inline]
    pub fn is_value24(&self) -> bool {
        *self == PC3R::VALUE24
    }
}
#[doc = "Values that can be written to the field `PC0`"]
pub enum PC0W {
    #[doc = "Input - No internal pull device active"]
    INPUT,
    #[doc = "Input - Internal pull-down device active"]
    INPUT_PULL_DOWN,
    #[doc = "Input - Internal pull-up device active"]
    INPUT_PULL_UP,
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    INPUT_STORE,
    #[doc = "Input inverted - No internal pull device active"]
    INPUT_INV,
    #[doc = "Input inverted - Internal pull-down device active"]
    INPUT_INV_PULL_DOWN,
    #[doc = "Input inverted - Internal pull-up device active"]
    INPUT_INV_PULL_UP,
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    INPUT_INV_STORE,
    #[doc = "Output Push-Pull - General-purpose output"]
    OUTPUT_PUSH_PULL,
    #[doc = "Output Push-Pull - Alternate output function 1"]
    OUTPUT_PUSH_PULL_ALT1,
    #[doc = "Output Push-Pull - Alternate output function 2"]
    OUTPUT_PUSH_PULL_ALT2,
    #[doc = "Output Push-Pull - Alternate output function 3"]
    OUTPUT_PUSH_PULL_ALT3,
    #[doc = "Output Push-Pull - Alternate output function 4"]
    OUTPUT_PUSH_PULL_ALT4,
    #[doc = "Output Push-Pull - Alternate output function 5"]
    OUTPUT_PUSH_PULL_ALT5,
    #[doc = "Output Push-Pull - Alternate output function 6"]
    OUTPUT_PUSH_PULL_ALT6,
    #[doc = "Output Push-Pull - Alternate output function 7"]
    OUTPUT_PUSH_PULL_ALT7,
    #[doc = "Output Open Drain - General-purpose output"]
    OUTPUT_OPEN_DRAIN,
    #[doc = "Output Open Drain - Alternate output function 1"]
    OUTPUT_OPEN_DRAIN_ALT1,
    #[doc = "Output Open Drain - Alternate output function 2"]
    OUTPUT_OPEN_DRAIN_ALT2,
    #[doc = "Output Open Drain - Alternate output function 3"]
    OUTPUT_OPEN_DRAIN_ALT3,
    #[doc = "Output Open Drain - Alternate output function 4"]
    OUTPUT_OPEN_DRAIN_ALT4,
    #[doc = "Output Open Drain - Alternate output function 5"]
    OUTPUT_OPEN_DRAIN_ALT5,
    #[doc = "Output Open Drain - Alternate output function 6"]
    OUTPUT_OPEN_DRAIN_ALT6,
    #[doc = "Output Open Drain - Alternate output function 7"]
    OUTPUT_OPEN_DRAIN_ALT7,
}
impl PC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC0W::INPUT => 0,
            PC0W::INPUT_PULL_DOWN => 1,
            PC0W::INPUT_PULL_UP => 2,
            PC0W::INPUT_STORE => 3,
            PC0W::INPUT_INV => 4,
            PC0W::INPUT_INV_PULL_DOWN => 5,
            PC0W::INPUT_INV_PULL_UP => 6,
            PC0W::INPUT_INV_STORE => 7,
            PC0W::OUTPUT_PUSH_PULL => 16,
            PC0W::OUTPUT_PUSH_PULL_ALT1 => 17,
            PC0W::OUTPUT_PUSH_PULL_ALT2 => 18,
            PC0W::OUTPUT_PUSH_PULL_ALT3 => 19,
            PC0W::OUTPUT_PUSH_PULL_ALT4 => 20,
            PC0W::OUTPUT_PUSH_PULL_ALT5 => 21,
            PC0W::OUTPUT_PUSH_PULL_ALT6 => 22,
            PC0W::OUTPUT_PUSH_PULL_ALT7 => 23,
            PC0W::OUTPUT_OPEN_DRAIN => 24,
            PC0W::OUTPUT_OPEN_DRAIN_ALT1 => 25,
            PC0W::OUTPUT_OPEN_DRAIN_ALT2 => 26,
            PC0W::OUTPUT_OPEN_DRAIN_ALT3 => 27,
            PC0W::OUTPUT_OPEN_DRAIN_ALT4 => 28,
            PC0W::OUTPUT_OPEN_DRAIN_ALT5 => 29,
            PC0W::OUTPUT_OPEN_DRAIN_ALT6 => 30,
            PC0W::OUTPUT_OPEN_DRAIN_ALT7 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC0W<'a> {
    w: &'a mut W,
}
impl<'a> _PC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(PC0W::INPUT)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn input_pull_down(self) -> &'a mut W {
        self.variant(PC0W::INPUT_PULL_DOWN)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn input_pull_up(self) -> &'a mut W {
        self.variant(PC0W::INPUT_PULL_UP)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn input_store(self) -> &'a mut W {
        self.variant(PC0W::INPUT_STORE)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn input_inv(self) -> &'a mut W {
        self.variant(PC0W::INPUT_INV)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn input_inv_pull_down(self) -> &'a mut W {
        self.variant(PC0W::INPUT_INV_PULL_DOWN)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn input_inv_pull_up(self) -> &'a mut W {
        self.variant(PC0W::INPUT_INV_PULL_UP)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn input_inv_store(self) -> &'a mut W {
        self.variant(PC0W::INPUT_INV_STORE)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn output_push_pull(self) -> &'a mut W {
        self.variant(PC0W::OUTPUT_PUSH_PULL)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn output_push_pull_alt1(self) -> &'a mut W {
        self.variant(PC0W::OUTPUT_PUSH_PULL_ALT1)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn output_push_pull_alt2(self) -> &'a mut W {
        self.variant(PC0W::OUTPUT_PUSH_PULL_ALT2)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn output_push_pull_alt3(self) -> &'a mut W {
        self.variant(PC0W::OUTPUT_PUSH_PULL_ALT3)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn output_push_pull_alt4(self) -> &'a mut W {
        self.variant(PC0W::OUTPUT_PUSH_PULL_ALT4)
    }
    #[doc = "Output Push-Pull - Alternate output function 5"]
    #[inline]
    pub fn output_push_pull_alt5(self) -> &'a mut W {
        self.variant(PC0W::OUTPUT_PUSH_PULL_ALT5)
    }
    #[doc = "Output Push-Pull - Alternate output function 6"]
    #[inline]
    pub fn output_push_pull_alt6(self) -> &'a mut W {
        self.variant(PC0W::OUTPUT_PUSH_PULL_ALT6)
    }
    #[doc = "Output Push-Pull - Alternate output function 7"]
    #[inline]
    pub fn output_push_pull_alt7(self) -> &'a mut W {
        self.variant(PC0W::OUTPUT_PUSH_PULL_ALT7)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn output_open_drain(self) -> &'a mut W {
        self.variant(PC0W::OUTPUT_OPEN_DRAIN)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn output_open_drain_alt1(self) -> &'a mut W {
        self.variant(PC0W::OUTPUT_OPEN_DRAIN_ALT1)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn output_open_drain_alt2(self) -> &'a mut W {
        self.variant(PC0W::OUTPUT_OPEN_DRAIN_ALT2)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn output_open_drain_alt3(self) -> &'a mut W {
        self.variant(PC0W::OUTPUT_OPEN_DRAIN_ALT3)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn output_open_drain_alt4(self) -> &'a mut W {
        self.variant(PC0W::OUTPUT_OPEN_DRAIN_ALT4)
    }
    #[doc = "Output Open Drain - Alternate output function 5"]
    #[inline]
    pub fn output_open_drain_alt5(self) -> &'a mut W {
        self.variant(PC0W::OUTPUT_OPEN_DRAIN_ALT5)
    }
    #[doc = "Output Open Drain - Alternate output function 6"]
    #[inline]
    pub fn output_open_drain_alt6(self) -> &'a mut W {
        self.variant(PC0W::OUTPUT_OPEN_DRAIN_ALT6)
    }
    #[doc = "Output Open Drain - Alternate output function 7"]
    #[inline]
    pub fn output_open_drain_alt7(self) -> &'a mut W {
        self.variant(PC0W::OUTPUT_OPEN_DRAIN_ALT7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PC1`"]
pub enum PC1W {
    #[doc = "Input - No internal pull device active"]
    VALUE1,
    #[doc = "Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "Output Push-Pull - Alternate output function 5"]
    VALUE14,
    #[doc = "Output Push-Pull - Alternate output function 6"]
    VALUE15,
    #[doc = "Output Push-Pull - Alternate output function 7"]
    VALUE16,
    #[doc = "Output Open Drain - General-purpose output"]
    VALUE17,
    #[doc = "Output Open Drain - Alternate output function 1"]
    VALUE18,
    #[doc = "Output Open Drain - Alternate output function 2"]
    VALUE19,
    #[doc = "Output Open Drain - Alternate output function 3"]
    VALUE20,
    #[doc = "Output Open Drain - Alternate output function 4"]
    VALUE21,
    #[doc = "Output Open Drain - Alternate output function 5"]
    VALUE22,
    #[doc = "Output Open Drain - Alternate output function 6"]
    VALUE23,
    #[doc = "Output Open Drain - Alternate output function 7"]
    VALUE24,
}
impl PC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC1W::VALUE1 => 0,
            PC1W::VALUE2 => 1,
            PC1W::VALUE3 => 2,
            PC1W::VALUE4 => 3,
            PC1W::VALUE5 => 4,
            PC1W::VALUE6 => 5,
            PC1W::VALUE7 => 6,
            PC1W::VALUE8 => 7,
            PC1W::VALUE9 => 16,
            PC1W::VALUE10 => 17,
            PC1W::VALUE11 => 18,
            PC1W::VALUE12 => 19,
            PC1W::VALUE13 => 20,
            PC1W::VALUE14 => 21,
            PC1W::VALUE15 => 22,
            PC1W::VALUE16 => 23,
            PC1W::VALUE17 => 24,
            PC1W::VALUE18 => 25,
            PC1W::VALUE19 => 26,
            PC1W::VALUE20 => 27,
            PC1W::VALUE21 => 28,
            PC1W::VALUE22 => 29,
            PC1W::VALUE23 => 30,
            PC1W::VALUE24 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC1W<'a> {
    w: &'a mut W,
}
impl<'a> _PC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC1W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC1W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC1W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC1W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC1W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC1W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC1W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC1W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC1W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC1W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC1W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC1W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC1W::VALUE13)
    }
    #[doc = "Output Push-Pull - Alternate output function 5"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC1W::VALUE14)
    }
    #[doc = "Output Push-Pull - Alternate output function 6"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC1W::VALUE15)
    }
    #[doc = "Output Push-Pull - Alternate output function 7"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC1W::VALUE16)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC1W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC1W::VALUE18)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value19(self) -> &'a mut W {
        self.variant(PC1W::VALUE19)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value20(self) -> &'a mut W {
        self.variant(PC1W::VALUE20)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value21(self) -> &'a mut W {
        self.variant(PC1W::VALUE21)
    }
    #[doc = "Output Open Drain - Alternate output function 5"]
    #[inline]
    pub fn value22(self) -> &'a mut W {
        self.variant(PC1W::VALUE22)
    }
    #[doc = "Output Open Drain - Alternate output function 6"]
    #[inline]
    pub fn value23(self) -> &'a mut W {
        self.variant(PC1W::VALUE23)
    }
    #[doc = "Output Open Drain - Alternate output function 7"]
    #[inline]
    pub fn value24(self) -> &'a mut W {
        self.variant(PC1W::VALUE24)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PC2`"]
pub enum PC2W {
    #[doc = "Input - No internal pull device active"]
    VALUE1,
    #[doc = "Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "Output Push-Pull - Alternate output function 5"]
    VALUE14,
    #[doc = "Output Push-Pull - Alternate output function 6"]
    VALUE15,
    #[doc = "Output Push-Pull - Alternate output function 7"]
    VALUE16,
    #[doc = "Output Open Drain - General-purpose output"]
    VALUE17,
    #[doc = "Output Open Drain - Alternate output function 1"]
    VALUE18,
    #[doc = "Output Open Drain - Alternate output function 2"]
    VALUE19,
    #[doc = "Output Open Drain - Alternate output function 3"]
    VALUE20,
    #[doc = "Output Open Drain - Alternate output function 4"]
    VALUE21,
    #[doc = "Output Open Drain - Alternate output function 5"]
    VALUE22,
    #[doc = "Output Open Drain - Alternate output function 6"]
    VALUE23,
    #[doc = "Output Open Drain - Alternate output function 7"]
    VALUE24,
}
impl PC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC2W::VALUE1 => 0,
            PC2W::VALUE2 => 1,
            PC2W::VALUE3 => 2,
            PC2W::VALUE4 => 3,
            PC2W::VALUE5 => 4,
            PC2W::VALUE6 => 5,
            PC2W::VALUE7 => 6,
            PC2W::VALUE8 => 7,
            PC2W::VALUE9 => 16,
            PC2W::VALUE10 => 17,
            PC2W::VALUE11 => 18,
            PC2W::VALUE12 => 19,
            PC2W::VALUE13 => 20,
            PC2W::VALUE14 => 21,
            PC2W::VALUE15 => 22,
            PC2W::VALUE16 => 23,
            PC2W::VALUE17 => 24,
            PC2W::VALUE18 => 25,
            PC2W::VALUE19 => 26,
            PC2W::VALUE20 => 27,
            PC2W::VALUE21 => 28,
            PC2W::VALUE22 => 29,
            PC2W::VALUE23 => 30,
            PC2W::VALUE24 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC2W<'a> {
    w: &'a mut W,
}
impl<'a> _PC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC2W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC2W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC2W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC2W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC2W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC2W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC2W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC2W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC2W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC2W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC2W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC2W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC2W::VALUE13)
    }
    #[doc = "Output Push-Pull - Alternate output function 5"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC2W::VALUE14)
    }
    #[doc = "Output Push-Pull - Alternate output function 6"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC2W::VALUE15)
    }
    #[doc = "Output Push-Pull - Alternate output function 7"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC2W::VALUE16)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC2W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC2W::VALUE18)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value19(self) -> &'a mut W {
        self.variant(PC2W::VALUE19)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value20(self) -> &'a mut W {
        self.variant(PC2W::VALUE20)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value21(self) -> &'a mut W {
        self.variant(PC2W::VALUE21)
    }
    #[doc = "Output Open Drain - Alternate output function 5"]
    #[inline]
    pub fn value22(self) -> &'a mut W {
        self.variant(PC2W::VALUE22)
    }
    #[doc = "Output Open Drain - Alternate output function 6"]
    #[inline]
    pub fn value23(self) -> &'a mut W {
        self.variant(PC2W::VALUE23)
    }
    #[doc = "Output Open Drain - Alternate output function 7"]
    #[inline]
    pub fn value24(self) -> &'a mut W {
        self.variant(PC2W::VALUE24)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PC3`"]
pub enum PC3W {
    #[doc = "Input - No internal pull device active"]
    VALUE1,
    #[doc = "Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "Output Push-Pull - Alternate output function 5"]
    VALUE14,
    #[doc = "Output Push-Pull - Alternate output function 6"]
    VALUE15,
    #[doc = "Output Push-Pull - Alternate output function 7"]
    VALUE16,
    #[doc = "Output Open Drain - General-purpose output"]
    VALUE17,
    #[doc = "Output Open Drain - Alternate output function 1"]
    VALUE18,
    #[doc = "Output Open Drain - Alternate output function 2"]
    VALUE19,
    #[doc = "Output Open Drain - Alternate output function 3"]
    VALUE20,
    #[doc = "Output Open Drain - Alternate output function 4"]
    VALUE21,
    #[doc = "Output Open Drain - Alternate output function 5"]
    VALUE22,
    #[doc = "Output Open Drain - Alternate output function 6"]
    VALUE23,
    #[doc = "Output Open Drain - Alternate output function 7"]
    VALUE24,
}
impl PC3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC3W::VALUE1 => 0,
            PC3W::VALUE2 => 1,
            PC3W::VALUE3 => 2,
            PC3W::VALUE4 => 3,
            PC3W::VALUE5 => 4,
            PC3W::VALUE6 => 5,
            PC3W::VALUE7 => 6,
            PC3W::VALUE8 => 7,
            PC3W::VALUE9 => 16,
            PC3W::VALUE10 => 17,
            PC3W::VALUE11 => 18,
            PC3W::VALUE12 => 19,
            PC3W::VALUE13 => 20,
            PC3W::VALUE14 => 21,
            PC3W::VALUE15 => 22,
            PC3W::VALUE16 => 23,
            PC3W::VALUE17 => 24,
            PC3W::VALUE18 => 25,
            PC3W::VALUE19 => 26,
            PC3W::VALUE20 => 27,
            PC3W::VALUE21 => 28,
            PC3W::VALUE22 => 29,
            PC3W::VALUE23 => 30,
            PC3W::VALUE24 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC3W<'a> {
    w: &'a mut W,
}
impl<'a> _PC3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC3W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC3W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC3W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC3W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC3W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC3W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC3W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC3W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC3W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC3W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC3W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC3W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC3W::VALUE13)
    }
    #[doc = "Output Push-Pull - Alternate output function 5"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC3W::VALUE14)
    }
    #[doc = "Output Push-Pull - Alternate output function 6"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC3W::VALUE15)
    }
    #[doc = "Output Push-Pull - Alternate output function 7"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC3W::VALUE16)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC3W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC3W::VALUE18)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value19(self) -> &'a mut W {
        self.variant(PC3W::VALUE19)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value20(self) -> &'a mut W {
        self.variant(PC3W::VALUE20)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value21(self) -> &'a mut W {
        self.variant(PC3W::VALUE21)
    }
    #[doc = "Output Open Drain - Alternate output function 5"]
    #[inline]
    pub fn value22(self) -> &'a mut W {
        self.variant(PC3W::VALUE22)
    }
    #[doc = "Output Open Drain - Alternate output function 6"]
    #[inline]
    pub fn value23(self) -> &'a mut W {
        self.variant(PC3W::VALUE23)
    }
    #[doc = "Output Open Drain - Alternate output function 7"]
    #[inline]
    pub fn value24(self) -> &'a mut W {
        self.variant(PC3W::VALUE24)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 27;
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
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline]
    pub fn pc0(&self) -> PC0R {
        PC0R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline]
    pub fn pc1(&self) -> PC1R {
        PC1R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline]
    pub fn pc2(&self) -> PC2R {
        PC2R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline]
    pub fn pc3(&self) -> PC3R {
        PC3R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 27;
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
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline]
    pub fn pc0(&mut self) -> _PC0W {
        _PC0W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline]
    pub fn pc1(&mut self) -> _PC1W {
        _PC1W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline]
    pub fn pc2(&mut self) -> _PC2W {
        _PC2W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline]
    pub fn pc3(&mut self) -> _PC3W {
        _PC3W { w: self }
    }
}
