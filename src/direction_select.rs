use std::f64::consts::PI;

use vek::Vec2;
use specs::prelude::*;

use crate::resources::{GameState, InputState, LastLaunchPoint};

pub struct DirectionSelect;

// The minimum angle on either side, calculated from the horizontal axis at the launch point
const MIN_ANGLE: f64 = 5.0 * PI / 180.0; // radians

impl<'a> System<'a> for DirectionSelect {
    type SystemData = (
        WriteExpect<'a, GameState>,
        ReadExpect<'a, InputState>,
        ReadExpect<'a, LastLaunchPoint>,
    );

    fn run(&mut self, (mut game_state, input, launch): Self::SystemData) {
        match *game_state {
            GameState::SelectDirection if input.perform_action => {},
            _ => return,
        }

        let LastLaunchPoint(launch_point) = *launch;
        let direction = input.pos - launch_point;
        // Arbitrary point on the horizontal axis
        let horizontal_axis = Vec2 {x: 10.0, y: 0.0};
        let initial_angle = horizontal_axis.angle_between(direction);

        if initial_angle >= MIN_ANGLE && initial_angle <= PI - MIN_ANGLE {
            *game_state = GameState::Simulate {initial_angle};
        }
    }
}
