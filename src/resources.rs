use vek::Vec2;

#[derive(Debug, Clone)]
pub enum GameState {
    SelectDirection,
    Simulate {initial_angle: f64},
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
