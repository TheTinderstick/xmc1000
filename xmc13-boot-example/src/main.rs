//! Prints "Hello, world!" on the OpenOCD console using semihosting
//!
//! ---

#![no_main]
#![no_std]
#![feature(asm)]

#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate cortex_m;
extern crate panic_semihosting;
//extern crate panic_abort;
extern crate r0;

use core::fmt::Write;

use rt::ExceptionFrame;
use sh::hio;
use core::ptr;

struct Constants {
    v1: u32,
    v2: u32,
    v3: u32,
}

/*
pub unsafe extern "C" fn handler_veneer() -> ! {
    let veneer_base: u32;
    let scb_icsr: *const u8;

    // Get the base address of this veneer.
    // We store the base address at +0x34 to save an instruction.
    asm!("ldr $0, [pc, #32]": "=r"(veneer_base) :: "volatile");

    // Get the current exception.
    // ICSR address is 0xE000ED04, address is stored at +0x30
    // 3 is HardFault, where the address is stored at 0x2000_000E
    scb_icsr = ptr::read((veneer_base + 48) as *const u32) as *const u8;
    let idx = ptr::read(scb_icsr) as u8;

    // We use the high 16-bits of each vector to store a compressed
    // representation to the runtime handler.
    // We offset the base by 2 to get the higher 16-bits.
    // The compressed representation is
    //   flash_offset / 4
    // or
    //   ram_offset / 4 + 0xC800
    let loc = veneer_base + 2 + ((idx as u32) << 2);
    let addr = (ptr::read(loc as *const u16) as u32) << 2;

    // flash_base is the start address for flash
    // ram_base is the start address for RAM subtracting the 0xC800
    // ram_cmp is 0xC800 for comparison
    let flash_base: u32;
    let ram_base: u32;
    let ram_cmp: u32;

    // We hardcode the ldmia instruction here because LLVM doesn't always
    // optimise it.
    asm!("ldmia $0, {$0, $1, $2}"
         : "=r"(flash_base), "=r"(ram_base), "=r"(ram_cmp)
         : "0"(veneer_base));

    let full_addr: u32;

    if addr >= ram_cmp {
        full_addr = addr + ram_base;
    } else {
        full_addr = addr + flash_base;
    }

    // The LR register has the correct return from event handler.
    // We use a BX to branch to the handler without changing the LR.
    // We also force idx to be in the r1 position to match the function signature
    //   fn(u8);
    asm!("bx $0" :: "r"(full_addr), "{r1}"(idx));
    loop {}
} */

pre_init!(pre_init);

#[inline(never)]
pub unsafe fn pre_init() {
    extern "C" {
        fn HardFault() -> !;
        fn DefaultHandler();
    }

    const load_next: u32 = 0x4800;
    const branch_r0: u32 = 0x4700;
    const load_and_branch: u32 = load_next | (branch_r0 << 16);

    unsafe fn write_vector(i: u32, val: u32) {
        const vector_base: u32 = 0x2000_0000;
        ptr::write((vector_base + i*4) as *mut u32, val);
    }

    write_vector(3, load_and_branch);
    write_vector(4, HardFault as u32);
    write_vector(5, load_and_branch);
    write_vector(6, DefaultHandler as u32);

    for x in 11..48 {
        let offset = (4 - x) << 1; // We want to jump to 5, but the PC is 1 extra forward
        let ins = 0xE000 | (offset & 0x7FF);
        write_vector(x, ins);
    }
}

entry!(main);

fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, world!").unwrap();

    loop {}
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
