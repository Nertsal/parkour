use crate::{control::BodyControl, model::*, Assets};

use geng::prelude::*;
use geng_utils::conversions::*;

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
const HAND_PUSH_COLOR: Rgba<f32> = Rgba {
    r: 0.7,
    g: 0.7,
    b: 0.7,
    a: 1.0,
};
const HAND_PUSH_GRAB_COLOR: Rgba<f32> = Rgba {
    r: 0.0,
    g: 0.7,
    b: 0.7,
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

        // Hand push point
        if let Some(push) = &model.player.push_point {
            let color = if push.grabbed {
                HAND_PUSH_GRAB_COLOR
            } else {
                HAND_PUSH_COLOR
            };
            self.draw_circle(
                model.player.collider.position + push.position,
                r32(0.2),
                color,
                framebuffer,
            );
        }

        // Hand target
        let hand_target = control.hand_target + model.player.collider.position;
        let color =
        // if model.player.holding_to.is_some() {
        //     HAND_TARGET_HOLD_COLOR
        // } else {
            HAND_TARGET_COLOR
        // }
        ;
        self.draw_circle_outline(hand_target, r32(0.3), color, framebuffer);
    }

    fn draw_circle(
        &self,
        pos: Position,
        radius: Coord,
        color: Rgba<f32>,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        self.geng.draw2d().draw2d(
            framebuffer,
            &self.camera,
            &draw2d::Ellipse::circle(pos.as_f32(), radius.as_f32(), color),
        );
    }

    fn draw_circle_outline(
        &self,
        pos: Position,
        radius: Coord,
        color: Rgba<f32>,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        self.geng.draw2d().draw2d(
            framebuffer,
            &self.camera,
            &draw2d::Ellipse::circle_with_cut(
                pos.as_f32(),
                radius.as_f32() * 0.75,
                radius.as_f32(),
                color,
            ),
        );
    }

    fn draw_collider(
        &self,
        collider: &Collider,
        color: Rgba<f32>,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        match collider.shape {
            Shape::Segment { a, b } => {
                self.geng.draw2d().draw2d(
                    framebuffer,
                    &self.camera,
                    &draw2d::Segment::new(Segment(a.as_f32(), b.as_f32()), 0.1, color),
                );
            }
            Shape::Circle { radius } => {
                self.geng.draw2d().draw2d(
                    framebuffer,
                    &self.camera,
                    &draw2d::Ellipse::circle(collider.position.as_f32(), radius.as_f32(), color),
                );
            }
            Shape::Rectangle { width, height } => {
                self.geng.draw2d().draw2d(
                    framebuffer,
                    &self.camera,
                    &draw2d::Quad::new(
                        Aabb2::point(collider.position.as_f32())
                            .extend_symmetric(vec2(width, height).as_f32() / 2.0),
                        color,
                    ),
                );
            }
        }
    }

    fn draw_body(&self, body: &Body, framebuffer: &mut ugli::Framebuffer) {
        // Body
        self.draw_collider(&body.collider, Rgba::GRAY, framebuffer);

        // Arm skeleton
        // let [shoulder, elbow, hand] = body.arm.get_skeleton(&body.center);
        // self.draw_point(
        //     shoulder.position,
        //     shoulder.radius,
        //     SHOULDER_COLOR,
        //     framebuffer,
        // );
        // self.draw_point(elbow.position, elbow.radius, ELBOW_COLOR, framebuffer);
        // self.draw_point(hand.position, hand.radius, HAND_COLOR, framebuffer);
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
}
