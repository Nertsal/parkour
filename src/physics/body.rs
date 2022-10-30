use super::*;

pub struct Body {
    pub center: PhysicsPoint,
    pub arm: ArmSkeleton,
}

impl Body {
    pub fn new(position: Position) -> Self {
        Self {
            center: PhysicsPoint::new(position, Coord::new(1.0), Mass::new(5.0)),
            arm: ArmSkeleton::new(
                PhysicsPoint::new(Vec2::ZERO, Coord::new(0.2), Mass::new(0.5)),
                PhysicsPoint::new(vec2(0.0, -0.45).map(r32), Coord::new(0.15), Mass::new(0.7)),
                PhysicsPoint::new(vec2(0.0, -0.55).map(r32), Coord::new(0.2), Mass::new(1.0)),
            ),
        }
    }

    pub fn movement(&mut self, relative_target: Position, delta_time: Time) {
        self.move_hand_towards(relative_target, delta_time);
        self.center.movement(delta_time);
    }

    fn move_hand_towards(&mut self, relative_target: Position, delta_time: Time) {
        let impulse = self.arm.move_hand_towards(relative_target, delta_time);
        self.center.velocity -= impulse / self.center.mass;
    }
}
