use super::*;

pub struct Body {
    pub collider: Collider,
    pub velocity: vec2<Coord>,
    pub ground_normal: Option<vec2<Coord>>,
    pub history: BodyMovementHistory,
    pub push_point: Option<PushPoint>,
}

#[derive(Debug, Clone)]
pub struct PushPoint {
    /// Relative position of the point being pushed.
    pub position: Position,
    /// Whether something got grabbed at the position.
    pub grabbed: bool,
}

impl Body {
    pub fn new(position: Position) -> Self {
        Self {
            collider: Collider::new_aabb(
                Aabb2::point(position).extend_symmetric((vec2(0.5, 1.2) / 2.0).as_r32()),
            ),
            velocity: vec2::ZERO,
            ground_normal: None,
            history: BodyMovementHistory::default(),
            push_point: None,
        }
    }

    pub fn max_reach(&self) -> Coord {
        r32(1.5)
    }

    pub fn movement(&mut self, delta_time: Time) {
        self.collider.position += self.velocity * delta_time;
    }
}
