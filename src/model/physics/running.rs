use std::collections::VecDeque;

use super::*;

const RECORD_LENGTH: f32 = 2.0;
const MAX_AMPLITUDE: f32 = 0.8;

const WALKING_SPEED: f32 = 2.0;
const MAX_RUNNING_SPEED: f32 = 6.0;

#[derive(Debug, Clone)]
pub struct BodyMovementHistory {
    pub time: Time,
    states: VecDeque<BodyMovementState>,
}

#[derive(Debug, Clone)]
pub struct BodyMovementState {
    pub time: Time,
    pub hand: Position,
}

#[derive(Debug, Clone)]
pub struct BodyMovementInfo {
    positive_time: Time,
    negative_time: Time,
    pos_bounds: Aabb2<Coord>,
    hand: Position,
}

#[derive(Debug, Clone)]
pub struct BodyMovementStats {
    pub move_speed: Coord,
}

impl BodyMovementHistory {
    pub fn update(&mut self, mut state: BodyMovementState, delta_time: Time) {
        self.time += delta_time;
        state.time = self.time;
        let limit = self.time - Time::new(RECORD_LENGTH);
        while let Some(state) = self.states.front() {
            if state.time > limit {
                break;
            }
            self.states.pop_front();
        }
        self.states.push_back(state);
    }

    pub fn analyze(&self) -> BodyMovementInfo {
        let mut info = BodyMovementInfo {
            positive_time: Time::ZERO,
            negative_time: Time::ZERO,
            pos_bounds: Aabb2::ZERO,
            hand: Position::ZERO,
        };
        let mut states = self.states.iter();
        if let Some(state) = states.next() {
            let mut last_time = state.time;
            info.pos_bounds = Aabb2::point(state.hand);
            for state in std::iter::once(state).chain(states) {
                let positive = state.hand.x >= Coord::ZERO;
                let time_record = if positive {
                    &mut info.positive_time
                } else {
                    &mut info.negative_time
                };
                *time_record += state.time - last_time;
                last_time = state.time;
                info.hand = state.hand;
                info.pos_bounds.min.x = info.pos_bounds.min.x.min(state.hand.x);
                info.pos_bounds.max.x = info.pos_bounds.max.x.max(state.hand.x);
                info.pos_bounds.min.y = info.pos_bounds.min.y.min(state.hand.y);
                info.pos_bounds.max.y = info.pos_bounds.max.y.max(state.hand.y);
            }
        }
        info
    }
}

impl BodyMovementInfo {
    pub fn calc_stats(&self) -> BodyMovementStats {
        let amplitude = self.pos_bounds.width() / Coord::new(2.0); // Normalize
        let rhythm = Time::ONE; // / (Time::ONE + (self.positive_time - self.negative_time).abs());
        let max_amplitude = Coord::new(MAX_AMPLITUDE);
        let t = (amplitude * rhythm).clamp(Coord::ZERO, max_amplitude) / max_amplitude;
        let move_speed =
            Coord::new(WALKING_SPEED) + Coord::new(MAX_RUNNING_SPEED - WALKING_SPEED) * t;
        BodyMovementStats { move_speed }
    }
}

impl Default for BodyMovementHistory {
    fn default() -> Self {
        Self {
            time: Time::ZERO,
            states: default(),
        }
    }
}
