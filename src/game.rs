use super::*;

const CAMERA_INTERPOLATION: f32 = 0.5;

const MOUSE_SENSITIVITY: f32 = 0.005;

const KEYS_MOVE_RIGHT: [geng::Key; 2] = [geng::Key::D, geng::Key::Right];
const KEYS_MOVE_LEFT: [geng::Key; 2] = [geng::Key::A, geng::Key::Left];

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    pub render: Render,
    pub model: Model,
    pub player_control: BodyControl,
    toggle_editor: bool,
    camera_target: Vec2<Coord>,
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
            camera_target: Vec2::ZERO,
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.render
            .draw(&self.model, &self.player_control, framebuffer);
    }

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseMove { delta, .. } => {
                let delta = delta.map(|x| Coord::new(x as f32 * MOUSE_SENSITIVITY));
                self.player_control.hand_target += delta;
            }
            geng::Event::KeyDown { key: geng::Key::T } => self.toggle_editor = true,
            _ => {}
        }
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;

        // Update control
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

        self.player_control.jump = pressed(&[geng::Key::Space]);

        self.player_control.hold = window.is_button_pressed(geng::MouseButton::Left);

        // Update model
        self.model
            .update(&mut self.player_control, Time::new(delta_time));

        // Update camera position
        self.camera_target = self.model.player.center.position;
        let delta = self.camera_target - self.render.camera.center.map(Coord::new);
        self.render.camera.center +=
            (delta * Coord::new(delta_time / CAMERA_INTERPOLATION)).map(Coord::as_f32);
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
