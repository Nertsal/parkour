mod collider;
mod level;
mod logic;
mod physics;

pub use self::{collider::*, level::*, physics::*};

use geng::prelude::*;
use geng_utils::conversions::*;

pub type Time = R32;
pub type Coord = R32;
pub type Position = vec2<Coord>;
pub type Mass = R32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

pub struct Model {
    pub config: Config,
    pub player: Body,
    pub level: Level,
}

impl Model {
    pub fn new(config: Config, level: Level) -> Self {
        Self {
            config,
            player: Body::new(level.spawn_point),
            level,
        }
    }
}
