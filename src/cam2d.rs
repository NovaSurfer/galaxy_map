use crate::Quat;
use notan::math::{vec3, Mat4, Vec2, Vec3};
use notan::prelude::{KeyCode, Keyboard};
use notan::Event;

pub struct Camera2d {
    pub projection: Mat4,
    pub view: Mat4,
    pub view_projection: Mat4,
    pub position: Vec2,
    pub speed: f32,
    pub zoom: f32,
    aspect_ratio: f32,
}

impl Camera2d {
    pub fn new(aspect_ratio: f32, speed: f32, zoom: f32) -> Self {
        let ortho = Mat4::orthographic_rh_gl(
            -aspect_ratio * zoom,
            aspect_ratio * zoom,
            -zoom,
            zoom,
            -1.0,
            1.0,
        );
        Self {
            projection: ortho,
            view: Mat4::IDENTITY,
            view_projection: ortho * Mat4::IDENTITY,
            position: Vec2::ZERO,
            speed,
            zoom,
            aspect_ratio,
        }
    }

    pub fn on_event(&mut self, evt: &Event, dt: f32) {
        match evt {
            Event::MouseWheel {
                delta_x: _,
                delta_y,
            } => {
                self.zoom -= delta_y * 50000.0 * dt;
                self.zoom = f32::max(self.zoom, 50.0);
                self.set_projection(
                    -self.aspect_ratio * self.zoom,
                    self.aspect_ratio * self.zoom,
                    -self.zoom,
                    self.zoom,
                );
            }
            _ => {}
        }
    }

    pub fn on_update(&mut self, kb: &Keyboard, dt: f32) {
        if kb.is_down(KeyCode::W) {
            self.position.y += self.speed * dt;
        }
        if kb.is_down(KeyCode::S) {
            self.position.y -= self.speed * dt;
        }
        if kb.is_down(KeyCode::A) {
            self.position.x -= self.speed * dt;
        }
        if kb.is_down(KeyCode::D) {
            self.position.x += self.speed * dt;
        }

        self.reload_view_matrix();
    }

    pub fn set_projection(&mut self, left: f32, right: f32, bottom: f32, top: f32) {
        self.projection = Mat4::orthographic_rh_gl(left, right, bottom, top, -1.0, 1.0);
        self.view_projection = self.projection * self.view;
    }

    pub fn reload_view_matrix(&mut self) {
        let transform = Mat4::from_rotation_translation(
            Quat::from_axis_angle(Vec3::Z, 0.0),
            vec3(self.position.x, self.position.y, 0.0),
        );
        self.view = transform.inverse();
        self.view_projection = self.projection * self.view;
    }
}
