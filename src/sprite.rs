use notan::app::Texture;
use notan::{fragment_shader, vertex_shader};
use notan::prelude::ShaderSource;
use crate::transform2d::Transform2d;

#[rustfmt::skip]
pub const QUAD_VERTICES: [f32; 16] = [
     0.5,  0.5,   1.0, 1.0,
     0.5, -0.5,   1.0, 0.0,
    -0.5, -0.5,   0.0, 0.0,
    -0.5,  0.5,   0.0, 1.0,
];


pub const QUAD_INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

//language=glsl
pub const SPRITE_FRAGMENT: ShaderSource = fragment_shader! {
    r#"
    #version 450
    precision mediump float;
    layout(location = 0) in vec2 v_texcoord;
    layout(location = 0) out vec4 outColor;
    layout(binding = 0) uniform sampler2D u_texture;
    void main() {
        outColor = texture(u_texture, v_texcoord);
    }
    "#
};

//language=glsl
pub const SPRITE_VERTEX: ShaderSource = vertex_shader! {
    r#"
    #version 450 core
    precision mediump float;
    layout (location = 0) in vec2 l_pos;
    layout (location = 1) in vec2 l_uv;
    layout (location = 2) in vec2 offset;

    layout (location = 0) out vec2 v_texcoord;

    layout(set = 0, binding = 1) uniform MVP {
        mat4 mvp;
    };

    void main()
    {
        v_texcoord = l_uv;
        gl_Position = mvp * vec4(l_pos + offset, 0.0, 1.0);
    }
    "#
};



pub struct Sprite
{
    pub texture: Texture,
    pub transform: Transform2d,
}

impl Sprite
{
    pub const fn new(texture: Texture, transform: Transform2d) -> Self {
        Self{
            texture,
            transform
        }
    }
}





