use super::*;

use physics::*;

const MAX_HAND_SPEED: f32 = 20.0;

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
        self.movement();
    }

    fn player_control(&mut self) {
        self.model
            .player
            .shift_target(self.player_control.hand_target_delta);
    }

    fn movement(&mut self) {
        self.model
            .player
            .body
            .shift_hand(self.player_control.hand_target_delta, self.delta_time);
    }
}

impl Player {
    fn shift_target(&mut self, delta: Position) {
        self.hand_target_delta += delta;
    }
}

impl Body {
    fn shift_hand(&mut self, delta: Position, delta_time: Time) {
        let max_speed = Coord::new(MAX_HAND_SPEED) * delta_time;
        let delta = delta.clamp_len(..=max_speed);
        let relative_target = self.hand.position + delta - self.center.position;
        let relative_target = relative_target.clamp_len(..=self.hand_length);
        self.hand.position = self.center.position + relative_target;
    }
}
