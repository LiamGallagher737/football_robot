#![no_std]
#![no_main]

use lib::{color::ColorSensor, compass::Compass};

const X_OFFSET: i16 = 747;
const Y_OFFSET: i16 = -718;

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

    let Ok(mut compass) = Compass::new(bus.acquire_i2c(), X_OFFSET, Y_OFFSET) else {
        // ufmt::uwriteln!(&mut serial, "Failed to initialize compass").unwrap();
        panic!("Failed to initialize compass");
    };

    let Ok(mut color_sensor) = ColorSensor::new(bus.acquire_i2c()) else {
        // ufmt::uwriteln!(&mut serial, "Failed to initialize color sensor").unwrap();
        panic!("Failed to initialize color sensor");
    };

    loop {
        let heading = compass.heading().unwrap();
        let color = color_sensor.read().unwrap();
        ufmt::uwriteln!(
            &mut serial,
            "Heading: {}    Color: {:?}",
            heading.to_degrees() as i32,
            color
        )
        .unwrap();
    }
}
