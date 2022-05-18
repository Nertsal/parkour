use super::*;

impl Logic<'_> {
    pub fn collision(&mut self) {
        let point = &mut self.model.player.body.center;
        if point.position.y < Coord::ZERO {
            point.position.y = Coord::ZERO;
            point.velocity = Vec2::ZERO;
        }
    }
}
