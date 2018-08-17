//! General Purpose Input / Output

use core::marker::PhantomData;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;
/// Pulled down input (type state)
pub struct PullDown;
/// Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;
/// Push pull output alternate function 1 (type state)
pub struct PushPullAlt1;
/// Push pull output alternate function 2 (type state)
pub struct PushPullAlt2;
/// Push pull output alternate function 3 (type state)
pub struct PushPullAlt3;
/// Push pull output alternate function 4 (type state)
pub struct PushPullAlt4;
/// Push pull output alternate function 5 (type state)
pub struct PushPullAlt5;
/// Push pull output alternate function 6 (type state)
pub struct PushPullAlt6;
/// Push pull output alternate function 7 (type state)
pub struct PushPullAlt7;
/// Open drain output (type state)
pub struct OpenDrain;
/// Open drain output alternate function 1 (type state)
pub struct OpenDrainAlt1;
/// Open drain output alternate function 2 (type state)
pub struct OpenDrainAlt2;
/// Open drain output alternate function 3 (type state)
pub struct OpenDrainAlt3;
/// Open drain output alternate function 4 (type state)
pub struct OpenDrainAlt4;
/// Open drain output alternate function 5 (type state)
pub struct OpenDrainAlt5;
/// Open drain output alternate function 6 (type state)
pub struct OpenDrainAlt6;
/// Open drain output alternate function 7 (type state)
pub struct OpenDrainAlt7;

/// Output mode (type state)
pub struct Direct<MODE> {
    _mode: PhantomData<MODE>,
}

/// Hardware driven 0 (type state)
pub struct HW0;
/// Hardware driven 1 (type state)
pub struct HW1;

macro_rules! gpio {
    ($PORTX:ident, $portx:ident, $PXx:ident, [
        $($IOCRY:ident: $iocry:ident, [
            $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty),)+
        ],)+
    ]) => {
        pub mod $portx {
            use core::marker::PhantomData;

            use hal::digital::OutputPin;
            use xmc1000::{$portx, $PORTX};
            use xmc1000::port0::iocr0::PC0R as PC;

            use super::{
                GpioExt,
                Input, Floating, PullUp, PullDown,
                Output, OpenDrain, PushPull,
                //Direct, HW0,
            };

            /// GPIO parts
            pub struct Parts {
                $(
                    pub $iocry: $IOCRY,
                    $(
                        /// Pin
                        pub $pxi: $PXi<$MODE>,
                    )+
                )+
            }

            impl GpioExt for $PORTX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    Parts {
                        $(
                            $iocry: $IOCRY { _0: () },
                            $(
                                $pxi: $PXi { _mode: PhantomData },
                            )+
                        )+
                    }
                }
            }

            pub struct $PXx<MODE> {
                i: u8,
                _mode: PhantomData<MODE>,
            }

            impl<MODE> OutputPin for $PXx<Output<MODE>> {
                fn set_high(&mut self) {
                    unsafe {
                        (*$PORTX::ptr()).omr.write(|w| w.bits(1 << self.i))
                    }
                }

                fn set_low(&mut self) {
                    unsafe {
                        (*$PORTX::ptr()).omr.write(|w| w.bits(1 << (16 + self.i)))
                    }
                }
            }

            impl<MODE> $PXx<Output<MODE>> {
                pub fn toggle(&mut self) {
                    unsafe {
                        (*$PORTX::ptr()).omr.write(|w| w.bits(0x10001 << self.i))
                    }
                }

                pub fn is_set_low(&mut self) -> bool {
                    unsafe {
                        (*$PORTX::ptr()).out.read().bits() & (1 << self.i) == 0
                    }
                }

                pub fn is_set_high(&mut self) -> bool {
                    !self.is_set_low()
                }
            }

            $(
                pub struct $IOCRY {
                    _0: (),
                }

                impl $IOCRY {
                    pub(crate) fn iocr(&mut self) -> &$portx::$IOCRY {
                        unsafe { &(*$PORTX::ptr()).$iocry }
                    }
                }

                $(
                    pub struct $PXi<MODE> {
                        _mode: PhantomData<MODE>,
                    }

                    impl<MODE> $PXi<MODE> {
                        pub fn into_open_drain_output(self, set_high: bool, iocr: &mut $IOCRY) -> $PXi<Output<OpenDrain>> {
                            let offset = ($i%4) * 8 + 3;
                            let mode = PC::OUTPUT_OPEN_DRAIN.bits() as u32;

                            let mut output = $PXi { _mode: PhantomData };
                            if set_high { output.set_high() } else { output.set_low() }

                            iocr.iocr().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b11111 << offset)) | (mode << offset))
                            });

                            output
                        }

                        pub fn into_push_pull_output(self, set_high: bool, iocr: &mut $IOCRY) -> $PXi<Output<PushPull>> {
                            let offset = ($i%4) * 8 + 3;
                            let mode = PC::OUTPUT_PUSH_PULL.bits() as u32;

                            let mut output = $PXi { _mode: PhantomData };
                            if set_high { output.set_high() } else { output.set_low() }

                            iocr.iocr().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b11111 << offset)) | (mode << offset))
                            });

                            output
                        }

                        pub fn into_floating_input(self, iocr: &mut $IOCRY) -> $PXi<Input<Floating>> {
                            let offset = ($i%4) * 8 + 3;
                            let mode = PC::INPUT.bits() as u32;
                            iocr.iocr().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b11111 << offset)) | (mode << offset))
                            });

                            $PXi { _mode: PhantomData }
                        }

                        pub fn into_pull_down_input(self, iocr: &mut $IOCRY) -> $PXi<Input<PullDown>> {
                            let offset = ($i%4) * 8 + 3;
                            let mode = PC::INPUT_PULL_DOWN.bits() as u32;
                            iocr.iocr().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b11111 << offset)) | (mode << offset))
                            });

                            $PXi { _mode: PhantomData }
                        }

                        pub fn into_pull_up_input(self, iocr: &mut $IOCRY) -> $PXi<Input<PullUp>> {
                            let offset = ($i%4) * 8 + 3;
                            let mode = PC::INPUT_PULL_UP.bits() as u32;
                            iocr.iocr().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b11111 << offset)) | (mode << offset))
                            });

                            $PXi { _mode: PhantomData }
                        }

                        /*
                        pub fn into_hw0_direct(self, hwsel: &mut $portx::HWSEL) -> $PXi<Direct<HW0>> {
                        hwsel.modify(|_, w| w.hw0().value2());

                        $PXi { _mode: PhantomData }
                    }
                         */
                    }

                    impl<MODE> $PXi<Output<MODE>> {
                        pub fn downgrade(self) -> $PXx<Output<MODE>> {
                            $PXx {
                                i: $i,
                                _mode: self._mode,
                            }
                        }

                        pub fn toggle(&mut self) {
                            unsafe {
                                (*$PORTX::ptr()).omr.write(|w| w.bits(0x10001 << $i))
                            }
                        }

                        pub fn is_set_low(&mut self) -> bool {
                            unsafe {
                                (*$PORTX::ptr()).out.read().bits() & (1 << $i) == 0
                            }
                        }

                        pub fn is_set_high(&mut self) -> bool {
                            !self.is_set_low()
                        }
                    }

                    impl<MODE> OutputPin for $PXi<Output<MODE>> {
                        fn set_high(&mut self) {
                            unsafe {
                                (*$PORTX::ptr()).omr.write(|w| w.bits(1 << $i))
                            }
                        }

                        fn set_low(&mut self) {
                            unsafe {
                                (*$PORTX::ptr()).omr.write(|w| w.bits(1 << (16 + $i)))
                            }
                        }
                    }
                )+
            )+
        }
    }
}

gpio!(PORT0, port0, P0x, [
    IOCR0: iocr0, [
        P0_00: (p0_00,  0, Input<Floating>),
        P0_01: (p0_01,  1, Input<Floating>),
        P0_02: (p0_02,  2, Input<Floating>),
        P0_03: (p0_03,  3, Input<Floating>),
    ],
    IOCR4: iocr4, [
        P0_04: (p0_04,  4, Input<Floating>),
        P0_05: (p0_05,  5, Input<Floating>),
        P0_06: (p0_06,  6, Input<Floating>),
        P0_07: (p0_07,  7, Input<Floating>),
    ],
    IOCR8: iocr8, [
        P0_08: (p0_08,  8, Input<Floating>),
        P0_09: (p0_09,  9, Input<Floating>),
        P0_10: (p0_10, 10, Input<Floating>),
        P0_11: (p0_11, 11, Input<Floating>),
    ],
    IOCR12: iocr12, [
        P0_12: (p0_12, 12, Input<Floating>),
        P0_13: (p0_13, 13, Input<Floating>),
        P0_14: (p0_14, 14, Input<Floating>),
        P0_15: (p0_15, 15, Input<Floating>),
    ],
]);

gpio!(PORT1, port1, P1x, [
    IOCR0: iocr0, [
        P1_00: (p1_00, 0, Input<Floating>),
        P1_01: (p1_01, 1, Input<Floating>),
        P1_02: (p1_02, 2, Input<Floating>),
        P1_03: (p1_03, 3, Input<Floating>),
    ],
    IOCR4: iocr4, [
        P1_04: (p1_04, 4, Input<Floating>),
        P1_05: (p1_05, 5, Input<Floating>),
        P1_06: (p1_06, 6, Input<Floating>),
    ],
]);

gpio!(PORT2, port2, P2x, [
    IOCR0: iocr0, [
        P2_00: (p2_00,  0, Input<Floating>),
        P2_01: (p2_01,  1, Input<Floating>),
        P2_02: (p2_02,  2, Input<Floating>),
        P2_03: (p2_03,  3, Input<Floating>),
    ],
    IOCR4: iocr4, [
        P2_04: (p2_04,  4, Input<Floating>),
        P2_05: (p2_05,  5, Input<Floating>),
        P2_06: (p2_06,  6, Input<Floating>),
        P2_07: (p2_07,  7, Input<Floating>),
    ],
    IOCR8: iocr8, [
        P2_08: (p2_08,  8, Input<Floating>),
        P2_09: (p2_09,  9, Input<Floating>),
        P2_10: (p2_10, 10, Input<Floating>),
        P2_11: (p2_11, 11, Input<Floating>),
    ],
]);
