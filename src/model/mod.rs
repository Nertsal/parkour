use super::*;

use physics::*;

pub type Time = R32;
pub type Coord = R32;
pub type Position = Vec2<Coord>;
pub type Velocity = Vec2<Coord>;

pub struct Model {
    pub player: Player,
}

pub struct Player {
    pub hand_target_delta: Position,
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
            hand_target_delta: Position::ZERO,
            body: Body::new(position),
        }
    }
}
