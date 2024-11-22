#![no_std]
#![no_main]

use lib::{compass::Compass, ir::IrSensor, ir_sensors, motors};

const X_OFFSET: i16 = 126;
const Y_OFFSET: i16 = -646;

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

    let Ok(mut compass) = Compass::new(i2c, X_OFFSET, Y_OFFSET) else {
        panic!("Failed to initialize compass");
    };

    let mut _motors = motors!(dp, pins);

    let mut ir_sensors = ir_sensors!(dp, pins, [55, 81, 0, 59, 93, 85, 129, 103]);

    let zero_heading = compass.heading().unwrap().degrees() + 180.0;

    loop {
        let heading = compass.heading().unwrap().degrees();
        let readings = [
            ir_sensors.read_single(IrSensor::IR3),
            ir_sensors.read_single(IrSensor::IR7),
        ];

        let mut ball_direction = heading - zero_heading;
        if ball_direction.is_sign_negative() {
            ball_direction += 360.0;
        }

        ufmt::uwrite!(&mut serial, "{},", ball_direction as i16).unwrap();
        for reading in readings {
            ufmt::uwrite!(&mut serial, "{},", reading).unwrap();
        }
        ufmt::uwriteln!(&mut serial, "").unwrap();

        arduino_hal::delay_ms(100);
    }
}
