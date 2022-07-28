use notan::math::{Mat4, Vec3};
use crate::Quat;

pub struct Camera2d
{
    pub projection: Mat4,
    pub view: Mat4,
    pub view_projection: Mat4,
    pub position: Vec3
}

impl Camera2d
{
    pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        let ortho = Mat4::orthographic_rh_gl(left, right, bottom, top, -1.0, 1.0);
        Self{
            projection: ortho,
            view: Mat4::IDENTITY,
            view_projection: ortho * Mat4::IDENTITY,
            position: Vec3::ZERO,
        }
    }

    pub fn set_position(mut self, pos: Vec3)
    {
        self.position = pos;
        self.reload_view_matrix();
    }

    pub fn set_projection(mut self, left: f32, right: f32, bottom: f32, top: f32)
    {
        self.projection = Mat4::orthographic_rh_gl(left, right, bottom, top, -1.0, 1.0);
        self.view_projection = self.projection * self.view;
    }

    fn reload_view_matrix(mut self)
    {
        let transform = Mat4::from_rotation_translation(Quat::from_axis_angle(Vec3::Z, 0.0), self.position);
        self.view = transform.inverse();
        self.view_projection = self.projection * self.view;
    }
}