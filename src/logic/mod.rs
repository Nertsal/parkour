use super::*;

mod collision;

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

    fn player_control(&mut self) {
        self.model
            .player
            .shift_target(self.player_control.hand_target_delta);
    }

    fn gravity(&mut self) {
        self.model.player.body.center.velocity += GRAVITY.map(|x| Coord::new(x)) * self.delta_time;
    }

    fn movement(&mut self) {
        self.model.player.move_hand(self.delta_time);
        self.model.player.body.center.movement(self.delta_time);
    }
}

impl Player {
    fn shift_target(&mut self, delta: Position) {
        let max_distance = self.body.hand_length;
        self.relative_target = (self.relative_target + delta).clamp_len(..=max_distance);
    }

    fn move_hand(&mut self, delta_time: Time) {
        self.body
            .move_hand_towards(self.relative_target, delta_time);
    }
}
