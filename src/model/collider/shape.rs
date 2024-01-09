use super::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Shape {
    Segment { a: Position, b: Position },
    Circle { radius: Coord },
    Rectangle { width: Coord, height: Coord },
}

impl Shape {
    pub fn to_parry(self) -> Box<dyn parry2d::shape::Shape> {
        match self {
            Shape::Segment { a, b } => {
                let a = to_point(a.as_f32());
                let b = to_point(b.as_f32());
                Box::new(parry2d::shape::Segment::new(a, b))
            }
            Shape::Circle { radius } => Box::new(parry2d::shape::Ball::new(radius.as_f32())),
            Shape::Rectangle { width, height } => {
                let aabb = Aabb2::ZERO.extend_symmetric(vec2(width, height).as_f32() / 2.0);
                let points = aabb.corners().map(to_point);
                // All rectangles are convex
                Box::new(parry2d::shape::ConvexPolygon::from_convex_hull(&points).unwrap())
            }
        }
    }
}

fn to_point(p: vec2<f32>) -> parry2d::math::Point<parry2d::math::Real> {
    let vec2(x, y) = p;
    parry2d::math::Point::new(x, y)
}
