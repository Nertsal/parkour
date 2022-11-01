use super::*;

const GROUND_ACCELERATION: f32 = 100.0;
const AIR_ACCELERATION: f32 = 5.0;

pub struct Body {
    pub center: PhysicsPoint,
    pub arm: ArmSkeleton,
    pub holding_to: Option<Vec2<Coord>>,
    pub ground_normal: Option<Vec2<Coord>>,
    history: running::BodyMovementHistory,
}

impl Body {
    pub fn new(position: Position) -> Self {
        Self {
            center: PhysicsPoint::new(position, Coord::new(1.0), Mass::new(30.0)),
            arm: ArmSkeleton::new(
                PhysicsPoint::new(Vec2::ZERO, Coord::new(0.2), Mass::new(0.5)),
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

        // Calculate running velocity
        let (direction, acceleration) = match self.ground_normal {
            Some(normal) => (-normal.rotate_90(), GROUND_ACCELERATION),
            None => (vec2(Coord::ONE, Coord::ZERO), AIR_ACCELERATION),
        };
        let control = control::BodyControl::from(control);
        let target_speed = control.move_speed * stats.move_speed;
        let delta_speed = target_speed - self.center.velocity.x;
        self.center.velocity +=
            direction * delta_speed.clamp_abs(Coord::new(acceleration) * delta_time);

        // Movement
        self.center.movement(delta_time);
        let relative_target = control.hand_target;
        let hold = self.holding_to.map(|pos| pos - self.center.position);
        let (impulse, release) =
            self.arm
                .control(relative_target, hold, self.center.impulse(), delta_time);
        self.center.velocity -= impulse / self.center.mass;
        if release {
            self.holding_to = None;
        } else if let Some(hold) = hold {
            let reach = self.arm.max_reach();
            if hold.len() > reach {
                self.center.position = self.holding_to.unwrap() - hold.normalize_or_zero() * reach;
            }
        }
    }
}
