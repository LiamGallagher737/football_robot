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
    /// Setup a new instance of [`Motors`]. Use the [`motors`] macro rather than this directly.
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
