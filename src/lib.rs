#![no_std]

extern crate efm32gg990;

use efm32gg990::gpio;

pub mod led;
pub mod button;

// As long as there is no abstraction that allows GPIO to be split, you can only have LEDs or
// buttons
pub struct Board {
    pub leds: led::LEDs,
//     pub buttons: button::Buttons,
}

pub fn init() -> Board {
    let p = efm32gg990::Peripherals::take().unwrap();

    let mut cmu = p.CMU;
    let mut gpio = p.GPIO;

    let leds = led::LEDs::new(gpio, &mut cmu);

//     let buttons = button::Buttons::new(gpio, &mut cmu);

    Board {
        leds,
//         buttons,
    }
}
