use embedded_hal::i2c::I2c;
use lis3mdl::{Address, Error, I16xyz, Lis3mdl};

const X_OFFSET: i16 = 747;
const Y_OFFSET: i16 = -718;

pub struct Compass<I2C: I2c> {
    device: Lis3mdl<I2C>,
}

impl<I2C: I2c> Compass<I2C> {
    pub fn new(i2c: I2C) -> Result<Self, Error> {
        let mut device = Lis3mdl::new(i2c, Address::Addr1C)?;
        let _ = device.set_temperature_sensor_enable(false);
        Ok(Self { device })
    }

    pub fn heading(&mut self) -> Result<f64, Error> {
        let I16xyz { x, y, .. } = self.device.get_raw_mag_axes()?;
        let calibrated_x = f64::from(x - X_OFFSET);
        let calibrated_y = f64::from(y - Y_OFFSET);

        Ok(libm::atan2(calibrated_y, calibrated_x))
    }
}
