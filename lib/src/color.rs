use arduino_hal::I2c;
use tcs3472::{AllChannelMeasurement, Tcs3472};
use ufmt::derive::uDebug;

type Error = tcs3472::Error<arduino_hal::i2c::Error>;

pub struct ColorSensor {
    sensor: Tcs3472<I2c>,
}

impl ColorSensor {
    /// Setup a new instance of [`ColorSensor`].
    pub fn new(i2c: I2c) -> Result<Self, Error> {
        let mut sensor = Tcs3472::new(i2c);
        sensor.enable()?;
        sensor.enable_rgbc()?;
        while !sensor.is_rgbc_status_valid()? {}
        Ok(Self { sensor })
    }

    /// Read the [`Color`] from the sensor.
    pub fn read(&mut self) -> Result<Color, Error> {
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
    /// Get the distance between this [`Color`] and other one.
    pub fn distance(&self, other: &Self) -> f32 {
        libm::sqrtf(f32::from(self.sqr_distance(other)))
    }

    /// Get the squared distance between this [`Color`] and other one.
    pub fn sqr_distance(&self, other: &Self) -> u16 {
        (self.red - other.red).pow(2)
            + (self.blue - other.blue).pow(2)
            + (self.green - other.green).pow(2)
    }
}
