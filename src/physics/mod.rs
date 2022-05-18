use super::*;

mod body;

pub use body::*;

#[derive(Debug, Clone, Copy)]
pub struct PhysicsPoint {
    pub position: Position,
    pub radius: Coord,
    pub velocity: Velocity,
}

impl PhysicsPoint {
    pub fn new(position: Position, radius: Coord) -> Self {
        Self {
            velocity: Velocity::ZERO,
            position,
            radius,
        }
    }

    pub fn relative(&self, other: &Self) -> Self {
        Self {
            position: self.position + other.position,
            velocity: self.velocity + other.velocity,
            radius: self.radius,
        }
    }
}
