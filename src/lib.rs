//! This is a board support crate for the [EFM32GG-STK3700] Starter Kit.
//!
//! State and quick-start
//! ---------------------
//!
//! Right now, it is very minimal; LEDs and buttons are exposed in a board struct, but that's about
//! it. It will grow as the underlying [HAL implementation] does.
//!
//! Thus, the larger value is in providing runnable examples. With an STK connected via USB and an
//! [OpenOCD] running (typically ``openocd -f board/efm32.cfg``), a blinking example can be run
//! using ``cargo +nightly run --example blink``.
//!
//! Usage
//! -----
//!
//! See the examples provided (``blink`` is a good start) to get familiar with how the abstract
//! peripherals can be obtained. The usual way is to utilize the board init function, but the main
//! structs of the individual modules can be initialized on their own instead just as well.
//!
//! Noteworthy features
//! -------------------
//!
//! The cargo configuration that enables ``cargo run`` to work is a little more elaborate than that
//! of the [f3] crate that provided me with much guidance: It contains a small gdb-wrapper script
//! in the .cargo directory that detects any usable gdb (might be ``arm-none-eabi-gdb`` or
//! ``gdb-multiarch``), and passes the initial setup commands on the command line rather than using
//! a .gdbinit (because the latter requires [safe-path configuration]).
//!
//! [EFM32GG-STK3700]: https://www.silabs.com/products/development-tools/mcu/32-bit/efm32-giant-gecko-starter-kit
//! [HAL implementation]: https://github.com/chrysn/efm32gg-hal
//! [OpenOCD]: http://openocd.org/
//! [f3]: https://crates.io/crates/f3
//! [safe-path configuration]: https://sourceware.org/gdb/onlinedocs/gdb/Auto_002dloading-safe-path.html#Auto_002dloading-safe-path

#![no_std]

extern crate cortex_m;
extern crate efm32gg990;
extern crate embedded_hal;
extern crate efm32gg_hal;

pub mod led;
pub mod button;
pub mod lcd;

use efm32gg_hal::{
    gpio::GPIOExt,
    cmu::CMUExt,
    systick::SystickExt,
};

pub struct Board {
    pub leds: led::LEDs,
    pub buttons: button::Buttons,
    pub delay: efm32gg_hal::systick::SystickDelay,
    pub lcd: lcd::LCD,
}

/// Initialize the board
///
/// This does little configuration, but primarily ``take``s the system and EFM32 peripherals and
/// distributes them to the the suitable abstractions for the board.
///
/// Peripherals that are not part of the defined board are lost when the structs are taken apart.
/// The current recommendation for composite devices (ie. "The STK3700 with something actually
/// connected to the extension header or breakoutp pins") is to not use this function but rather
/// look at its code, replicate what is needed and add in the composite board's additional devices
/// in a new board initialization function. The author is open to suggestions as to how that would
/// be done better.

pub fn init() -> Board {
    let corep = cortex_m::peripheral::Peripherals::take().unwrap();
    let p = efm32gg990::Peripherals::take().unwrap();

    let mut cmu = p.CMU;

    let gpios = p.GPIO.split(&mut cmu);

    let lcd = lcd::LCD::new(
        gpios.pe5,
        gpios.pe4,
        gpios.pa15,
        gpios.pd9,
    );

    let leds = led::LEDs::new(gpios.pe2, gpios.pe3);

    let buttons = button::Buttons::new(gpios.pb9, gpios.pb10);

    let cmu = cmu.constrain();
    let hfcoreclk = cmu.split().hfcoreclk;
    let syst = corep.SYST.constrain();

    let delay = efm32gg_hal::systick::SystickDelay::new(syst, hfcoreclk);

    Board {
        leds: leds,
        buttons: buttons,
        delay: delay,
        lcd: lcd,
    }
}
