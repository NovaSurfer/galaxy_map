use notan::math::{Mat4, Vec2};
use crate::utils;

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
        utils::transform2d(self.scale,
                           self.rotation,
                           Vec2::new(0.5 * self.scale.x + self.position.x, 0.5 * self.scale.y + self.position.y))
    }
}
