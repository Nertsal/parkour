use super::*;

const ACCELERATION: f32 = 100.0;

pub struct Body {
    pub center: PhysicsPoint,
    pub arm: ArmSkeleton,
    history: running::BodyMovementHistory,
}

impl Body {
    pub fn new(position: Position) -> Self {
        Self {
            center: PhysicsPoint::new(position, Coord::new(1.0), Mass::new(20.0)),
            arm: ArmSkeleton::new(
                PhysicsPoint::new(Vec2::ZERO, Coord::new(0.2), Mass::new(0.5)),
                PhysicsPoint::new(vec2(0.0, -0.45).map(r32), Coord::new(0.15), Mass::new(0.7)),
                PhysicsPoint::new(vec2(0.0, -0.55).map(r32), Coord::new(0.2), Mass::new(1.0)),
            ),
            history: default(),
        }
    }

    pub fn movement(&mut self, control: control::VerifiedBodyControl, delta_time: Time) {
        // Record
        let [_, _, hand] = self.arm.get_skeleton(&self.center);
        let state = running::BodyMovementState {
            time: self.history.time,
            hand: (hand.position - self.center.position) / self.arm.max_reach(), // Normalized
        };
        self.history.update(state, delta_time);
        let info = self.history.analyze();
        let stats = info.calc_stats();

        // Velocity
        let control = control::BodyControl::from(control);
        let target_speed = control.move_speed * stats.move_speed;
        let delta_speed = target_speed - self.center.velocity.x;
        self.center.velocity.x += delta_speed.clamp_abs(Coord::new(ACCELERATION) * delta_time);

        // Movement
        self.move_hand_towards(control.hand_target, delta_time);
        self.center.movement(delta_time);
    }

    fn move_hand_towards(&mut self, relative_target: Position, delta_time: Time) {
        let impulse = self.arm.move_hand_towards(relative_target, delta_time);
        self.center.velocity -= impulse * r32(5.0) / self.center.mass;
    }
}
