use efm32gg990::{GPIO, CMU};

pub struct LEDs {
    // Actually, we only need PE2 and PE3; a HAL could help us .split() that and retain ownership
    // of only what we need.
    gpio: GPIO,
}

impl LEDs {
    pub fn new(gpio: GPIO, cmu: &mut CMU) -> Self {
        cmu.hfperclken0.modify(|_, w| w.gpio().bit(true));

        // the "Wired OR" mode drives active high
        gpio.pe_model.modify(|_, w| w.mode2().wiredor().mode3().wiredor());

        LEDs { gpio }
    }

    pub fn led0_on(&mut self)
    {
        self.gpio.pe_doutset.write(|w| unsafe { w.bits(1 << 2) });
    }

    pub fn led0_off(&mut self)
    {
        self.gpio.pe_doutclr.write(|w| unsafe { w.bits(1 << 2) });
    }

    pub fn led1_on(&mut self)
    {
        self.gpio.pe_doutset.write(|w| unsafe { w.bits(1 << 3) });
    }

    pub fn led1_off(&mut self)
    {
        self.gpio.pe_doutclr.write(|w| unsafe { w.bits(1 << 3) });
    }
}
