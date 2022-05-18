use super::*;

mod collision;

use physics::*;

const MAX_HAND_SPEED: f32 = 20.0;
const GRAVITY: Vec2<f32> = vec2(0.0, -9.8);

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
    }
}

impl<'a> Logic<'a> {
    pub fn process(mut self) {
        self.player_control();
        self.gravity();
        self.movement();
        self.collision();
    }

    fn player_control(&mut self) {}

    fn gravity(&mut self) {
        self.model.player.body.center.velocity += GRAVITY.map(|x| Coord::new(x)) * self.delta_time;
    }

    fn movement(&mut self) {
        self.model
            .player
            .body
            .shift_hand(self.player_control.hand_target_delta, self.delta_time);
    }
}

impl Body {
    fn shift_hand(&mut self, delta: Position, delta_time: Time) {
        let max_speed = Coord::new(MAX_HAND_SPEED) * delta_time;
        let delta = delta.clamp_len(..=max_speed);
        let target = self.relative_hand.position + delta;
        let target = target.clamp_len(..=self.hand_length);
        self.relative_hand.position = target;
    }
}
