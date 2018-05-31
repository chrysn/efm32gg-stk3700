use efm32gg_hal::gpio;
use embedded_hal::digital::InputPin;
use efm32gg_hal::gpio::EFM32Pin;

pub struct Buttons {
    button0: gpio::pins::PB9<gpio::Input>,
    button1: gpio::pins::PB10<gpio::Input>,
}

impl Buttons {
    pub fn new(pb9: gpio::pins::PB9<gpio::Disabled>, pb10: gpio::pins::PB10<gpio::Disabled>) -> Self {
        Buttons { button0: pb9.as_input(), button1: pb10.as_input() }
    }

    pub fn button0_pressed(&self) -> bool
    {
        self.button0.is_low()
    }

    pub fn button1_pressed(&self) -> bool
    {
        self.button1.is_low()
    }
}
