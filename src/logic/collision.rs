use super::*;
use crate::physics::*;

impl Logic<'_> {
    pub fn collisions(&mut self) {
        self.model.player.body.collide(&self.model.level.surfaces);
    }
}

#[derive(Debug, Clone, Copy)]
struct Collision {
    pub normal: Vec2<Coord>,
    pub penetration: Coord,
}

impl Body {
    fn collide(&mut self, surfaces: &[Surface]) {
        // Find the appropriate collision
        let collision = self
            .get_collisions(surfaces)
            .max_by_key(|collision| collision.penetration);

        // Resolve the collision
        if let Some(collision) = collision {
            self.resolve_collision(collision);
        }
    }

    fn get_collisions<'a>(
        &'a self,
        surfaces: impl IntoIterator<Item = &'a Surface> + 'a,
    ) -> impl Iterator<Item = Collision> + 'a {
        surfaces.into_iter().filter_map(|surface| {
            let delta = surface.delta_to(self.center.position);
            let penetration = self.center.radius - delta.len();
            (penetration > Coord::ZERO && Vec2::dot(delta, self.center.velocity) > Coord::ZERO)
                .then(|| Collision {
                    normal: -delta.normalize_or_zero(),
                    penetration,
                })
        })
    }

    fn resolve_collision(&mut self, collision: Collision) {
        self.center.position += collision.normal * collision.penetration;
        let normal_vel = Vec2::dot(self.center.velocity, collision.normal);
        self.center.velocity -= collision.normal * normal_vel;
    }
}
