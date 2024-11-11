use core::fmt::Write;
use embedded_hal::i2c::I2c;
use ssd1306::mode::TerminalModeError;
use ssd1306::prelude::*;
use ssd1306::{mode::TerminalMode, I2CDisplayInterface, Ssd1306};

pub struct Display<I2C: I2c> {
    device: Ssd1306<I2CInterface<I2C>, DisplaySize128x64, TerminalMode>,
}

impl<I2C: I2c> Display<I2C> {
    pub fn new(i2c: I2C) -> Result<Self, TerminalModeError> {
        let interface = I2CDisplayInterface::new(i2c);
        let mut device = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_terminal_mode();
        device.init()?;
        let _ = device.clear();
        Ok(Self { device })
    }
}

impl<I2C: I2c> ufmt::uWrite for Display<I2C> {
    type Error = core::fmt::Error;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.device.write_str(s)
    }
}
