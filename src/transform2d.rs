use notan::math::{Mat4, Quat, Vec2, Vec3};

pub struct Transform2d
{
    position: Vec2,
    scale: Vec2,
    rotation: f32,
}

impl Transform2d
{
    pub const fn new(position: Vec2, scale: Vec2, rotation: f32) -> Self
    {
        Self {
            position,
            scale,
            rotation,
        }
    }

    // pub fn set_postion(&mut self, pos: Vec2)
    // {
    //     self.position = pos;
    // }

    pub fn constructed(&mut self) -> Mat4
    {
        transform2d(self.scale,
                           self.rotation,
                           Vec2::new(0.5 * self.scale.x + self.position.x, 0.5 * self.scale.y + self.position.y))
    }
}

fn transform2d(scale: Vec2, rot_angle: f32, translate: Vec2) -> Mat4
{
    Mat4::from_scale_rotation_translation(Vec3::new(scale.x, scale.y, 0.0),
                                          Quat::from_axis_angle(Vec3::Z, rot_angle),
                                          Vec3::new(translate.x, translate.y, 0.0))
}
