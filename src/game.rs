use super::*;

const MOUSE_SENSITIVITY: f32 = 0.005;

const KEYS_MOVE_RIGHT: [geng::Key; 2] = [geng::Key::D, geng::Key::Right];
const KEYS_MOVE_LEFT: [geng::Key; 2] = [geng::Key::A, geng::Key::Left];

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    pub render: Render,
    pub model: Model,
    pub player_control: PlayerControl,
    toggle_editor: bool,
}

pub struct PlayerControl {
    pub hand_target_delta: Position,
    pub move_speed: Coord,
    pub jump: bool,
    pub hold: bool,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        geng.window().lock_cursor();
        let level = Level::load(static_path().join("new_level.json")).unwrap_or_default();
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            render: Render::new(geng, assets),
            model: Model::new(level),
            player_control: default(),
            toggle_editor: false,
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
            geng::Event::KeyDown { key } => match key {
                geng::Key::Space => self.player_control.jump = true,
                geng::Key::R => self.model.best_jump = None,
                geng::Key::T => self.toggle_editor = true,
                _ => {}
            },
            _ => {}
        }
    }

    fn update(&mut self, delta_time: f64) {
        let window = self.geng.window();
        let pressed =
            |keys: &[geng::Key]| -> bool { keys.iter().any(|&key| window.is_key_pressed(key)) };
        let mut movement = 0.0;
        if pressed(&KEYS_MOVE_RIGHT) {
            movement += 1.0;
        }
        if pressed(&KEYS_MOVE_LEFT) {
            movement -= 1.0;
        }
        self.player_control.move_speed = r32(movement);

        self.player_control.hold = window.is_button_pressed(geng::MouseButton::Left);

        self.model
            .update(self.player_control.take(), Time::new(delta_time as _));
    }

    fn transition(&mut self) -> Option<geng::Transition> {
        self.toggle_editor.then(|| {
            geng::Transition::Switch(Box::new(crate::editor::Editor::new(
                &self.geng,
                &self.assets,
            )))
        })
    }
}

impl Default for PlayerControl {
    fn default() -> Self {
        Self {
            hand_target_delta: Position::ZERO,
            move_speed: Coord::ZERO,
            jump: false,
            hold: false,
        }
    }
}

impl PlayerControl {
    pub fn take(&mut self) -> Self {
        std::mem::take(self)
    }
}
