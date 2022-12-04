use super::*;
use crate::physics::*;

const GROUND_ANGLE: f32 = 0.5;

impl Logic<'_> {
    pub fn collisions(&mut self) {
        let i = self.model.player.collide(&self.model.level.surfaces);
        self.model.surface_collision = i;
    }
}

impl Body {
    fn collide(&mut self, surfaces: &[Surface]) -> Option<usize> {
        // Reset ground
        self.ground_normal = None;

        // Find the appropriate collision
        let collision = self
            .get_collisions(surfaces)
            .max_by_key(|(_, collision)| collision.penetration);

        // Resolve the collision
        if let Some((_, collision)) = collision {
            self.resolve_collision(collision);
        }

        collision.map(|(i, _)| i)
    }

    fn get_collisions<'a>(
        &'a self,
        surfaces: impl IntoIterator<Item = &'a Surface> + 'a,
    ) -> impl Iterator<Item = (usize, Collision)> + 'a {
        surfaces.into_iter().enumerate().filter_map(|(i, surface)| {
            self.center
                .collider
                .check_collision(&Collider::Surface(*surface), -self.center.position)
                .filter(|collision| Vec2::dot(collision.normal, self.center.velocity) < Coord::ZERO)
                .map(|col| (i, col))
        })
    }

    fn resolve_collision(&mut self, collision: Collision) {
        self.center.position += collision.normal * collision.penetration;
        let normal_vel = Vec2::dot(self.center.velocity, collision.normal);
        self.center.velocity -= collision.normal * normal_vel;

        // Check for grounded
        let angle = collision.normal.arg() - R32::PI / r32(2.0);
        self.ground_normal = (angle.abs().as_f32() < GROUND_ANGLE).then_some(collision.normal);
    }
}
