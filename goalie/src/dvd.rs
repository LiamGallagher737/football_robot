//! This is the functionality the robot uses when the ball cannot be found.

use crate::Movement;
use core::f64::consts::PI;
use lib::{location::Location, motors::Turn};
use radian::Angle;

pub fn calculate_move(_location: Location, initial_heading: Angle, heading: Angle) -> Movement {
    // If the robot is off axis, correct it.
    let distance = heading.distance(&initial_heading);
    if distance.radians() > 0.4 {
        let turn = if heading.is_clockwise_to(&initial_heading) {
            Turn::Anticlockwise
        } else {
            Turn::Clockwise
        };
        return Movement::Rotation {
            turn,
            speed: (distance.radians() / PI * 255.0).min(50.0) as u8,
        };
    }

    Movement::None
}
