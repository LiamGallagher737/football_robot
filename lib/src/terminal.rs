use arduino_hal::{
    hal::port::{PE0, PE1},
    pac::USART0,
    port::{
        mode::{Input, Output},
        Pin,
    },
    Usart,
};
use embedded_hal::i2c::I2c;

use crate::display::Display;

pub struct Terminal<I2C: I2c> {
    usb: Option<Usart<USART0, Pin<Input, PE0>, Pin<Output, PE1>>>,
    display: Option<Display<I2C>>,
}

impl<I2C: I2c> Terminal<I2C> {
    pub fn new() -> Self {
        Self {
            usb: None,
            display: None,
        }
    }

    pub fn with_usb(mut self, usb: Usart<USART0, Pin<Input, PE0>, Pin<Output, PE1>>) -> Self {
        self.usb = Some(usb);
        self
    }

    pub fn with_display(mut self, display: Display<I2C>) -> Self {
        self.display = Some(display);
        self
    }
}

impl<I2C: I2c> ufmt::uWrite for Terminal<I2C> {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        if let Some(usb) = &mut self.usb {
            let _ = ufmt::uwrite!(usb, "{}", s);
        }

        if let Some(display) = &mut self.display {
            let _ = ufmt::uwrite!(display, "{}", s);
        }

        Ok(())
    }
}
