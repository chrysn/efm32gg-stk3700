/// This is a very low-quality demo subset of what could be in a BSP for the LCD.
///
/// All it currently does is grab some of the LCD pins and offers two single icon flashing methods.
/// This implementation does not use the built-in LCD engine, voltage boost or anything else
/// advanced, it is here solely to explore how well the mechanisms of passing around peripherals
/// work, and to provide a demo along which to bootstrap a later proper implementation.

use efm32gg_hal::gpio::pins::*;
use efm32gg_hal::gpio::{
    EFM32Pin,
    Output,
    Disabled,
};
use embedded_hal::digital::OutputPin;

/// The LCD struct represents the 160-segment liquid crystal display on the STK3700 board.
///
/// It is constructed at board setup and holds all resources required for actuating it (thus, it
/// can only be constructed once, as they only exist once).
pub struct LCD {
    com6: PE5<Output>,
    com7: PE4<Output>,
    seg0: PA15<Output>,
    seg8: PD9<Output>,
}

impl LCD {
    // I'd prefer to say <any>, but then I'd have to define this impl as generic over any pin.
    pub fn new(com6: PE5<Disabled>, com7: PE4<Disabled>, seg0: PA15<Disabled>, seg8: PD9<Disabled>) -> LCD
    {
        LCD {
            com6: com6.as_output(),
            com7: com7.as_output(),
            seg0: seg0.as_output(),
            seg8: seg8.as_output(),
        }
    }

    /// Briefly flash the Gecko symbol at the top left corner. This is best called in a loop as
    /// long as it should be visible.
    pub fn flash_gecko(&mut self)
    {
        self.com6.set_low();
        self.seg0.set_low();
        self.seg0.set_high();
        self.com6.set_high();
    }

    /// Briefly flash the EFM32 symbol. This is best called in a loop as long as it should be
    /// visible.
    pub fn flash_efm(&mut self)
    {
        self.com7.set_low();
        for _i in 0..3 {
            // this needs a little more, it seems -- time to get the engine running
            self.seg8.set_low();
            self.seg8.set_high();
        }
        self.com7.set_high();
    }
}
