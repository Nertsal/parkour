use super::*;
use crate::physics::Body;

#[derive(Debug, Clone, Copy)]
pub struct BodyControl {
    /// Target position for the hand relative to the body.
    pub hand_target: Position,
    /// Expected to be in the range -1.0..=1.0 (clamped to the range if not)
    /// where positive direction is right and negative is left.
    pub move_speed: Coord,
    /// Target height of the body above feet. Expected to be in the range 0.0..=1.0
    /// (clamped to the range if not), where 0 is sitting on the ground
    /// (or tucking) and 1 is fully extended.
    pub target_height: Coord,
    /// Whether hands are trying to hold onto an object.
    pub hold: bool,
    pub jump: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct VerifiedBodyControl(BodyControl);

impl Default for BodyControl {
    fn default() -> Self {
        Self {
            hand_target: Position::ZERO,
            move_speed: Coord::ZERO,
            target_height: Coord::ZERO,
            hold: false,
            jump: false,
        }
    }
}

impl From<VerifiedBodyControl> for BodyControl {
    fn from(control: VerifiedBodyControl) -> Self {
        control.0
    }
}

impl BodyControl {
    pub fn verify(mut self, body: &Body) -> VerifiedBodyControl {
        self.hand_target = self
            .hand_target
            .clamp_len(..=body.arm.max_reach() * r32(1.1));
        self.move_speed = self.move_speed.clamp_range(-Coord::ONE..=Coord::ONE);
        self.target_height = self.target_height.clamp_range(Coord::ZERO..=Coord::ONE);
        VerifiedBodyControl(self)
    }
}
