// This example uses no board support crate yet
//
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

extern crate efm32gg990;

extern crate panic_semihosting;

use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    let p = efm32gg990::Peripherals::take().unwrap();

    let cmu = p.CMU;

    if cmu.ctrl.read().hfxoboost().is_50pcent() { panic!("matches") }

    cmu.hfperclken0.modify(|_, w| w.gpio().bit(true));
    let gpio = p.GPIO;
    // leds are pe2 and pe3
    gpio.pe_model.modify(|_, w| w.mode2().pushpull());
    gpio.pe_doutset.write(|w| unsafe { w.bits(1 << 2) });
    // led1 not set to pushpull, but left at default (disabled w/ pullup)
    gpio.pe_doutset.write(|w| unsafe { w.bits(1 << 3) });

    // FIXME: not blinking yet
    loop {}
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
