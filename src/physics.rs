use specs::prelude::*;

use crate::components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut positions, velocities): Self::SystemData) {
        for (Position(pos), &Velocity(vel)) in (&mut positions, &velocities).join() {
            *pos = *pos + vel;
        }
    }
}
