use geng::prelude::itertools::Itertools;

use super::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Collider {
    Aabb(AABB<Coord>),
    Surface(Surface),
}

#[derive(Debug, Clone, Copy)]
pub struct Collision {
    pub normal: Vec2<Coord>,
    pub penetration: Coord,
}

impl Collider {
    pub fn square(side: Coord) -> Self {
        Self::Aabb(AABB::ZERO.extend_uniform(side / Coord::new(2.0)))
    }

    pub fn rectangle(width: Coord, height: Coord) -> Self {
        Self::Aabb(AABB::ZERO.extend_symmetric(vec2(width, height) / Coord::new(2.0)))
    }

    pub fn check_collision(&self, other: &Self, other_offset: Vec2<Coord>) -> Option<Collision> {
        let mut collision = None;
        for axis in get_critical_axes(self, other) {
            let axis = axis.normalize_or_zero();
            let self_proj = self.project(axis, Vec2::ZERO);
            let other_proj = other.project(axis, other_offset);
            let penetration = self_proj.get_intersection(other_proj)?;
            let normal = -axis;
            let col = Collision {
                normal: normal * penetration.signum(),
                penetration: penetration.abs(),
            };
            collision = Some(match collision.take() {
                Some(old) => std::cmp::min_by_key(old, col, |col| col.penetration),
                None => col,
            });
        }
        collision
    }

    pub fn contains(&self, point: Vec2<Coord>, collider_pos: Vec2<Coord>) -> bool {
        self.check_collision(&Self::Aabb(AABB::ZERO), point - collider_pos)
            .is_some()
    }

    pub fn project(&self, axis: Vec2<Coord>, offset: Vec2<Coord>) -> Projection {
        let points = self.points().into_iter().map(|x| x + offset);
        project_points(points, axis).expect("Colliders cannot have 0 points")
    }

    fn points(&self) -> Vec<Vec2<Coord>> {
        match self {
            Self::Aabb(aabb) => aabb.corners().to_vec(),
            Self::Surface(surface) => vec![surface.p1, surface.p2],
        }
    }

    fn critical_axes(&self) -> Vec<Vec2<Coord>> {
        match self {
            Self::Aabb(_) => {
                vec![vec2(Coord::ONE, Coord::ZERO), vec2(Coord::ZERO, Coord::ONE)]
            }
            Self::Surface(surface) => {
                vec![surface.direction(), surface.normal()]
            } // _ => {
              //     let points = self.points();
              //     points
              //         .iter()
              //         .copied()
              //         .zip(points.iter().skip(1).copied())
              //         .map(|(x, y)| y - x)
              //         .collect()
              // }
        }
    }
}

fn get_critical_axes<'a>(
    this: &'a Collider,
    other: &'a Collider,
) -> impl Iterator<Item = Vec2<Coord>> + 'a {
    let other_points = other.points();
    let deltas: Vec<_> = this
        .points()
        .iter()
        .copied()
        .flat_map(|x| other_points.iter().copied().map(move |y| y - x))
        .collect();
    this.critical_axes()
        .into_iter()
        .chain(other.critical_axes())
        .chain(deltas)
}

fn project_points(
    points: impl IntoIterator<Item = Vec2<Coord>>,
    axis: Vec2<Coord>,
) -> Option<Projection> {
    let axis = axis.normalize_or_zero();
    let (a, b) = points
        .into_iter()
        .map(|point| Vec2::dot(point, axis))
        .minmax()
        .into_option()?;
    Some(Projection::new(a, b))
}

#[derive(Debug, Clone, Copy)]
pub struct Projection {
    pub a: Coord,
    pub b: Coord,
}

impl Projection {
    pub fn new(a: Coord, b: Coord) -> Self {
        Self { a, b }
    }

    pub fn get_intersection(self, other: Self) -> Option<Coord> {
        let da = self.b - other.a;
        let db = other.b - self.a;
        if da < db {
            (da > Coord::ZERO).then_some(da)
        } else {
            (db > Coord::ZERO).then_some(-db)
        }
    }
}
