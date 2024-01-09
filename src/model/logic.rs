mod collision;

use super::*;

use crate::control::BodyControl;

const GRAVITY: vec2<f32> = vec2(0.0, -9.8);

const GROUND_ACCELERATION: f32 = 30.0;
const GROUND_DECELERATION: f32 = 50.0;
const AIR_ACCELERATION: f32 = 5.0;

const HAND_RADIUS: f32 = 0.2;
const PUSH_STRENGTH: f32 = 20.0;

pub struct Logic<'a> {
    pub model: &'a mut Model,
    pub delta_time: Time,
    pub player_control: &'a mut BodyControl,
}

impl Model {
    pub fn update(&mut self, player_control: &mut BodyControl, delta_time: Time) {
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
        self.gravity();
        self.player_control();
        self.collisions();
    }

    fn player_control(&mut self) {
        let control = self.player_control.verify(&self.model.player);
        let control = BodyControl::from(control);
        *self.player_control = control;

        if self.player_control.push {
            self.player_pushing();
        } else {
            self.model.player.push_point = None;
        }

        let player = &mut self.model.player;

        // if self.player_control.hold {
        //     player.try_holding(&self.model.level.surfaces);
        // } else {
        //     player.holding_to = None;
        // }

        // Record
        let state = BodyMovementState {
            time: player.history.time,
            hand: control.hand_target / player.max_reach(), // Normalized
        };
        player.history.update(state, self.delta_time);
        let info = player.history.analyze();
        let stats = info.calc_stats();

        // Calculate running velocity
        let (direction, acceleration, deceleration) = match player.ground_normal {
            Some(normal) => (normal.rotate_90(), GROUND_ACCELERATION, GROUND_DECELERATION),
            None => (
                vec2(Coord::ONE, Coord::ZERO),
                AIR_ACCELERATION,
                AIR_ACCELERATION,
            ),
        };
        let target_speed = control.move_speed * stats.move_speed;
        let delta_speed = target_speed - player.velocity.x;
        let acc = if (player.velocity.x * delta_speed).as_f32() > 0.0 {
            acceleration
        } else {
            deceleration
        };
        player.velocity += direction * delta_speed.clamp_abs(Coord::new(acc) * self.delta_time);

        // // Jump
        // if let Some(normal) = player.ground_normal.filter(|_| control.jump) {
        //     player.center.velocity +=
        //         normal * r32(5.0) + player.arm.impulse() * r32(4.0) / player.center.mass;
        // }

        // Movement
        player.movement(self.delta_time);

        // let relative_target = control.hand_target;
        // let hold = player.holding_to.map(|pos| pos - player.center.position);
        // let (impulse, release) = player.arm.control(
        //     relative_target,
        //     hold,
        //     player.center.impulse(),
        //     self.delta_time,
        // );
        // player.center.velocity -= impulse / player.center.mass;

        // if release {
        //     player.holding_to = None;
        // } else if let Some(hold) = hold {
        //     let reach = player.arm.max_reach();
        //     if hold.len() > reach {
        //         player.center.position =
        //             player.holding_to.unwrap() - hold.normalize_or_zero() * reach;
        //     }
        // }
    }

    fn player_pushing(&mut self) {
        let hand_target = self
            .player_control
            .hand_target
            .clamp_len(..=self.model.player.max_reach());

        if self.model.player.push_point.is_none() {
            self.model.player.push_point = Some(PushPoint {
                position: hand_target,
                grabbed: false,
            });
        }
        let last = self.model.player.push_point.as_mut().unwrap();

        if !last.grabbed {
            last.position = hand_target;
        }

        let pos = self.model.player.collider.position + last.position;
        let current = Collider::new_aabb(Aabb2::point(pos).extend_uniform(r32(HAND_RADIUS)));

        // TODO: continuous collision detection
        let can_grab = self
            .model
            .level
            .surfaces
            .iter()
            .any(|s| current.check(&s.collider()));

        if !last.grabbed {
            if can_grab {
                // Grab
                last.grabbed = true;
            }
        } else if !can_grab {
            // Lost grip
            last.grabbed = false;
        }

        if last.grabbed {
            // Push
            let push_dir = self.player_control.hand_target - last.position;
            if vec2::dot(push_dir, last.position).as_f32() > 0.0 {
                // Push, don't pull
                let push_dir = push_dir.normalize_or_zero();
                let strength = r32(PUSH_STRENGTH);
                self.model.player.velocity -= push_dir * strength * self.delta_time;
            }
        }
    }

    fn gravity(&mut self) {
        self.model.player.velocity += GRAVITY.as_r32() * self.delta_time;
    }
}
