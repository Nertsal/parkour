use super::*;

mod collision;

const GRAVITY: Vec2<f32> = vec2(0.0, -5.8);

pub struct Logic<'a> {
    pub model: &'a mut Model,
    pub delta_time: Time,
    pub player_control: PlayerControl,
}

impl Model {
    pub fn update(&mut self, player_control: PlayerControl, delta_time: Time) {
        let logic = Logic {
            model: self,
            delta_time,
            player_control,
        };
        logic.process();

        let height = self.player.body.center.position.y - self.player.body.center.radius;
        match &mut self.best_jump {
            None => self.best_jump = Some(height),
            Some(best) => {
                if height > *best {
                    *best = height;
                }
            }
        }
    }
}

impl<'a> Logic<'a> {
    pub fn process(mut self) {
        self.player_control();
        self.gravity();
        self.movement();
        self.collision();
    }

    fn player_control(&mut self) {
        self.model
            .player
            .shift_target(self.player_control.hand_target_delta);
        if self.player_control.jump {
            self.model.player.body.center.velocity += vec2(0.0, 5.0).map(Coord::new)
                + self.model.player.body.arm.impulse() / self.model.player.body.center.mass;
        }
    }

    fn gravity(&mut self) {
        self.model.player.body.center.velocity += GRAVITY.map(Coord::new) * self.delta_time;
    }

    fn movement(&mut self) {
        self.model.player.movement(self.delta_time);
    }
}

impl Player {
    fn shift_target(&mut self, delta: Position) {
        self.relative_target =
            (self.relative_target + delta).clamp_len(..=self.body.arm.max_reach() * r32(1.1));
    }
}
