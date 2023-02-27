use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Settings {
    pub g: f32,
    pub mass: f32,
    pub drag_coef: f32,
    pub max_dist: f32,
    pub max_velocity: f32,
    pub edge_mode: EdgeMode,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            g: 0.098,
            mass: 1.0,
            drag_coef: 0.65,
            max_dist: 80.0,
            max_velocity: 20.0,
            edge_mode: EdgeMode::WRAP,
        }
    }
}

pub enum EdgeMode {
    WRAP,
    BOUNCE,
    STOP,
}
