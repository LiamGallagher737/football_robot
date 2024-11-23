//! This is the functionality the robot uses when the ball cannot be found.

use crate::Movement;
use lib::{location::Location, motors::Turn};
use radian::Angle;
use ufmt::uwriteln;

pub fn calculate_move(
    _location: Location,
    initial_heading: Angle,
    heading: Angle,
    serial: &mut impl ufmt::uWrite,
) -> Movement {
    // If the robot is off axis, correct it.
    let difference = heading.difference(&initial_heading);
    uwriteln!(serial, "{}", difference);

    if difference.abs().radians() > 0.4 {
        let turn = if difference.radians() < 0.0 {
            Turn::Anticlockwise
        } else {
            Turn::Clockwise
        };
        return Movement::Rotation { turn, speed: 50 };
    }

    Movement::None
}
