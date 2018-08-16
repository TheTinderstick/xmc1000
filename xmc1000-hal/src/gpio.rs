//! General Purpose Input / Output

use core::marker::PhantomData;

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

pub mod port0 {
    use core::marker::PhantomData;

    use hal::digital::OutputPin;
    use xmc1000::{port0, PORT0};

    use super::{
        Input, Floating, PullUp, PullDown,
        Output, OpenDrain, PushPull,
        Direct, HW0,
    };

    pub struct P0<MODE> {
        i: u8,
        _mode: PhantomData<MODE>,
    }

    impl<MODE> OutputPin for P0<Output<MODE>> {
        fn set_high(&mut self) {
            unsafe {
                (*PORT0::ptr()).omr.write(|w| w.bits(1 << self.i))
            }
        }

        fn set_low(&mut self) {
            unsafe {
                (*PORT0::ptr()).omr.write(|w| w.bits(1 << (16 + self.i)))
            }
        }
    }

    impl<MODE> P0<Output<MODE>> {
        pub fn toggle(&mut self) {
            unsafe {
                (*PORT0::ptr()).omr.write(|w| w.bits(0x10001 << self.i))
            }
        }

        pub fn is_set_low(&mut self) -> bool {
            unsafe {
                (*PORT0::ptr()).out.read().bits() & (1 << self.i) == 0
            }
        }

        pub fn is_set_high(&mut self) -> bool {
            !self.is_set_low()
        }
    }

    pub struct P0_0<MODE> {
        _mode: PhantomData<MODE>,
    }

    impl<MODE> P0_0<MODE> {
        pub fn into_open_drain_output(self, iocr: &mut port0::IOCR0) -> P0_0<Output<OpenDrain>> {
            iocr.modify(|_, w| w.pc0().output_open_drain());

            P0_0 { _mode: PhantomData }
        }

        pub fn into_push_pull_output(self, iocr: &mut port0::IOCR0) -> P0_0<Output<PushPull>> {
            iocr.modify(|_, w| w.pc0().output_push_pull());

            P0_0 { _mode: PhantomData }
        }

        pub fn into_floating_input(self, iocr: &mut port0::IOCR0) -> P0_0<Input<Floating>> {
            iocr.modify(|_, w| w.pc0().input());

            P0_0 { _mode: PhantomData }
        }

        pub fn into_pull_down_input(self, iocr: &mut port0::IOCR0) -> P0_0<Input<PullDown>> {
            iocr.modify(|_, w| w.pc0().input_pull_down());

            P0_0 { _mode: PhantomData }
        }

        pub fn into_pull_up_input(self, iocr: &mut port0::IOCR0) -> P0_0<Input<PullUp>> {
            iocr.modify(|_, w| w.pc0().input_pull_up());

            P0_0 { _mode: PhantomData }
        }

        pub fn into_hw0_direct(self, hwsel: &mut port0::HWSEL) -> P0_0<Direct<HW0>> {
            hwsel.modify(|_, w| w.hw0().value2());

            P0_0 { _mode: PhantomData }
        }
    }

    impl<MODE> P0_0<Output<MODE>> {
        pub fn downgrade(self) -> P0<Output<MODE>> {
            P0 {
                i: 0,
                _mode: self._mode,
            }
        }

        pub fn toggle(&mut self) {
            unsafe {
                (*PORT0::ptr()).omr.write(|w| w.bits(0x10001 << 0))
            }
        }

        pub fn is_set_low(&mut self) -> bool {
            unsafe {
                (*PORT0::ptr()).out.read().bits() & (1 << 0) == 0
            }
        }

        pub fn is_set_high(&mut self) -> bool {
            !self.is_set_low()
        }
    }

    impl<MODE> OutputPin for P0_0<Output<MODE>> {
        fn set_high(&mut self) {
            unsafe {
                (*PORT0::ptr()).omr.write(|w| w.bits(1 << 0))
            }
        }

        fn set_low(&mut self) {
            unsafe {
                (*PORT0::ptr()).omr.write(|w| w.bits(1 << (16 + 0)))
            }
        }
    }
}
