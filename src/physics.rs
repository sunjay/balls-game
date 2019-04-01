use vek::Vec2;
use specs::prelude::*;

use crate::components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut positions, velocities): Self::SystemData) {
        for (Position(pos), vel) in (&mut positions, &velocities).join() {
            let speed = vel.speed as f64;

            // Need to flip the y-axis sign because the y-axis goes down
            let direction = Vec2 {x: vel.angle.cos(), y: -vel.angle.sin()};
            let next_pos = *pos + direction * speed;
            *pos = next_pos;
        }
    }
}
