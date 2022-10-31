use super::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Surface {
    pub p1: Vec2<Coord>,
    pub p2: Vec2<Coord>,
}

impl Surface {
    pub fn segment_f32(&self) -> Segment<f32> {
        Segment::new(self.p1.map(Coord::as_f32), self.p2.map(Coord::as_f32))
    }

    pub fn delta_to(&self, point: Vec2<Coord>) -> Vec2<Coord> {
        if Vec2::dot(point - self.p1, self.p2 - self.p1) < Coord::ZERO {
            return self.p1 - point;
        }
        if Vec2::dot(point - self.p2, self.p1 - self.p2) < Coord::ZERO {
            return self.p2 - point;
        }
        let normal = (self.p2 - self.p1).rotate_90();
        let penetration = Vec2::dot(self.p1 - point, normal) / Vec2::dot(normal, normal);
        normal * penetration
    }
}
