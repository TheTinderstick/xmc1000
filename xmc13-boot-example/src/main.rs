//! Prints "Hello, world!" on the OpenOCD console using semihosting
//!
//! ---

#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate cortex_m;
extern crate panic_semihosting;
//extern crate panic_abort;
extern crate r0;
extern crate xmc1000_hal as hal;

use rt::ExceptionFrame;
use core::ptr;
use hal::prelude::*;

pre_init!(pre_init);

pub unsafe fn pre_init() {
    extern "C" {
        fn HardFault() -> !;
        fn DefaultHandler();
    }

    const VECTOR_BASE: u32 = 0x2000_0000;
    const LOAD_NEXT: u32 = 0x4800;
    const BRANCH_R0: u32 = 0x4700;
    const LOAD_AND_BRANCH: u32 = LOAD_NEXT | (BRANCH_R0 << 16);

    unsafe fn write_vector(i: u32, val: u32) {
        ptr::write((VECTOR_BASE + i*4) as *mut u32, val);
    }

    write_vector(3, LOAD_AND_BRANCH);
    write_vector(4, HardFault as u32);
    write_vector(5, LOAD_AND_BRANCH);
    write_vector(6, DefaultHandler as u32);

    for x in 11..48 {
        let offset = (4 - x) << 1; // We want to jump to 5, but the PC is 1 extra forward
        let ins = 0xE000 | (offset & 0x7FF);
        write_vector(x, ins);
    }
}

entry!(main);

fn main() -> ! {
    let core = hal::xmc1000::CorePeripherals::take().unwrap();
    let peripherals = hal::xmc1000::Peripherals::take().unwrap();
    let mut port0 = peripherals.PORT0.split();
    let mut systick = core.SYST;

    const CORE_CLOCK: u32 = 32_000_000;
    const TICKS_PER_SECOND: u32 = 100;

    systick.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    systick.set_reload(CORE_CLOCK / TICKS_PER_SECOND);
    systick.clear_current();
    systick.enable_counter();
    //systick.enable_interrupt();

    // PCLK to 2*MCLK
    // RTC to DCO2
    // fdiv = 0
    // idiv = 4 => MCLK = 8MHz
    // SCU_CLOCK -> MCLK
    // SysTick handler
    // SysTick -> coreclock / ticks_per_second

    // XMC13_boot LEDs
    // 1 => 0_00
    // 2 => 0_01
    // 3 => 0_06
    // 4 => 0_07
    // 5 => 0_08
    // 6 => 0_09
    let mut led = port0.p0_00.into_push_pull_output(true, &mut port0.iocr0);

    // XMC13_boot other
    // P2_05 has potentiometer (10k from VAREF to AGND)

    let mut ticks = 0;
    loop {
        while !systick.has_wrapped() {}

        ticks += 1;
        if ticks >= 2 * TICKS_PER_SECOND {
            ticks = 0;
            led.toggle();
        }
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    match irqn {
        -1 => {
            cortex_m::asm::bkpt();
        },
        _ => panic!("Unhandled exception (IRQn = {})", irqn),
    }
}
