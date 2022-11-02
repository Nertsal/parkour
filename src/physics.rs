use super::*;

mod arm_skeleton;
mod body;
mod running;
mod surface;

pub use arm_skeleton::*;
pub use body::*;
pub use running::*;
pub use surface::*;

#[derive(Debug, Clone, Copy)]
pub struct PhysicsPoint {
    pub position: Position,
    pub radius: Coord,
    pub velocity: Vec2<Coord>,
    pub mass: Mass,
}

impl PhysicsPoint {
    pub fn new(position: Position, radius: Coord, mass: Mass) -> Self {
        Self {
            velocity: Vec2::ZERO,
            position,
            radius,
            mass,
        }
    }

    pub fn movement(&mut self, delta_time: Time) {
        self.position += self.velocity * delta_time;
    }

    pub fn relative(&self, other: &Self) -> Self {
        Self {
            position: self.position + other.position,
            velocity: self.velocity + other.velocity,
            ..*self
        }
    }

    pub fn impulse(&self) -> Vec2<Coord> {
        self.velocity * self.mass
    }
}
