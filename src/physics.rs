use specs::prelude::*;

use crate::components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut positions, velocities): Self::SystemData) {
        for (Position(pos), vel) in (&mut positions, &velocities).join() {
            let speed = vel.speed as f64;
            pos.x += vel.angle.cos() * speed;
            // Need to flip the sign because y-axis goes down
            pos.y -= vel.angle.sin() * speed;
        }
    }
}
