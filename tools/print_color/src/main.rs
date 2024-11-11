//! This tool reads the color sensor and prints its value.

#![no_std]
#![no_main]

use lib::{color::ColorSensor, display::Display};

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

    let bus = shared_bus::BusManagerSimple::new(i2c);

    let mut color_sensor = ColorSensor::new(bus.acquire_i2c()).unwrap();
    let mut display = Display::new(bus.acquire_i2c());

    loop {
        let color = color_sensor.read().unwrap();
        ufmt::uwriteln!(&mut serial, "Color: {}", color).unwrap();
        if let Ok(display) = &mut display {
            ufmt::uwriteln!(
                display,
                "Red: {}\nGreen: {}\nBlue: {}\n\n\n\n\n",
                color.red,
                color.green,
                color.blue
            )
            .unwrap();
        }
    }
}
