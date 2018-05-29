use efm32gg990::{GPIO, CMU};

pub struct Buttons {
    // see led::LEDs comment
    gpio: GPIO,
}

impl Buttons {
    pub fn new(gpio: GPIO, cmu: &mut CMU) -> Self {
        cmu.hfperclken0.modify(|_, w| w.gpio().bit(true));

        gpio.pe_modeh.modify(|_, w| w.mode9().wiredor().mode10().wiredor());

        Buttons { gpio }
    }

    pub fn button0_pressed(&self) -> bool
    {
        self.gpio.pb_din.read().bits() & (1 << 9) != 0
    }

    pub fn button1_pressed(&self) -> bool
    {
        self.gpio.pb_din.read().bits() & (1 << 10) != 0
    }
}
