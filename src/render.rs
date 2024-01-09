use geng::Camera2d;

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
    // assets: Rc<Assets>,
    pub camera: Camera2d,
}

impl Render {
    pub fn new(geng: &Geng, _assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            // assets: assets.clone(),
            camera: Camera2d {
                center: vec2::ZERO,
                rotation: Angle::ZERO,
                fov: 20.0,
            },
        }
    }

    pub fn draw(&self, model: &Model, control: &BodyControl, framebuffer: &mut ugli::Framebuffer) {
        // Level
        self.draw_level(&model.level, framebuffer);

        // Body
        self.draw_body(&model.player, framebuffer);

        // Hand target
        let hand_target = control.hand_target + model.player.center.position;
        let color = if model.player.holding_to.is_some() {
            HAND_TARGET_HOLD_COLOR
        } else {
            HAND_TARGET_COLOR
        };
        self.draw_point(hand_target, Coord::new(0.3), color, framebuffer);
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
            self.geng.draw2d().draw2d(
                framebuffer,
                &self.camera,
                &draw2d::Segment::new(surface.segment_f32(), 0.1, Rgba::GRAY),
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
        self.geng.draw2d().draw2d(
            framebuffer,
            &self.camera,
            &draw2d::Ellipse::circle_with_cut(
                position.map(|x| x.as_f32()),
                radius.as_f32() * 0.75,
                radius.as_f32(),
                color,
            ),
        );
    }
}
