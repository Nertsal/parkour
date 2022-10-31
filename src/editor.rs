use crate::physics::Surface;

use super::*;

const AUTOSAVE_PERIOD: f32 = 10.0;
const SNAP_DISTANCE: f32 = 0.5;
const HOVER_DISTANCE: f32 = 0.5;

pub struct Editor {
    geng: Geng,
    assets: Rc<Assets>,
    render: Render,
    framebuffer_size: Vec2<f32>,
    next_autosave: f32,
    mouse_drag: Option<MouseDrag>,
    level: Level,
    play: bool,
}

struct MouseDrag {
    pub start_camera: Vec2<f32>,
    pub start: Vec2<Coord>,
    pub button: geng::MouseButton,
}

impl Editor {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        geng.window().unlock_cursor();
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            render: Render::new(geng, assets),
            framebuffer_size: vec2(1.0, 1.0),
            next_autosave: 0.0,
            mouse_drag: None,
            level: Level::load(static_path().join("new_level.json")).unwrap_or_default(),
            play: false,
        }
    }

    fn find_hovered_surface(&self, position: Vec2<Coord>) -> Option<usize> {
        self.level
            .surfaces
            .iter()
            .position(|surface| surface.delta_to(position).len().as_f32() <= HOVER_DISTANCE)
    }

    fn snap_position(&self, position: Vec2<Coord>) -> Vec2<Coord> {
        self.level
            .surfaces
            .iter()
            .flat_map(|surface| [surface.p1, surface.p2])
            .map(|p| (p, (p - position).len()))
            .filter(|(_, dist)| dist.as_f32() < SNAP_DISTANCE)
            .min_by_key(|(_, dist)| *dist)
            .map(|(p, _)| p)
            .unwrap_or(position)
    }

    pub fn save_level(&self) {
        self.level.save(static_path().join("new_level.json"));
    }
}

impl geng::State for Editor {
    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;
        self.next_autosave -= delta_time;
        if self.next_autosave < 0.0 {
            self.next_autosave = AUTOSAVE_PERIOD;
            self.save_level();
        }
    }

    fn handle_event(&mut self, event: geng::Event) {
        let window = self.geng.window();
        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::P => self.play = true,
                geng::Key::S if window.is_key_pressed(geng::Key::LCtrl) => {
                    self.next_autosave = AUTOSAVE_PERIOD;
                    self.save_level()
                }
                geng::Key::R => {
                    if window.is_key_pressed(geng::Key::LCtrl) {
                        self.level = Level::new();
                        self.save_level();
                    } else {
                        let position = self.geng.window().mouse_pos().map(|x| x as f32);
                        let world_pos = self
                            .render
                            .camera
                            .screen_to_world(self.framebuffer_size, position)
                            .map(Coord::new);
                        self.level.spawn_point = world_pos;
                    }
                }
                _ => {}
            },
            geng::Event::MouseDown { position, button } => {
                let position = position.map(|x| x as f32);
                let world_pos = self
                    .render
                    .camera
                    .screen_to_world(self.framebuffer_size, position)
                    .map(Coord::new);
                self.mouse_drag = Some(MouseDrag {
                    start_camera: self.render.camera.center,
                    start: self.snap_position(world_pos),
                    button,
                });
            }
            geng::Event::MouseMove { position, .. } => {
                let position = position.map(|x| x as f32);
                let world_pos = self
                    .render
                    .camera
                    .screen_to_world(self.framebuffer_size, position);
                if let Some(drag) = &self.mouse_drag {
                    if let geng::MouseButton::Right = drag.button {
                        self.render.camera.center =
                            drag.start_camera + drag.start.map(Coord::as_f32) - world_pos;
                    }
                }
            }
            geng::Event::MouseUp { button, position } => {
                let position = position.map(|x| x as f32);
                let world_pos = self
                    .render
                    .camera
                    .screen_to_world(self.framebuffer_size, position)
                    .map(Coord::new);
                match button {
                    geng::MouseButton::Left => {
                        if let Some(drag) = self.mouse_drag.take() {
                            let p1 = drag.start;
                            let p2 = self.snap_position(world_pos);
                            if (p2 - p1).len().as_f32() > SNAP_DISTANCE {
                                self.level.surfaces.push(Surface { p1, p2 });
                            }
                        }
                    }
                    geng::MouseButton::Right => {
                        if let Some(drag) = self.mouse_drag.take() {
                            if world_pos == drag.start {
                                if let Some(index) = self.find_hovered_surface(world_pos) {
                                    self.level.surfaces.remove(index);
                                }
                            }
                        }
                    }
                    geng::MouseButton::Middle => {
                        self.mouse_drag.take();
                    }
                }
            }
            _ => {}
        }
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);
        self.render.draw_level(&self.level, framebuffer);

        let position = self.geng.window().mouse_pos().map(|x| x as f32);
        let world_pos = self
            .render
            .camera
            .screen_to_world(self.framebuffer_size, position);
        let snapped = self
            .snap_position(world_pos.map(Coord::new))
            .map(Coord::as_f32);

        if let Some(index) = self.find_hovered_surface(world_pos.map(Coord::new)) {
            let surface = self.level.surfaces.get(index).unwrap();
            self.geng.draw_2d(
                framebuffer,
                &self.render.camera,
                &draw_2d::Segment::new(surface.segment_f32(), 0.2, Rgba::new(1.0, 0.0, 0.0, 0.5)),
            );
        }

        self.geng.draw_2d(
            framebuffer,
            &self.render.camera,
            &draw_2d::Quad::new(AABB::point(snapped).extend_uniform(0.1), Rgba::RED),
        );

        if let Some(drag) = &self.mouse_drag {
            if let geng::MouseButton::Left = drag.button {
                self.geng.draw_2d(
                    framebuffer,
                    &self.render.camera,
                    &draw_2d::Segment::new(
                        Segment::new(drag.start.map(Coord::as_f32), snapped),
                        0.1,
                        Rgba::WHITE,
                    ),
                );
            }
        }

        self.geng.draw_2d(
            framebuffer,
            &self.render.camera,
            &draw_2d::Ellipse::circle(self.level.spawn_point.map(Coord::as_f32), 1.0, Rgba::BLUE),
        );
    }

    fn transition(&mut self) -> Option<geng::Transition> {
        self.play.then(|| {
            self.save_level();
            geng::Transition::Switch(Box::new(Game::new(&self.geng, &self.assets)))
        })
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        self.save_level();
    }
}
