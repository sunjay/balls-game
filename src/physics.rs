use specs::prelude::*;
use sdl2::rect::Point;

use crate::components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut positions, velocities): Self::SystemData) {
        for (Position(pos), vel) in (&mut positions, &velocities).join() {
            let speed = vel.speed as f64;
            let x = pos.x() as f64 + vel.angle.cos() * speed;
            // Need to flip the sign because y-axis goes down
            let y = pos.y() as f64 - vel.angle.sin() * speed;

            // Do the truncation at the very end
            *pos = Point::new(x as i32, y as i32);
        }
    }
}
