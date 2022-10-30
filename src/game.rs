use super::*;

const MOUSE_SENSITIVITY: f32 = 0.001;

pub struct Game {
    pub render: Render,
    pub model: Model,
    pub player_control: PlayerControl,
}

pub struct PlayerControl {
    pub hand_target_delta: Position,
    pub jump: bool,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            render: Render::new(geng, assets),
            model: Model::new(),
            player_control: default(),
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.render.draw(&self.model, framebuffer);
    }

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseMove { delta, .. } => {
                let delta = delta.map(|x| Coord::new(x as f32 * MOUSE_SENSITIVITY));
                self.player_control.hand_target_delta += delta;
            }
            geng::Event::KeyDown {
                key: geng::Key::Space,
            } => {
                self.player_control.jump = true;
            }
            _ => {}
        }
    }

    fn update(&mut self, delta_time: f64) {
        self.model
            .update(self.player_control.take(), Time::new(delta_time as _));
    }
}

impl Default for PlayerControl {
    fn default() -> Self {
        Self {
            hand_target_delta: Position::ZERO,
            jump: false,
        }
    }
}

impl PlayerControl {
    pub fn take(&mut self) -> Self {
        std::mem::take(self)
    }
}
