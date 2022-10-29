use super::*;

const ELBOW_ACCELERATION: f32 = 20.0;
const HAND_ACCELERATION: f32 = 10.0;
const MAX_ANGULAR_VELOCITY: f32 = 10.0;

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

    pub fn move_hand_towards(&mut self, mut target: Vec2<Coord>, delta_time: Time) {
        // Make sure the target is within reach
        let mut len = target.len();
        let elbow = self.elbow.point.distance;
        let hand = self.hand.point.distance;
        let max_len = elbow + hand;
        let min_len = elbow - hand;
        if len.approx_eq(&R32::ZERO) {
            // safety check
            return;
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

        // Calculate target velocity
        let elbow_target = (self.elbow.point.angle.delta_to(Angle::new(elbow_target)) * r32(5.0))
            .clamp_abs(r32(MAX_ANGULAR_VELOCITY));
        let hand_target = (self.hand.point.angle.delta_to(Angle::new(hand_target)) * r32(5.0))
            .clamp_abs(r32(MAX_ANGULAR_VELOCITY));

        // Accelerate towards target velocity
        self.elbow.velocity +=
            (elbow_target - self.elbow.velocity).clamp_abs(r32(ELBOW_ACCELERATION) * delta_time);
        self.hand.velocity +=
            (hand_target - self.hand.velocity).clamp_abs(r32(HAND_ACCELERATION) * delta_time);

        // Move with accordance to the velocity
        self.elbow.point.angle += self.elbow.velocity * delta_time;
        self.hand.point.angle += self.hand.velocity * delta_time;
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
