use geng::{Camera2d, Draw2d};

use super::*;

use physics::*;

const HAND_TARGET_COLOR: Color<f32> = Color {
    r: 0.7,
    g: 0.7,
    b: 0.7,
    a: 1.0,
};

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
                fov: 50.0,
            },
        }
    }

    pub fn draw(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        self.draw_body(&model.player.body, framebuffer);
        let hand_target = model.player.relative_target + model.player.body.center.position;
        self.draw_point(
            hand_target,
            model.player.body.relative_hand.radius * Coord::new(0.5),
            HAND_TARGET_COLOR,
            framebuffer,
        );
    }

    fn draw_body(&self, body: &Body, framebuffer: &mut ugli::Framebuffer) {
        self.draw_point(
            body.center.position,
            body.center.radius,
            Color::GRAY,
            framebuffer,
        );

        let hand = body.absolute_hand();
        self.draw_point(hand.position, hand.radius, Color::WHITE, framebuffer);
    }

    fn draw_point(
        &self,
        position: Position,
        radius: Coord,
        color: Color<f32>,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        draw_2d::Ellipse::circle_with_cut(
            position.map(|x| x.as_f32()),
            radius.as_f32() * 0.75,
            radius.as_f32(),
            color,
        )
        .draw_2d(&self.geng, framebuffer, &self.camera);
    }
}
