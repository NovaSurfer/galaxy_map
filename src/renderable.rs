use notan::math::{Mat4};
use notan::prelude::{Buffer, Graphics, VertexFormat, VertexInfo, VertexStepMode};
use crate::sprite::{QUAD_INDICES, QUAD_VERTICES};

pub struct SpriteArrayBuff
{
    pub vbo: Buffer,
    pub instanced_vbo: Buffer,
    pub ebo: Buffer,
    pub px_ubo: Buffer,
    pub cam_ubo: Buffer,
    pub vert_info: VertexInfo,
    pub vert_instanced_info: VertexInfo,
}

impl SpriteArrayBuff
{
    pub fn new(gfx: &mut Graphics, offsets: &Vec<f32>, cam_view_project: &Mat4) -> Self
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
        let vertex_instanced_info = VertexInfo::new()
            .attr(3, VertexFormat::Float32x4)
            .attr(4, VertexFormat::Float32x4)
            .attr(5, VertexFormat::Float32x4)
            .attr(6, VertexFormat::Float32x4)
            .attr(7, VertexFormat::Float32x3)
            .step_mode(VertexStepMode::Instance);

        let vertex_instanced_buff = gfx
            .create_vertex_buffer()
            .with_info(&vertex_instanced_info)
            .with_data(&offsets)
            .build()
            .unwrap();

        // ebo
        let index_buffer = gfx
            .create_index_buffer()
            .with_data(&QUAD_INDICES)
            .build()
            .unwrap();

        let uniform_pxsize_buffer = gfx
            .create_uniform_buffer(1, "TextureInfo")
            .with_data(&[5.0])
            .build()
            .unwrap();

        let uniform_camera_buffer = gfx
            .create_uniform_buffer(2, "Camera")
            .with_data(cam_view_project)
            .build()
            .unwrap();

        Self
        {
            vbo: vertex_buff,
            instanced_vbo: vertex_instanced_buff,
            ebo: index_buffer,
            px_ubo: uniform_pxsize_buffer,
            cam_ubo: uniform_camera_buffer,
            vert_info: vertex_info,
            vert_instanced_info: vertex_instanced_info,
        }
    }
}
