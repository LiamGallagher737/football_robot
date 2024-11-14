#![no_std]
#![no_main]

use lib::{ir::IrSensor, ir_sensors};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut ir_sensors = ir_sensors!(dp, pins, [0; 8]);

    let mut left_min = u16::MAX;
    let mut left_max = u16::MIN;
    let mut right_min = u16::MAX;
    let mut right_max = u16::MIN;

    loop {
        let left = ir_sensors.read_single(IrSensor::IR3);
        let right = ir_sensors.read_single(IrSensor::IR7);

        if left_min > left {
            left_min = left;
        }

        if left_max < left {
            left_max = left;
        }

        if right_min > right {
            right_min = right;
        }

        if right_max < right {
            right_max = right;
        }

        let left_offset = (left_min + left_max) / 2;
        let right_offset = (right_min + right_max) / 2;

        ufmt::uwriteln!(
            &mut serial,
            "Left Offset: {}\tRight Offset: {}",
            left_offset,
            right_offset
        )
        .unwrap();

        arduino_hal::delay_ms(100);
    }
}
