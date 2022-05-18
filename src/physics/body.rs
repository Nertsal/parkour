use super::*;

pub struct Body {
    pub center: PhysicsPoint,
    pub hand: PhysicsPoint,
    pub hand_length: Coord,
}

impl Body {
    pub fn new(position: Position) -> Self {
        Self {
            center: PhysicsPoint::new(position, Coord::new(1.0)),
            hand: PhysicsPoint::new(position, Coord::new(1.0)),
            hand_length: Coord::new(2.0),
        }
    }
}
