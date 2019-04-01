use std::f64::consts::PI;

use vek::Vec2;
use specs::prelude::*;

use crate::components::*;
use crate::resources::{GameState, InputState, LastLaunchPoint};

pub struct BallLauncher;

// The minimum angle on either side, calculated from the horizontal axis at the launch point
const MIN_ANGLE: f64 = 5.0 * PI / 180.0; // radians

const BALL_SPEED: f64 = 15.0;

impl<'a> System<'a> for BallLauncher {
    type SystemData = (
        WriteExpect<'a, GameState>,
        ReadExpect<'a, InputState>,
        ReadExpect<'a, LastLaunchPoint>,
        WriteStorage<'a, Ball>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (
        mut game_state,
        input,
        launch,
        mut balls,
        mut velocities,
    ): Self::SystemData) {
        let initial_angle = match *game_state {
            GameState::SelectDirection if !input.perform_action => return,
            GameState::SelectDirection => {
                // Perform direction selection

                let LastLaunchPoint(launch_point) = *launch;
                let direction = input.pos - launch_point;
                // Arbitrary point on the horizontal axis
                let horizontal_axis = Vec2 {x: 10.0, y: 0.0};
                let initial_angle = horizontal_axis.angle_between(direction);

                if initial_angle >= MIN_ANGLE && initial_angle <= PI - MIN_ANGLE {
                    *game_state = GameState::Simulate {initial_angle};
                    initial_angle
                } else {
                    // Not within the minimum angle
                    return;
                }
            },
            GameState::Simulate {initial_angle} => initial_angle,
        };

        // Launch one ball at a time
        let next_ball = (&mut balls, &mut velocities).join()
            .find(|(b, _)| b.state == BallState::Unlaunched);

        if let Some((ball, vel)) = next_ball {
            ball.state = BallState::Moving;
            vel.speed = BALL_SPEED;
            vel.angle = initial_angle;
        }
    }
}
