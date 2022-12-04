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

    pub fn draw(&self, model: &Model, control: &BodyControl, framebuffer: &mut ugli::Framebuffer) {
        // Level
        self.draw_level(&model.level, model.surface_collision, framebuffer);

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
        // Back arm skeleton
        self.draw_arm(&body.arm_back, &body.center, framebuffer);

        // Body
        self.draw_physics_body(&body.center, Rgba::GRAY, framebuffer);

        // Arm skeleton 💀
        self.draw_arm(&body.arm, &body.center, framebuffer);
    }

    pub fn draw_level(
        &self,
        level: &Level,
        surface_collision: Option<usize>,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        for (i, surface) in level.surfaces.iter().enumerate() {
            let color = if Some(i) == surface_collision {
                Rgba::RED
            } else {
                Rgba::GRAY
            };
            self.geng.draw_2d(
                framebuffer,
                &self.camera,
                &draw_2d::Segment::new(surface.segment_f32(), 0.1, color),
            );
        }
    }

    fn draw_arm(&self, arm: &ArmSkeleton, body: &PhysicsBody, framebuffer: &mut ugli::Framebuffer) {
        let [shoulder, elbow, hand] = arm.get_skeleton(body);
        self.draw_physics_body(&shoulder, SHOULDER_COLOR, framebuffer);
        self.draw_physics_body(&elbow, ELBOW_COLOR, framebuffer);
        self.draw_physics_body(&hand, HAND_COLOR, framebuffer);
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

    fn draw_physics_body(
        &self,
        body: &PhysicsBody,
        color: Rgba<f32>,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        self.draw_collider(body.position, &body.collider, color, framebuffer)
    }

    fn draw_collider(
        &self,
        position: Position,
        collider: &Collider,
        color: Rgba<f32>,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        match collider {
            Collider::Aabb(aabb) => {
                let aabb = aabb.translate(position).map(Coord::as_f32);
                let left_mid = (aabb.bottom_left() + aabb.top_left()) / 2.0;
                let chain = Chain::new(vec![
                    left_mid,
                    aabb.bottom_left(),
                    aabb.bottom_right(),
                    aabb.top_right(),
                    aabb.top_left(),
                    left_mid,
                ]);
                self.geng.draw_2d(
                    framebuffer,
                    &self.camera,
                    &draw_2d::Chain::new(chain, aabb.width() * 0.125, color, 1),
                );
            }
            Collider::Surface(surface) => self.geng.draw_2d(
                framebuffer,
                &self.camera,
                &draw_2d::Segment::new(
                    Segment::new(
                        (surface.p1 + position).map(Coord::as_f32),
                        (surface.p2 + position).map(Coord::as_f32),
                    ),
                    0.1,
                    Rgba::GRAY,
                ),
            ),
        }
    }
}
