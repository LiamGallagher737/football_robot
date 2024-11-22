#![no_std]
#![no_main]

use lib::{
    color::{Color, ColorSensor},
    compass::Compass,
    display::Display,
    location::{FieldColors, LocationSensor},
    motors::Turn,
};
use radian::Angle;

mod dvd;

const X_OFFSET: i16 = 25;
const Y_OFFSET: i16 = -324;

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
    let mut _serial = arduino_hal::default_serial!(dp, pins, 57600);

    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.d20.into_pull_up_input(),
        pins.d21.into_pull_up_input(),
        50000,
    );

    let bus = shared_bus::BusManagerSimple::new(i2c);

    let Ok(_display) = Display::new(bus.acquire_i2c()) else {
        panic!("Failed to initialize display");
    };

    let Ok(mut compass) = Compass::new(bus.acquire_i2c(), X_OFFSET, Y_OFFSET) else {
        panic!("Failed to initialize compass");
    };

    let Ok(color_sensor) = ColorSensor::new(bus.acquire_i2c()) else {
        panic!("Failed to initialize color sensor");
    };

    let mut location_sensor = LocationSensor::new(color_sensor, FIELD_COLORS);

    let mut motors = lib::motors!(dp, pins);

    let initial_heading = compass.heading().unwrap();

    loop {
        let location = location_sensor.closest().unwrap();
        let heading = compass.heading().unwrap();

        let ball_heading: Option<f64> = None;

        let movement = match ball_heading {
            Some(_heading) => todo!(),
            None => dvd::calculate_move(location, initial_heading, heading),
        };

        match movement {
            Movement::Translation { heading, speed } => motors.translate(heading, speed),
            Movement::Rotation { turn, speed } => motors.rotate(turn, speed),
            Movement::None => {}
        };
    }
}

enum Movement {
    Translation { heading: Angle, speed: u8 },
    Rotation { turn: Turn, speed: u8 },
    None,
}

impl ufmt::uDebug for Movement {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        match self {
            Movement::Translation { heading, speed } => f
                .debug_struct("Translation")?
                .field("heading", &(heading.degrees() as i16))?
                .field("speed", speed)?
                .finish(),
            Movement::Rotation { turn, speed } => f
                .debug_struct("Rotation")?
                .field("turn", turn)?
                .field("speed", speed)?
                .finish(),
            Movement::None => f.debug_struct("None")?.finish(),
        }
    }
}
