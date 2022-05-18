use super::*;

mod body;

pub use body::*;

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
}
