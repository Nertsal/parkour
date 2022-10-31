use super::*;

const MOUSE_SENSITIVITY: f32 = 0.01;

const MOVE_RIGHT: [geng::Key; 2] = [geng::Key::D, geng::Key::Right];
const MOVE_LEFT: [geng::Key; 2] = [geng::Key::A, geng::Key::Left];

pub struct Game {
    geng: Geng,
    pub render: Render,
    pub model: Model,
    pub player_control: PlayerControl,
}

pub struct PlayerControl {
    pub hand_target_delta: Position,
    pub jump: bool,
    pub move_speed: Coord,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
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
            geng::Event::KeyDown { key } => match key {
                geng::Key::Space => self.player_control.jump = true,
                geng::Key::R => self.model.best_jump = None,
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
        if pressed(&MOVE_RIGHT) {
            movement += 1.0;
        }
        if pressed(&MOVE_LEFT) {
            movement -= 1.0;
        }
        self.player_control.move_speed = r32(movement);

        self.model
            .update(self.player_control.take(), Time::new(delta_time as _));
    }
}

impl Default for PlayerControl {
    fn default() -> Self {
        Self {
            hand_target_delta: Position::ZERO,
            move_speed: Coord::ZERO,
            jump: false,
        }
    }
}

impl PlayerControl {
    pub fn take(&mut self) -> Self {
        std::mem::take(self)
    }
}
