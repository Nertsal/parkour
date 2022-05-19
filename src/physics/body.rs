use super::*;

const MAX_HAND_SPEED: f32 = 20.0;

pub struct Body {
    pub center: PhysicsPoint,
    pub relative_hand: PhysicsPoint,
    pub hand_length: Coord,
}

impl Body {
    pub fn new(position: Position) -> Self {
        Self {
            center: PhysicsPoint::new(position, Coord::new(1.0), Mass::new(5.0)),
            relative_hand: PhysicsPoint::new(position, Coord::new(0.5), Mass::new(1.0)),
            hand_length: Coord::new(2.0),
        }
    }

    pub fn absolute_hand(&self) -> PhysicsPoint {
        self.relative_hand.relative(&self.center)
    }

    pub fn move_hand_towards(&mut self, relative_target: Position, delta_time: Time) {
        let old_velocity = self.relative_hand.velocity;
        let mut velocity = relative_target - self.relative_hand.position;
        let len = velocity.len();
        let max_speed = Coord::new(MAX_HAND_SPEED);
        if len <= max_speed * delta_time {
            // One-frame move
            velocity /= delta_time;
        } else {
            velocity *= max_speed / len;
        }
        self.relative_hand.velocity = velocity;

        self.center.velocity -=
            (velocity - old_velocity) * self.relative_hand.mass / self.center.mass;

        let delta = self.relative_hand.velocity * delta_time;
        self.relative_hand.position =
            (self.relative_hand.position + delta).clamp_len(..=self.hand_length);
    }
}
