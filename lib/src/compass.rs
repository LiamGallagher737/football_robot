use embedded_hal::i2c::I2c;
use lis3mdl::{Address, Error, I16xyz, Lis3mdl};

pub struct Compass<I2C: I2c> {
    device: Lis3mdl<I2C>,
    x_offset: i16,
    y_offset: i16,
}

impl<I2C: I2c> Compass<I2C> {
    pub fn new(i2c: I2C, x_offset: i16, y_offset: i16) -> Result<Self, Error> {
        let mut device = Lis3mdl::new(i2c, Address::Addr1C)?;
        let _ = device.set_temperature_sensor_enable(false);
        Ok(Self {
            device,
            x_offset,
            y_offset,
        })
    }

    pub fn heading(&mut self) -> Result<f64, Error> {
        let I16xyz { x, y, .. } = self.device.get_raw_mag_axes()?;
        let calibrated_x = f64::from(x - self.x_offset);
        let calibrated_y = f64::from(y - self.y_offset);

        Ok(libm::atan2(calibrated_y, calibrated_x))
    }
}
