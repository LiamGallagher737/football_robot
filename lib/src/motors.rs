use arduino_hal::{
    hal::port::{PA0, PA2, PA4, PA6, PB4, PB5, PB6, PB7},
    pac::{TC1, TC2},
    port::{
        mode::{Floating, Input, Output, PwmOutput},
        Pin,
    },
    simple_pwm::{IntoPwmPin, Prescaler, Timer1Pwm, Timer2Pwm},
};
use ufmt::derive::uDebug;

#[derive(uDebug, Clone, Copy)]
pub enum Motor {
    FrontLeft,
    FrontRight,
    Rear,
    Dribbler,
}

#[derive(uDebug, Clone, Copy)]
pub enum Direction {
    Forward,
    Back,
}

#[derive(uDebug, Clone, Copy)]
pub enum Turn {
    Clockwise,
    Anticlockwise,
}

pub struct Motors {
    // The pin types can be found by hovering on `pins.d**`.
    front_left: MotorPins<Timer1Pwm, PB5, PA4>,
    front_right: MotorPins<Timer1Pwm, PB7, PA0>,
    rear: MotorPins<Timer1Pwm, PB6, PA2>,
    dribbler: MotorPins<Timer2Pwm, PB4, PA6>,
}

struct MotorPins<TC, EN, PH> {
    en: Pin<PwmOutput<TC>, EN>,
    ph: Pin<Output, PH>,
}

impl Motors {
    /// Setup a new instance of [`Motors`]. Use the [`crate::motors!`] macro rather than this directly.
    pub fn new(
        tc1: TC1,
        tc2: TC2,
        d10: Pin<Input<Floating>, PB4>,
        d11: Pin<Input<Floating>, PB5>,
        d12: Pin<Input<Floating>, PB6>,
        d13: Pin<Input<Floating>, PB7>,
        d22: Pin<Input<Floating>, PA0>,
        d24: Pin<Input<Floating>, PA2>,
        d26: Pin<Input<Floating>, PA4>,
        d28: Pin<Input<Floating>, PA6>,
    ) -> Self {
        let timer1 = Timer1Pwm::new(tc1, Prescaler::Prescale64);
        let timer2 = Timer2Pwm::new(tc2, Prescaler::Prescale64);

        let mut motors = Self {
            front_left: MotorPins {
                en: d11.into_output().into_pwm(&timer1),
                ph: d26.into_output(),
            },
            front_right: MotorPins {
                en: d13.into_output().into_pwm(&timer1),
                ph: d22.into_output(),
            },
            rear: MotorPins {
                en: d12.into_output().into_pwm(&timer1),
                ph: d24.into_output(),
            },
            dribbler: MotorPins {
                en: d10.into_output().into_pwm(&timer2),
                ph: d28.into_output(),
            },
        };

        motors.front_left.en.enable();
        motors.front_right.en.enable();
        motors.rear.en.enable();
        motors.dribbler.en.enable();

        motors
    }

    /// Move the entire robot using the motors.
    pub fn translate(&mut self, heading: f64, speed: u8) {
        // Turn the heading in to its X and Y parts.
        let ax = libm::cos(heading);
        let ay = libm::sin(heading);

        // Calculate the amount each motor contributes.
        let fl_amount = ax * 0.58 + ay * -0.33;
        let fr_amount = ax * -0.58 + ay * -0.33;
        let rear_amount = ay * 0.67;

        // Find the max contribute amount.
        let mut max_amount = libm::fabs(fl_amount);
        for a in [libm::fabs(fr_amount), libm::fabs(rear_amount)] {
            if max_amount < a {
                max_amount = a;
            }
        }

        // Get a multiplier to normalize the contribution
        // amounts so the highest value is 1.0.
        let multiplier = 1.0 / max_amount;

        // Calculate the speed by taking the product of absolute of the contribute
        // amount, the normalizing multiplier and the provided speed.
        let fl_value = fl_amount * multiplier * speed as f64;
        let fr_value = fr_amount * multiplier * speed as f64;
        let rear_value = rear_amount * multiplier * speed as f64;

        self.set_motor(Motor::FrontLeft, fl_value as i16);
        self.set_motor(Motor::FrontRight, fr_value as i16);
        self.set_motor(Motor::Rear, rear_value as i16);
    }

    /// Rotate the entire robot using the motors.
    pub fn rotate(&mut self, turn: Turn, speed: u8) {
        for motor in [Motor::FrontLeft, Motor::FrontRight, Motor::Rear] {
            self.set_speed(motor, speed);
        }

        let direction = match turn {
            Turn::Clockwise => Direction::Forward,
            Turn::Anticlockwise => Direction::Back,
        };

        self.set_direction(Motor::FrontLeft, direction);
        self.set_direction(Motor::FrontRight, direction);
        self.set_direction(Motor::Rear, direction);
    }

    /// Set the speed of a single motor.
    pub fn set_speed(&mut self, motor: Motor, speed: u8) {
        match motor {
            Motor::FrontLeft => self.front_left.en.set_duty(speed),
            Motor::FrontRight => self.front_right.en.set_duty(speed),
            Motor::Rear => self.rear.en.set_duty(speed),
            Motor::Dribbler => self.dribbler.en.set_duty(speed),
        }
    }

    /// Set the direction a single motor.
    pub fn set_direction(&mut self, motor: Motor, direction: Direction) {
        match (motor, direction) {
            (Motor::FrontLeft, Direction::Forward) => self.front_left.ph.set_high(),
            (Motor::FrontLeft, Direction::Back) => self.front_left.ph.set_low(),

            (Motor::FrontRight, Direction::Forward) => self.front_right.ph.set_high(),
            (Motor::FrontRight, Direction::Back) => self.front_right.ph.set_low(),

            (Motor::Rear, Direction::Forward) => self.rear.ph.set_high(),
            (Motor::Rear, Direction::Back) => self.rear.ph.set_low(),

            (Motor::Dribbler, Direction::Forward) => self.dribbler.ph.set_high(),
            (Motor::Dribbler, Direction::Back) => self.dribbler.ph.set_low(),
        }
    }

    /// Set the motors speed and direction with a single value.
    ///
    /// NOTE: The values absolute must be less than or equal to 255.
    fn set_motor(&mut self, motor: Motor, value: i16) {
        assert!(value.abs() <= 255, "Max motor speed is 255");

        self.set_speed(motor, value.abs() as u8);
        self.set_direction(
            motor,
            if value.is_positive() {
                Direction::Forward
            } else {
                Direction::Back
            },
        );
    }
}

#[macro_export]
macro_rules! motors {
    ($dp:expr, $pins:expr) => {
        $crate::motors::Motors::new(
            $dp.TC1, $dp.TC2, $pins.d10, $pins.d11, $pins.d12, $pins.d13, $pins.d22, $pins.d24,
            $pins.d26, $pins.d28,
        )
    };
}
