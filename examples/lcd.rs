//! Flash something on the LCD screen when buttons are pressed.
//!
//! See the lcd::LCD documentation for details on the state of the LCD implementation.
//!
//! Everything outside of main is taken from the cortex-m-quickstart examples.

#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

extern crate efm32gg_stk3700;

extern crate embedded_hal;

extern crate panic_semihosting;

use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    let board = efm32gg_stk3700::init();
    let mut lcd = board.lcd;

    loop {
        if board.buttons.button0_pressed() {
            lcd.flash_gecko();
        }
        if board.buttons.button1_pressed() {
            lcd.flash_efm();
        }
    }
}

// define the hard fault handler
exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

// define the default exception handler
exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
