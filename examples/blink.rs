//! Blink an LED while running the other one continuously, while monitoring the buttons to change
//! the behavior.
//!
//! The example runs in a tight loop and only occasionally reads out the buttons (rather than
//! sleeping, using interrupts and timers, as one would preferably do to be responsive and
//! efficient), but that's what the current HAL implementation can give.
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
use embedded_hal::blocking::delay::DelayMs;

entry!(main);

fn main() -> ! {
    let board = efm32gg_stk3700::Board::new();
    let mut leds = board.leds;
    let buttons = board.buttons;
    let mut delay = board.delay;

    loop {
        if buttons.button1_pressed() {
            leds.led0_off();
        } else {
            leds.led0_on();
        }

        delay.delay_ms(500u16);
        if !buttons.button0_pressed() {
            leds.led1_on();
        }
        delay.delay_ms(500u16);
        leds.led1_off();
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
