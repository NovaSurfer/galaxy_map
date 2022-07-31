use notan::app::Texture;
use notan::math::Vec2;

#[rustfmt::skip]
pub const VERTICES: [f32; 16] = [
     0.5,   0.5 ,  1.0, 1.0,
     0.5,  -0.5 ,  1.0, 0.0,
    -0.5,  -0.5 ,  0.0, 0.0,
    -0.5,   0.5 ,  0.0, 1.0,
];


pub const INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

struct RenderableSprite
{

}

pub struct Sprite
{
    texture: Texture,
    position: Vec2
}
