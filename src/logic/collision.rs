use super::*;

impl Logic<'_> {
    pub fn collision(&mut self) {
        self.model
            .player
            .body
            .center
            .position
            .y
            .clamp_range(Coord::new(0.0)..);
    }
}
