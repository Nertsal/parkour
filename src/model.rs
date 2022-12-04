use super::*;

mod level;

pub use level::*;

pub type Time = R32;
pub type Coord = R32;
pub type Position = Vec2<Coord>;
pub type Mass = R32;

pub struct Model {
    pub player: Body,
    pub level: Level,
    pub surface_collision: Option<usize>,
}

impl Model {
    pub fn new(level: Level) -> Self {
        Self {
            player: Body::new(level.spawn_point),
            level,
            surface_collision: None,
        }
    }
}
