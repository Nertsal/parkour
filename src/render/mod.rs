use geng::{Camera2d, Draw2d};

use super::*;

use physics::*;

pub struct Render {
    geng: Geng,
    assets: Rc<Assets>,
    camera: Camera2d,
}

impl Render {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 100.0,
            },
        }
    }

    pub fn draw(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        self.draw_body(&model.player.body, framebuffer);
    }

    fn draw_body(&self, body: &Body, framebuffer: &mut ugli::Framebuffer) {
        self.draw_point(&body.center, Color::GRAY, framebuffer);
        self.draw_point(&body.absolute_hand(), Color::WHITE, framebuffer);
    }

    fn draw_point(
        &self,
        point: &PhysicsPoint,
        color: Color<f32>,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        draw_2d::Ellipse::circle_with_cut(
            point.position.map(|x| x.as_f32()),
            point.radius.as_f32() * 0.75,
            point.radius.as_f32(),
            color,
        )
        .draw_2d(&self.geng, framebuffer, &self.camera);
    }
}
