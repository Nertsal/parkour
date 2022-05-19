use super::*;

use physics::*;

pub type Time = R32;
pub type Coord = R32;
pub type Position = Vec2<Coord>;
pub type Velocity = Vec2<Coord>;
pub type Mass = R32;

pub struct Model {
    pub player: Player,
}

pub struct Player {
    pub relative_target: Position,
    pub body: Body,
}

impl Model {
    pub fn new() -> Self {
        Self {
            player: Player::new(Position::ZERO),
        }
    }
}

impl Player {
    pub fn new(position: Position) -> Self {
        Self {
            relative_target: Position::ZERO,
            body: Body::new(position),
        }
    }
}
