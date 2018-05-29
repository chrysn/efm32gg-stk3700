// This example uses no board support crate yet
//
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

extern crate efm32_stk3700;

extern crate panic_semihosting;

use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    let board = efm32_stk3700::init();
    let mut leds = board.leds;
//     let buttons = board.buttons;

    leds.led0_on();

    loop {
//         if !buttons.button0_pressed() {
            leds.led1_on();
//         }
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
