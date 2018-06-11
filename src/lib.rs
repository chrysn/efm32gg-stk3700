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
