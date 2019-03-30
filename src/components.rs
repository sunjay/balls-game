use specs::prelude::*;
use specs_derive::Component;
use sdl2::rect::Point;
use sdl2::pixels::Color;

/// The current position of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point);

/// The current speed and direction of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity(pub Point);

/// Signifies that a rectangle should be drawn centered around this entity's Position
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ColoredRect {
    /// The color of the rectangle to draw
    pub color: Color,
    /// The width of the rectangle
    pub width: u32,
    /// The height of the rectangle
    pub height: u32,
}
