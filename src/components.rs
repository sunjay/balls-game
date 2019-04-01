use specs::prelude::*;
use specs_derive::Component;
use sdl2::pixels::Color;
use vek::Vec2;

/// The current position of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Vec2<f64>);

/// The current speed and direction of a given entity
#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Velocity {
    /// The speed of the ball
    pub speed: f64,
    /// The angle of the ball in radians
    pub angle: f64,
}

/// Signifies that a rectangle should be drawn centered around this entity's Position
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Block {
    /// The numeric value shown on the block
    pub value: usize,
    /// The color of the rectangle to draw
    pub color: Color,
    /// The width of the rectangle
    pub width: u32,
    /// The height of the rectangle
    pub height: u32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BallState {
    /// Ball hasn't been launched yet
    Unlaunched,
    /// Ball is being simulated
    Moving,
    /// Ball is stopped
    Stopped,
}

/// Signifies that a rectangle should be drawn centered around this entity's Position
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Ball {
    /// The radius of the ball
    pub radius: u32,
    /// The color of the ball
    pub color: Color,
    /// The current state of this ball
    pub state: BallState,
}
