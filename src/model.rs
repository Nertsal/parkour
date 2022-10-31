use crate::control::*;

use super::*;

use physics::*;

pub type Time = R32;
pub type Coord = R32;
pub type Position = Vec2<Coord>;
pub type Mass = R32;

pub struct Model {
    pub player: Player,
    pub surfaces: Vec<Surface>,
    pub best_jump: Option<Coord>,
}

pub struct Player {
    pub control: BodyControl,
    pub body: Body,
}

impl Model {
    pub fn new() -> Self {
        Self {
            player: Player::new(Position::ZERO),
            surfaces: default(),
            best_jump: None,
        }
    }
}

impl Player {
    pub fn new(position: Position) -> Self {
        Self {
            control: default(),
            body: Body::new(position),
        }
    }

    pub fn movement(&mut self, delta_time: Time) {
        let control = self.control.verify(&self.body);
        self.control = control.into();
        self.body.movement(control, delta_time);
    }
}
