use super::*;

const MAX_HAND_SPEED: f32 = 20.0;
const MAX_HAND_ACCELERATION: f32 = 500.0;

pub struct Body {
    pub center: PhysicsPoint,
    pub relative_hand: PhysicsPoint,
    pub hand_length: Coord,
}

impl Body {
    pub fn new(position: Position) -> Self {
        Self {
            center: PhysicsPoint::new(position, Coord::new(1.0), Mass::new(2.0)),
            relative_hand: PhysicsPoint::new(position, Coord::new(0.5), Mass::new(1.0)),
            hand_length: Coord::new(2.0),
        }
    }

    pub fn absolute_hand(&self) -> PhysicsPoint {
        self.relative_hand.relative(&self.center)
    }

    pub fn move_hand_towards(&mut self, relative_target: Position, delta_time: Time) {
        {
            // Acceleration
            let max_acceleration = Coord::new(MAX_HAND_ACCELERATION) * delta_time;
            let mut target_velocity = relative_target - self.relative_hand.position;
            let len = target_velocity.len();
            if Coord::new(MAX_HAND_SPEED) * delta_time <= len && len <= Coord::new(MAX_HAND_SPEED) {
                target_velocity = target_velocity / len * Coord::new(MAX_HAND_SPEED);
            }
            let delta = target_velocity - self.relative_hand.velocity;
            self.relative_hand.velocity += delta.clamp_len(..=max_acceleration);
        }
        {
            // Movement
            let delta = self.relative_hand.velocity * delta_time;
            let mut target = self.relative_hand.position + delta;
            let len = target.len();
            if len > self.hand_length {
                // Stop hand
                target = target / len * self.hand_length;
                self.center.velocity += self.relative_hand.impulse() / self.center.mass;
                self.relative_hand.velocity = Velocity::ZERO;
            }
            self.relative_hand.position = target;
        }
    }
}
