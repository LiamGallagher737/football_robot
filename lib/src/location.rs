use crate::color::{Color, ColorSensor};
use embedded_hal::i2c::I2c;
use ufmt::derive::uDebug;

pub struct LocationSensor<I2C: I2c> {
    color_sensor: ColorSensor<I2C>,
    field_colors: FieldColors,
}

impl<I2C: I2c<Error = E>, E> LocationSensor<I2C> {
    /// Create a new instance of [`LocationSensor`] wrapping the given [`ColorSensor`].
    ///
    /// Since this takes ownership of the [`ColorSensor`], a [`LocationSensor::raw_color`] method is provided if you
    /// need to get the raw color read from the color sensor.
    pub fn new(color_sensor: ColorSensor<I2C>, field_colors: FieldColors) -> Self {
        Self {
            color_sensor,
            field_colors,
        }
    }

    /// Get the location with the color closest to what is read from the color sensor.
    pub fn closest(&mut self) -> Result<Location, tcs3472::Error<E>> {
        let color = self.color_sensor.read()?;
        let mut closest_distance = color.sqr_distance(&self.field_colors.goal);
        let mut closest_location = Location::Goal;

        for n in 1..5 {
            let (location, field_color) = match n {
                1 => (Location::Out, &self.field_colors.out),
                2 => (Location::Center, &self.field_colors.center),
                3 => (Location::SideA, &self.field_colors.side_a),
                4 => (Location::SideB, &self.field_colors.side_b),
                _ => unreachable!(),
            };

            let distance = color.sqr_distance(field_color);
            if distance < closest_distance {
                closest_distance = distance;
                closest_location = location;
            }
        }

        Ok(closest_location)
    }

    /// Get the raw color read from [`ColorSensor::read`].
    pub fn raw_color(&mut self) -> Result<Color, tcs3472::Error<E>> {
        self.color_sensor.read()
    }
}

#[derive(uDebug, PartialEq, Eq, Clone, Copy)]
pub enum Location {
    Goal,
    Out,
    Center,
    SideA,
    SideB,
}

pub struct FieldColors {
    pub goal: Color,
    pub out: Color,
    pub center: Color,
    pub side_a: Color,
    pub side_b: Color,
}
