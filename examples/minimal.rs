//! This example uses no board support crate yet, but is the developer's base line for testing,
//! derived from the cortex-m-quickstart examples.
//!
//! It does nothing, just idles in an infinite loop.

#![no_main]
#![no_std]
#![feature(lang_items)]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
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
