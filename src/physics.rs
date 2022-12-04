use super::*;

mod arm_skeleton;
mod body;
mod collider;
mod running;
mod surface;

pub use arm_skeleton::*;
pub use body::*;
pub use collider::*;
pub use running::*;
pub use surface::*;

#[derive(Debug, Clone, Copy)]
pub struct PhysicsBody {
    pub position: Position,
    pub velocity: Vec2<Coord>,
    pub mass: Mass,
    pub collider: Collider,
}

impl PhysicsBody {
    pub fn new(position: Position, mass: Mass, collider: Collider) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            mass,
            collider,
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

    pub fn check_collision(&self, other: &Self) -> Option<Collision> {
        self.collider
            .check_collision(&other.collider, other.position - self.position)
    }
}
