use super::*;

const ELBOW_ACCELERATION: f32 = 40.0;
const HAND_ACCELERATION: f32 = 60.0;
const MAX_ANGULAR_VELOCITY: f32 = 15.0;
const MAX_HOLD_FORCE: f32 = 300.0;

#[derive(Debug, Clone, Copy)]
struct PolarPoint {
    pub distance: Coord,
    /// Angle to the positive direction of the x axis.
    pub angle: Angle,
}

#[derive(Debug, Clone, Copy)]
struct PolarPhysicsPoint {
    point: PolarPoint,
    /// Angular velocity.
    velocity: R32,
    radius: Coord,
    mass: Mass,
}

#[derive(Debug, Clone, Copy)]
struct Angle(R32);

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

    pub fn impulse(&self) -> Vec2<Coord> {
        let (sin, cos) = self.elbow.point.angle.0.sin_cos();
        let elbow_speed = self.elbow.velocity * self.elbow.point.distance;
        let elbow_impulse = vec2(cos, sin).rotate_90() * elbow_speed * self.elbow.mass;
        let (sin, cos) = (self.elbow.point.angle + self.hand.point.angle).0.sin_cos();
        let hand_impulse = vec2(cos, sin).rotate_90()
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
        target: Vec2<Coord>,
        hold: Option<Vec2<Coord>>,
        body_impulse: Vec2<Coord>,
        delta_time: Time,
    ) -> (Vec2<Coord>, bool) {
        let [elbow_target, hand_target] = match self.solve_angles(hold.unwrap_or(target)) {
            Some(v) => v,
            None => return (Vec2::ZERO, false),
        };

        let mut total_impulse = Vec2::ZERO;
        if hold.is_some() {
            self.elbow.point.angle = Angle::new(elbow_target);
            self.hand.point.angle = Angle::new(hand_target);
        } else {
            // Calculate target velocity
            let elbow_target = (self.elbow.point.angle.delta_to(Angle::new(elbow_target))
                * r32(5.0))
            .clamp_abs(r32(MAX_ANGULAR_VELOCITY));
            let hand_target = (self.hand.point.angle.delta_to(Angle::new(hand_target)) * r32(5.0))
                .clamp_abs(r32(MAX_ANGULAR_VELOCITY));

            // Accelerate towards target velocity
            let elbow_acc = (elbow_target - self.elbow.velocity)
                .clamp_abs(r32(ELBOW_ACCELERATION) * delta_time);
            let hand_acc =
                (hand_target - self.hand.velocity).clamp_abs(r32(HAND_ACCELERATION) * delta_time);
            self.elbow.velocity += elbow_acc;
            self.hand.velocity += hand_acc;

            // Calculate impulses
            let (sin, cos) = self.elbow.point.angle.0.sin_cos();
            let elbow_impulse = vec2(cos, sin).rotate_90() * elbow_acc * self.elbow.mass;
            let (sin, cos) = (self.elbow.point.angle + self.hand.point.angle).0.sin_cos();
            let hand_impulse = vec2(cos, sin).rotate_90() * hand_acc * self.hand.mass;

            // Move with accordance to the velocity
            self.elbow.point.angle += self.elbow.velocity * delta_time;
            self.hand.point.angle += self.hand.velocity * delta_time;

            total_impulse += (elbow_impulse + hand_impulse) * r32(1.0);
        }

        // Check hold
        let mut release = false;
        if let Some(hold) = hold {
            let mut force_left = Coord::new(MAX_HOLD_FORCE);
            if hold.len() > self.max_reach() {
                let normal = hold.normalize_or_zero();
                let impulse = Vec2::dot(-body_impulse, normal);
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

    fn solve_angles(&self, mut target: Vec2<Coord>) -> Option<[R32; 2]> {
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
        let hand_target = R32::PI
            - r32(
                ((elbow.sqr() + hand.sqr() - len.sqr()) / (r32(2.0) * elbow * hand))
                    .as_f32()
                    .clamp_abs(1.0)
                    .acos(),
            );
        let elbow_target = Angle::new(r32(((len.sqr() + elbow.sqr() - hand.sqr())
            / (r32(2.0) * elbow * len))
            .as_f32()
            .clamp_abs(1.0)
            .acos()))
        .delta_to(Angle::new(target.arg()));

        Some([elbow_target, hand_target])
    }
}

impl PolarPoint {
    pub fn to_cartesian(self) -> Vec2<Coord> {
        let (sin, cos) = self.angle.0.sin_cos();
        vec2(cos, sin) * self.distance
    }

    pub fn from_cartesian(pos: Vec2<Coord>) -> Self {
        Self {
            distance: pos.len(),
            angle: Angle::new(pos.arg()),
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

impl Angle {
    pub fn new(mut angle: R32) -> Self {
        let pi2 = R32::PI * r32(2.0);
        while angle < R32::ZERO {
            angle += pi2;
        }
        while angle > pi2 {
            angle -= pi2;
        }
        Self(angle)
    }

    pub fn delta_to(self, other: Self) -> R32 {
        let mut angle = other.0 - self.0;
        if angle > R32::PI {
            angle -= R32::PI * r32(2.0);
        } else if angle < -R32::PI {
            angle += R32::PI * r32(2.0);
        }
        angle
    }
}

impl std::ops::Add<Self> for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign<Self> for Angle {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::new(self.0 + rhs.0);
    }
}

impl std::ops::Add<R32> for Angle {
    type Output = Self;

    fn add(self, rhs: R32) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}

impl std::ops::AddAssign<R32> for Angle {
    fn add_assign(&mut self, rhs: R32) {
        *self = Self::new(self.0 + rhs);
    }
}
