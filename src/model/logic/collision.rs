use super::*;

const GROUND_ANGLE: f32 = 0.5;

impl Logic<'_> {
    pub fn collisions(&mut self) {
        self.model.player.collide(&self.model.level.surfaces);
    }
}

impl Body {
    fn collide(&mut self, surfaces: &[Surface]) {
        // Reset ground
        self.ground_normal = None;

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
            self.collider
                .collide(&Collider::new_segment(vec2::ZERO, surface.p1, surface.p2))
        })
    }

    fn resolve_collision(&mut self, collision: Collision) {
        self.collider.position -= collision.normal * collision.penetration;
        let normal_vel = vec2::dot(self.velocity, collision.normal);
        self.velocity -= collision.normal * normal_vel;

        // Check for grounded
        let angle = collision.normal.arg() + Angle::from_degrees(r32(90.0));
        self.ground_normal =
            (angle.as_radians().abs().as_f32() < GROUND_ANGLE).then_some(collision.normal);
    }
}
