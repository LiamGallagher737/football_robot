#![no_std]
#![no_main]

use lis3mdl::{Address, I16xyz, Lis3mdl};
use panic_halt as _;
use ufmt_float::uFmt_f32;

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

    let mut lis3mdl = Lis3mdl::new(i2c, Address::Addr1C).unwrap();
    let _ = lis3mdl.set_temperature_sensor_enable(false);

    let mut x_min = i16::MAX;
    let mut x_max = i16::MIN;

    let mut y_min = i16::MAX;
    let mut y_max = i16::MIN;

    loop {
        let I16xyz { x, y, .. } = lis3mdl.get_raw_mag_axes().unwrap();

        if x < x_min {
            x_min = x;
        }

        if x > x_max {
            x_max = x;
        }

        if y < y_min {
            y_min = y;
        }

        if y > y_max {
            y_max = y;
        }

        let x_offset = (x_min + x_max) / 2;
        let y_offset = (y_min + y_max) / 2;

        let x_scale = 2.0 / (x_max - x_min) as f32;
        let y_scale = 2.0 / (y_max - y_min) as f32;

        ufmt::uwriteln!(
            &mut serial,
            "X-OFFSET: {}, Y-OFFSET: {}, X-SCALE: {}, Y-SCALE: {}"
            x_offset, y_offset, uFmt_f32::Five(x_scale), uFmt_f32::Five(y_scale)
        )
        .unwrap();
    }
}
