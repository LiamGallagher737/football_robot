#![no_std]
#![no_main]

use lib::motors::{Direction, Motor};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut motors = lib::motors!(dp, pins);

    motors.set_direction(Motor::Dribbler, Direction::Forward);

    loop {
        motors.set_speed(Motor::Dribbler, 50);
    }
}
