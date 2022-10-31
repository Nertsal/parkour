use geng::{Camera2d, Draw2d};

use super::*;

use physics::*;

const HAND_TARGET_COLOR: Rgba<f32> = Rgba {
    r: 0.7,
    g: 0.7,
    b: 0.7,
    a: 1.0,
};
const HAND_TARGET_HOLD_COLOR: Rgba<f32> = Rgba {
    r: 0.7,
    g: 0.4,
    b: 0.4,
    a: 1.0,
};
const SHOULDER_COLOR: Rgba<f32> = Rgba {
    r: 0.5,
    g: 0.5,
    b: 0.5,
    a: 1.0,
};
const ELBOW_COLOR: Rgba<f32> = Rgba {
    r: 0.7,
    g: 0.7,
    b: 0.7,
    a: 1.0,
};
const HAND_COLOR: Rgba<f32> = Rgba {
    r: 0.9,
    g: 0.9,
    b: 0.9,
    a: 1.0,
};

pub struct Render {
    geng: Geng,
    assets: Rc<Assets>,
    pub camera: Camera2d,
}

impl Render {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 20.0,
            },
        }
    }

    pub fn draw(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        // Level
        self.draw_level(&model.level, framebuffer);

        // Ground
        self.geng.draw_2d(
            framebuffer,
            &self.camera,
            &draw_2d::Segment::new(
                Segment::new(vec2(-100.0, -1.0), vec2(100.0, -1.0)),
                0.5,
                Rgba::GRAY,
            ),
        );

        // Body
        self.draw_body(&model.player.body, framebuffer);

        // Hand target
        let hand_target = model.player.control.hand_target + model.player.body.center.position;
        let color = if model.player.body.holding_to.is_some() {
            HAND_TARGET_HOLD_COLOR
        } else {
            HAND_TARGET_COLOR
        };
        self.draw_point(hand_target, Coord::new(0.3), color, framebuffer);

        // Best jump
        if let Some(height) = model.best_jump {
            let height = height.as_f32();
            let x = model.player.body.center.position.x.as_f32();
            self.geng.draw_2d(
                framebuffer,
                &self.camera,
                &draw_2d::Segment::new(
                    Segment::new(vec2(x - 0.5, height), vec2(x + 0.5, height)),
                    0.1,
                    Rgba::WHITE,
                ),
            );

            self.geng.draw_2d(
                framebuffer,
                &self.camera,
                &draw_2d::Text::unit(
                    &**self.geng.default_font(),
                    format!("Best height: {:.2}", height),
                    Rgba::WHITE,
                )
                .align_bounding_box(vec2(0.0, 0.0))
                .scale_uniform(0.5)
                .translate(vec2(x + 5.0, height + 3.0)),
            );
        }
    }

    fn draw_body(&self, body: &Body, framebuffer: &mut ugli::Framebuffer) {
        // Body
        self.draw_point(
            body.center.position,
            body.center.radius,
            Rgba::GRAY,
            framebuffer,
        );

        // Arm skeleton
        let [shoulder, elbow, hand] = body.arm.get_skeleton(&body.center);
        self.draw_point(
            shoulder.position,
            shoulder.radius,
            SHOULDER_COLOR,
            framebuffer,
        );
        self.draw_point(elbow.position, elbow.radius, ELBOW_COLOR, framebuffer);
        self.draw_point(hand.position, hand.radius, HAND_COLOR, framebuffer);
    }

    pub fn draw_level(&self, level: &Level, framebuffer: &mut ugli::Framebuffer) {
        for surface in &level.surfaces {
            self.geng.draw_2d(
                framebuffer,
                &self.camera,
                &draw_2d::Segment::new(surface.segment_f32(), 0.1, Rgba::GRAY),
            );
        }
    }

    fn draw_point(
        &self,
        position: Position,
        radius: Coord,
        color: Rgba<f32>,
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
