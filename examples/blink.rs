// Everything outside of main is taken from the cortex-m-quickstart examples and appears to be the
// most minimal working environment available so far.
//
// Note that this example does indeed blink, but very fast for lack of anything time related
// implemented; then again, it lets you suppress the blinking by the push of a button, so you could
// blink manually...

#![no_main]
#![no_std]
#![feature(lang_items)]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

extern crate efm32gg_stk3700;

extern crate embedded_hal;

use rt::ExceptionFrame;
use embedded_hal::blocking::delay::DelayMs;

entry!(main);

fn main() -> ! {
    let board = efm32gg_stk3700::init();
    let mut leds = board.leds;
    let buttons = board.buttons;
    let mut delay = board.delay;

    leds.led0_on();

    loop {
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

// For any non-minimal demo, and especially during development, you'll likely rather use this
// crate and remove everything below here and the lang_items feature.
//
// extern crate panic_semihosting;

#[lang="panic_fmt"]
extern fn panic_fmt() -> ! {
    loop {}
}
#[no_mangle]
pub fn rust_begin_unwind() -> ! {
    loop {}
}
