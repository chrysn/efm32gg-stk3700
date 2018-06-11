use efm32gg_hal::gpio;
use embedded_hal::digital::OutputPin;
use efm32gg_hal::gpio::EFM32Pin;

/// A representation of the two user LEDs on the STK3700
pub struct LEDs {
    led0: gpio::pins::PE2<gpio::Output>,
    led1: gpio::pins::PE3<gpio::Output>,
}

impl LEDs {
    pub fn new(pe2: gpio::pins::PE2<gpio::Disabled>, pe3: gpio::pins::PE3<gpio::Disabled>) -> Self {
        LEDs { led0: pe2.as_output(), led1: pe3.as_output() }
    }

    pub fn led0_on(&mut self)
    {
        self.led0.set_high();
    }

    pub fn led0_off(&mut self)
    {
        self.led0.set_low();
    }

    pub fn led1_on(&mut self)
    {
        self.led1.set_high();
    }

    pub fn led1_off(&mut self)
    {
        self.led1.set_low();
    }
}
