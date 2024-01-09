use super::*;

pub struct Body {
    pub center: PhysicsPoint,
    pub arm: ArmSkeleton,
    pub holding_to: Option<vec2<Coord>>,
    pub ground_normal: Option<vec2<Coord>>,
    pub history: running::BodyMovementHistory,
}

impl Body {
    pub fn new(position: Position) -> Self {
        Self {
            center: PhysicsPoint::new(position, Coord::new(1.0), Mass::new(20.0)),
            arm: ArmSkeleton::new(
                PhysicsPoint::new(vec2::ZERO, Coord::new(0.2), Mass::new(0.5)),
                PhysicsPoint::new(vec2(0.0, -0.7).map(r32), Coord::new(0.15), Mass::new(0.7)),
                PhysicsPoint::new(vec2(0.0, -0.8).map(r32), Coord::new(0.2), Mass::new(1.0)),
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
            .find(|&p| (p - hand.position).len() <= hand.radius);
        if let Some(point) = point {
            self.holding_to = Some(point);
        }
    }
}
