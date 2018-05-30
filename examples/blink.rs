// Everything outside of main is taken from the cortex-m-quickstart examples and appears to be the
// most minimal working environment available so far.
//
// Note that this example does indeed blink, but very fast for lack of anything time related
// implemented; then again, it lets you suppress the blinking by the push of a button, so you could
// blink manually...

#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

extern crate efm32gg_stk3700;

extern crate panic_semihosting;

use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    let board = efm32gg_stk3700::init();
    let mut leds = board.leds;
    let buttons = board.buttons;

    leds.led0_on();

    loop {
        if !buttons.button0_pressed() {
            leds.led1_on();
        }
        leds.led1_off();
        leds.led1_off();
        leds.led1_off();
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
