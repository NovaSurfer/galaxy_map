use notan::math::{Mat4};
use notan::prelude::{Buffer, Graphics, VertexFormat, VertexInfo, VertexStepMode};
use crate::sprite::{QUAD_INDICES, QUAD_VERTICES};

pub struct SpriteArrayBuff
{
    pub vbo: Buffer,
    pub offset_vbo: Buffer,
    pub ebo: Buffer,
    pub ubo: Buffer,
    pub px_ubo: Buffer,
    pub vert_info: VertexInfo,
    pub vert_offset_info: VertexInfo,
}

impl SpriteArrayBuff
{
    pub fn new(gfx: &mut Graphics, offsets: &Vec<f32>, mvp: Mat4) -> Self
    {
        // vert buff
        let vertex_info = VertexInfo::new()
            .attr(0, VertexFormat::Float32x2)
            .attr(1, VertexFormat::Float32x2);

        let vertex_buff = gfx
            .create_vertex_buffer()
            .with_info(&vertex_info)
            .with_data(&QUAD_VERTICES)
            .build()
            .unwrap();


        // vert offsets
        let vertex_offset_info = VertexInfo::new()
            .attr(2, VertexFormat::Float32x2)
            .attr(3, VertexFormat::Float32x3)
            .step_mode(VertexStepMode::Instance);

        let vertex_offset_buff = gfx
            .create_vertex_buffer()
            .with_info(&vertex_offset_info)
            .with_data(&offsets)
            .build()
            .unwrap();

        // ebo
        let index_buffer = gfx
            .create_index_buffer()
            .with_data(&QUAD_INDICES)
            .build()
            .unwrap();

        // ubo
        let uniform_buffer = gfx
            .create_uniform_buffer(1, "MVP")
            .with_data(&[mvp])
            .build()
            .unwrap();


        let uniform_pxsize_buffer = gfx
            .create_uniform_buffer(2, "TextureInfo")
            .with_data(&[5.0])
            .build()
            .unwrap();

        Self
        {
            vbo: vertex_buff,
            offset_vbo: vertex_offset_buff,
            ebo: index_buffer,
            ubo: uniform_buffer,
            px_ubo: uniform_pxsize_buffer,
            vert_info: vertex_info,
            vert_offset_info: vertex_offset_info,
        }
    }
}