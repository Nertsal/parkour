use super::*;

mod body;

pub use body::*;

#[derive(Debug, Clone, Copy)]
pub struct PhysicsPoint {
    pub position: Position,
    pub radius: Coord,
    pub velocity: Velocity,
    pub mass: Mass,
}

impl PhysicsPoint {
    pub fn new(position: Position, radius: Coord, mass: Mass) -> Self {
        Self {
            velocity: Velocity::ZERO,
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

    pub fn impulse(&self) -> Impulse {
        self.velocity * self.mass
    }
}
