use vek::Vec2;

#[derive(Debug, Clone)]
pub enum GameState {
    SelectDirection,
    Simulate {
        /// The initial angle at which to launch each ball
        initial_angle: f64,
    },
}

#[derive(Debug, Default, Clone)]
pub struct InputState {
    /// The position of the mouse/touch in world coordinates
    pub pos: Vec2<f64>,
    /// true if an action should be performed (mouse click, touch release)
    pub perform_action: bool,
}

#[derive(Debug, Clone)]
pub struct LastLaunchPoint(pub Vec2<f64>);

#[derive(Debug, Clone)]
pub struct GameBoundary {
    /// The top left corner of the boundary in world coordinates
    pub top_left: Vec2<f64>,
    /// The top right corner of the boundary in world coordinates
    pub top_right: Vec2<f64>,
    /// The bottom left corner of the boundary in world coordinates
    pub bottom_left: Vec2<f64>,
    /// The bottom right corner of the boundary in world coordinates
    pub bottom_right: Vec2<f64>,
}
