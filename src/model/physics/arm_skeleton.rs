use super::*;

const ELBOW_ACCELERATION: f32 = 40.0;
const HAND_ACCELERATION: f32 = 60.0;
const MAX_ANGULAR_VELOCITY: f32 = 15.0;
const MAX_HOLD_FORCE: f32 = 300.0;

#[derive(Debug, Clone, Copy)]
struct PolarPoint {
    pub distance: Coord,
    /// Angle to the positive direction of the x axis.
    pub angle: Angle<R32>,
}

#[derive(Debug, Clone, Copy)]
struct PolarPhysicsPoint {
    point: PolarPoint,
    /// Angular velocity.
    velocity: R32,
    radius: Coord,
    mass: Mass,
}

#[derive(Debug, Clone)]
pub struct ArmSkeleton {
    // Position of the shoulder relative to the body position.
    shoulder: PhysicsPoint,
    /// Elbow position in polar coordinates relative to the shoulder position.
    elbow: PolarPhysicsPoint,
    /// Hand position in polar coordinates relative to the elbow position.
    hand: PolarPhysicsPoint,
}

impl ArmSkeleton {
    /// Constructs an arm skeleton based on the relative positions of shoulder w.r.t. body,
    /// elbow w.r.t. shoulder, and hand w.r.t. elbow.
    pub fn new(shoulder: PhysicsPoint, elbow: PhysicsPoint, hand: PhysicsPoint) -> Self {
        Self {
            shoulder,
            elbow: elbow.into(),
            hand: hand.into(),
        }
    }

    pub fn max_reach(&self) -> Coord {
        self.elbow.point.distance + self.hand.point.distance
    }

    pub fn impulse(&self) -> vec2<Coord> {
        let elbow_dir = self.elbow.point.angle.unit_vec().rotate_90();
        let elbow_speed = self.elbow.velocity * self.elbow.point.distance;
        let elbow_impulse = elbow_dir * elbow_speed * self.elbow.mass;
        let hand_dir = (self.elbow.point.angle + self.hand.point.angle)
            .unit_vec()
            .rotate_90();
        let hand_impulse = hand_dir
            * (elbow_speed + self.hand.velocity * self.hand.point.distance)
            * self.hand.mass;
        elbow_impulse + hand_impulse
    }

    /// The returns the skeleton in world coordinates as an array `[shoulder, elbow, hand]`.
    pub fn get_skeleton(&self, body: &PhysicsPoint) -> [PhysicsPoint; 3] {
        let shoulder = self.shoulder.relative(body);
        let elbow = self.elbow.relative(&shoulder);
        let hand = PolarPhysicsPoint {
            point: PolarPoint {
                angle: self.hand.point.angle + self.elbow.point.angle,
                ..self.hand.point
            },
            ..self.hand
        }
        .relative(&elbow);
        [shoulder, elbow, hand]
    }

    /// Controls the bones and returns the total impulse used by the skeleton
    /// and whether the hold is broken.
    pub fn control(
        &mut self,
        target: vec2<Coord>,
        hold: Option<vec2<Coord>>,
        body_impulse: vec2<Coord>,
        delta_time: Time,
    ) -> (vec2<Coord>, bool) {
        let [elbow_target, hand_target] = match self.solve_angles(hold.unwrap_or(target)) {
            Some(v) => v,
            None => return (vec2::ZERO, false),
        };

        let mut total_impulse = vec2::ZERO;
        if hold.is_some() {
            self.elbow.point.angle = elbow_target;
            self.hand.point.angle = hand_target;
        } else {
            // Calculate target velocity
            let elbow_target = (self.elbow.point.angle.angle_to(elbow_target) * r32(5.0))
                .as_radians()
                .clamp_abs(r32(MAX_ANGULAR_VELOCITY));
            let hand_target = (self.hand.point.angle.angle_to(hand_target) * r32(5.0))
                .as_radians()
                .clamp_abs(r32(MAX_ANGULAR_VELOCITY));

            // Accelerate towards target velocity
            let elbow_acc = (elbow_target - self.elbow.velocity)
                .clamp_abs(r32(ELBOW_ACCELERATION) * delta_time);
            let hand_acc =
                (hand_target - self.hand.velocity).clamp_abs(r32(HAND_ACCELERATION) * delta_time);
            self.elbow.velocity += elbow_acc;
            self.hand.velocity += hand_acc;

            // Calculate impulses
            let elbow_dir = self.elbow.point.angle.unit_vec().rotate_90();
            let elbow_impulse = elbow_dir * elbow_acc * self.elbow.mass;
            let hand_dir = (self.elbow.point.angle + self.hand.point.angle)
                .unit_vec()
                .rotate_90();
            let hand_impulse = hand_dir * hand_acc * self.hand.mass;

            // Move with accordance to the velocity
            self.elbow.point.angle += Angle::from_radians(self.elbow.velocity * delta_time);
            self.hand.point.angle += Angle::from_radians(self.hand.velocity * delta_time);

            total_impulse += (elbow_impulse + hand_impulse) * r32(1.0);
        }

        // Check hold
        let mut release = false;
        if let Some(hold) = hold {
            let mut force_left = Coord::new(MAX_HOLD_FORCE);
            if hold.len() > self.max_reach() {
                let normal = hold.normalize_or_zero();
                let impulse = vec2::dot(-body_impulse, normal);
                if impulse > Coord::ZERO {
                    // Add the force required to hold on
                    force_left -= impulse;
                    if force_left < Coord::ZERO {
                        release = true;
                    } else {
                        total_impulse -= normal * impulse;
                    }
                }
            }

            if !release {
                // Pull towards target
                let delta = target - hold;
                total_impulse += delta.normalize_or_zero() * force_left * delta_time;
            }
        }

        (total_impulse, release)
    }

    fn solve_angles(&self, mut target: vec2<Coord>) -> Option<[Angle<R32>; 2]> {
        // Make sure the target is within reach
        let mut len = target.len();
        let elbow = self.elbow.point.distance;
        let hand = self.hand.point.distance;
        let max_len = elbow + hand;
        let min_len = elbow - hand;
        if len.approx_eq(&R32::ZERO) {
            // safety check
            return None;
        } else if len > max_len {
            target = target / len * max_len;
            len = max_len;
        } else if len < min_len {
            target = target / len * min_len;
            len = min_len;
        }

        // Find the target angles
        let hand_target = Angle::from_radians(
            R32::PI
                - r32(
                    ((elbow.sqr() + hand.sqr() - len.sqr()) / (r32(2.0) * elbow * hand))
                        .as_f32()
                        .clamp_abs(1.0)
                        .acos(),
                ),
        );
        let elbow_target = Angle::from_radians(r32(((len.sqr() + elbow.sqr() - hand.sqr())
            / (r32(2.0) * elbow * len))
            .as_f32()
            .clamp_abs(1.0)
            .acos()))
        .angle_to(target.arg());

        Some([elbow_target, hand_target])
    }
}

impl PolarPoint {
    pub fn to_cartesian(self) -> vec2<Coord> {
        self.angle.unit_vec() * self.distance
    }

    pub fn from_cartesian(pos: vec2<Coord>) -> Self {
        Self {
            distance: pos.len(),
            angle: pos.arg(),
        }
    }
}

impl PolarPhysicsPoint {
    pub fn relative(self, point: &PhysicsPoint) -> PhysicsPoint {
        PhysicsPoint {
            position: self.point.to_cartesian() + point.position,
            velocity: point.velocity,
            radius: self.radius,
            mass: self.mass,
        }
    }
}

impl From<PhysicsPoint> for PolarPhysicsPoint {
    fn from(point: PhysicsPoint) -> Self {
        Self {
            point: PolarPoint::from_cartesian(point.position),
            velocity: R32::ZERO,
            radius: point.radius,
            mass: point.mass,
        }
    }
}
