use super::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Surface {
    pub p1: vec2<Coord>,
    pub p2: vec2<Coord>,
}

impl Surface {
    pub fn collider(&self) -> Collider {
        Collider::new_segment(vec2::ZERO, self.p1, self.p2)
    }

    pub fn segment_f32(&self) -> Segment<f32> {
        Segment(self.p1.map(Coord::as_f32), self.p2.map(Coord::as_f32))
    }

    pub fn delta_to(&self, point: vec2<Coord>) -> vec2<Coord> {
        if vec2::dot(point - self.p1, self.p2 - self.p1) < Coord::ZERO {
            return self.p1 - point;
        }
        if vec2::dot(point - self.p2, self.p1 - self.p2) < Coord::ZERO {
            return self.p2 - point;
        }
        let normal = (self.p2 - self.p1).rotate_90();
        let penetration = vec2::dot(self.p1 - point, normal) / vec2::dot(normal, normal);
        normal * penetration
    }
}
