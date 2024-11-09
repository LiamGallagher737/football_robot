use embedded_hal::i2c::I2c;
use tcs3472::{AllChannelMeasurement, Error, Tcs3472};
use ufmt::derive::uDebug;

pub struct ColorSensor<I2C: I2c> {
    sensor: Tcs3472<I2C>,
}

impl<I2C: I2c<Error = E>, E> ColorSensor<I2C> {
    /// Setup a new instance of [`ColorSensor`].
    pub fn new(i2c: I2C) -> Result<Self, Error<E>> {
        let mut sensor = Tcs3472::new(i2c);
        sensor.enable()?;
        sensor.enable_rgbc()?;
        while !sensor.is_rgbc_status_valid()? {}
        Ok(Self { sensor })
    }

    /// Read the [`Color`] from the sensor.
    pub fn read(&mut self) -> Result<Color, Error<E>> {
        let AllChannelMeasurement {
            red, green, blue, ..
        } = self.sensor.read_all_channels()?;
        Ok(Color { red, green, blue })
    }
}

#[derive(uDebug)]
pub struct Color {
    red: u16,
    green: u16,
    blue: u16,
}

impl Color {
    /// Create a new [`Color`] with rgb values.
    pub const fn new(red: u16, green: u16, blue: u16) -> Self {
        Self { red, green, blue }
    }

    /// Get the squared distance between this [`Color`] and other one.
    pub fn sqr_distance(&self, other: &Self) -> u16 {
        (self.red - other.red).pow(2)
            + (self.blue - other.blue).pow(2)
            + (self.green - other.green).pow(2)
    }
}
