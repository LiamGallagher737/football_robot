#![no_std]
#![no_main]

use lib::{
    color::{Color, ColorSensor},
    compass::Compass,
    display::Display,
    location::{FieldColors, LocationSensor},
    terminal::Terminal,
};

const X_OFFSET: i16 = 747;
const Y_OFFSET: i16 = -718;
const FIELD_COLORS: FieldColors = FieldColors {
    goal: Color::new(0, 0, 0),
    out: Color::new(0, 0, 0),
    center: Color::new(0, 0, 0),
    side_a: Color::new(0, 0, 0),
    side_b: Color::new(0, 0, 0),
};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);

    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.d20.into_pull_up_input(),
        pins.d21.into_pull_up_input(),
        50000,
    );

    let bus = shared_bus::BusManagerSimple::new(i2c);

    let Ok(display) = Display::new(bus.acquire_i2c()) else {
        panic!("Failed to initialize display");
    };

    let mut terminal = Terminal::new().with_usb(serial).with_display(display);

    let Ok(mut compass) = Compass::new(bus.acquire_i2c(), X_OFFSET, Y_OFFSET) else {
        panic!("Failed to initialize compass");
    };

    let Ok(color_sensor) = ColorSensor::new(bus.acquire_i2c()) else {
        panic!("Failed to initialize color sensor");
    };

    let mut location_sensor = LocationSensor::new(color_sensor, FIELD_COLORS);

    loop {
        let heading = compass.heading().unwrap();
        let location = location_sensor.closest().unwrap();
        ufmt::uwriteln!(
            &mut terminal,
            "\n\n\nHeading: {}\nLocation: {:?}\n\n\n",
            heading.to_degrees() as i32,
            location,
        )
        .unwrap();
    }
}
