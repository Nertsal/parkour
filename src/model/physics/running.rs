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
    frequency: R32,
    amplitude: Coord,
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
            frequency: R32::ZERO,
            amplitude: Coord::ZERO,
            hand: Position::ZERO,
        };

        let mut last_side = 0.0;
        let mut last_amp = Coord::ZERO;

        let mut swings = Vec::new();
        let mut total_amp = R32::ZERO;

        for state in &self.states {
            let side = state.hand.x.signum().as_f32();
            let amp = state.hand.x.abs();
            if last_side != side {
                swings.push(last_amp);
                total_amp += last_amp;
                last_side = side;
            } else if amp > last_amp {
                last_amp = amp;
            }
        }

        let start = self.states.front().map_or(0.0, |s| s.time.as_f32());
        let end = self.states.back().map_or(0.0, |s| s.time.as_f32());
        info.frequency = r32(swings.len() as f32 / (end - start).max(0.01));
        info.amplitude = total_amp / r32(swings.len().max(1) as f32);

        info
    }
}

impl BodyMovementInfo {
    pub fn calc_stats(&self) -> BodyMovementStats {
        let amplitude = self.amplitude;
        let frequency = (self.frequency / r32(7.0)).min(r32(1.0));
        let rhythm = Time::ONE; // / (Time::ONE + (self.positive_time - self.negative_time).abs());
        let max_amplitude = Coord::new(MAX_AMPLITUDE);
        let t = (amplitude * rhythm * frequency).clamp(Coord::ZERO, max_amplitude) / max_amplitude;
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
