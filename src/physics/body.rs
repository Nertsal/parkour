use super::*;

pub struct Body {
    pub center: PhysicsBody,
    pub arm: ArmSkeleton,
    pub arm_back: ArmSkeleton,
    pub holding_to: Option<Vec2<Coord>>,
    pub ground_normal: Option<Vec2<Coord>>,
    pub history: running::BodyMovementHistory,
}

impl Body {
    pub fn new(position: Position) -> Self {
        Self {
            center: PhysicsBody::new(
                position,
                Mass::new(20.0),
                Collider::rectangle(Coord::new(1.0), Coord::new(2.0)),
            ),
            arm: ArmSkeleton::new(
                PhysicsBody::new(
                    vec2(0.0, 0.5).map(Coord::new),
                    Mass::new(0.5),
                    Collider::square(Coord::new(0.4)),
                ),
                PhysicsBody::new(
                    vec2(0.0, -0.7).map(Coord::new),
                    Mass::new(0.7),
                    Collider::square(Coord::new(0.3)),
                ),
                PhysicsBody::new(
                    vec2(0.0, -0.8).map(Coord::new),
                    Mass::new(1.0),
                    Collider::square(Coord::new(0.4)),
                ),
            ),
            arm_back: ArmSkeleton::new(
                PhysicsBody::new(
                    vec2(0.0, 0.5).map(Coord::new),
                    Mass::new(0.5),
                    Collider::square(Coord::new(0.4)),
                ),
                PhysicsBody::new(
                    vec2(0.0, -0.7).map(Coord::new),
                    Mass::new(0.7),
                    Collider::square(Coord::new(0.3)),
                ),
                PhysicsBody::new(
                    vec2(0.0, -0.8).map(Coord::new),
                    Mass::new(1.0),
                    Collider::square(Coord::new(0.4)),
                ),
            ),
            holding_to: None,
            ground_normal: None,
            history: default(),
        }
    }

    pub fn try_holding(&mut self, surfaces: &[Surface]) {
        let [_, _, hand] = self.arm.get_skeleton(&self.center);
        let point = surfaces
            .iter()
            .flat_map(|surface| [surface.p1, surface.p2])
            .find(|&p| hand.collider.contains(p, hand.position));
        if let Some(point) = point {
            self.holding_to = Some(point);
        }
    }
}
