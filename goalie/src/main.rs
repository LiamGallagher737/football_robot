#![no_std]
#![no_main]

use lib::{
    color::ColorSensor,
    motors::{Direction, Motor},
};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.d20.into_pull_up_input(),
        pins.d21.into_pull_up_input(),
        50000,
    );

    let mut color_sensor = ColorSensor::new(i2c).unwrap();

    loop {
        let color = color_sensor.read().unwrap();
        ufmt::uwriteln!(&mut serial, "Color: {:?}", color).unwrap();
    }
}
