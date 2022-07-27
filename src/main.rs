use notan::draw::*;
use notan::math::{Vec3, vec3};
use notan::prelude::*;

//TODO: Create transform struct, fill mvp inside vertex shader.

//language=glsl
const FRAGMENT: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec2 v_uvs;
    layout(location = 1) in vec4 v_color;

    layout(binding = 0) uniform sampler2D u_texture;
    layout(set = 0, binding = 1) uniform TextureInfo {
        float u_size;
        vec3 u_img_color;
    };

    layout(location = 0) out vec4 color;

    void main() {
        vec2 tex_size = textureSize(u_texture, 0);
        vec2 p_size = vec2(u_size);
        vec2 coord = fract(v_uvs) * tex_size;
        coord = floor(coord/p_size) * p_size;
        color = texture(u_texture, coord / tex_size) * v_color * vec4(u_img_color, 1.0);
    }
"#
};

//language=glsl
const VERTEX: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450 core
    layout (location = 0) in vec2 l_pos;
    layout (location = 1) in vec2 l_uv;

    layout (location = 0) out vec2 TexCoords;

    layout(set = 0, binding = 0) uniform MVP {
        mat4 mvp;
    };

    void main()
    {
        TexCoords = l_uv;
        gl_Position = mvp * vec4(l_pos, 0.0, 1.0);
    }
    "#
};

#[uniform]
struct TextureInfo
{
    size: f32,
    img_color: Vec3
}

#[derive(AppState)]
struct State {
    clear_options: ClearOptions,
    texture: Texture,
    pipeline: Pipeline,
    uniforms: Buffer,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    count: f32,
    multi: f32,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    let clear_options = ClearOptions::color(Color::new(0.1, 0.2, 0.3, 1.0));

    let texture = gfx
        .create_texture()
        .from_image(include_bytes!("../assets/flare_01.png"))
        .with_premultiplied_alpha()
        .build()
        .unwrap();


    let vertex_info = VertexInfo::new()
        .attr(0, VertexFormat::Float32x2)
        .attr(1, VertexFormat::Float32x2);

    let pipeline = gfx
        .create_pipeline()
        .from(&VERTEX, &FRAGMENT)
        .with_vertex_info(&vertex_info)
        .with_color_blend(BlendMode::NORMAL)
        .with_texture_location(0, "u_texture")
        .build()
        .unwrap();

    let texture_info = TextureInfo{size: 5.0, img_color: vec3(1.0, 1.0, 1.0)};
    let uniforms = gfx
        .create_uniform_buffer(1, "TextureInfo")
        .with_data(&texture_info)
        .build()
        .unwrap();


    #[rustfmt::skip]
        let vertices = [
        0.5, 0.5,     1.0, 1.0,
        0.5, -0.5,    1.0, 0.0,
        -0.5, -0.5,   0.0, 0.0,
        -0.5, 0.5,    0.0, 1.0,
    ];


    let vertex_buffer = gfx
        .create_vertex_buffer()
        .with_info(&vertex_info)
        .with_data(&vertices)
        .build()
        .unwrap();

    let indices = [0, 1, 2, 0, 2, 3];
    let index_buffer = gfx
        .create_index_buffer()
        .with_data(&indices)
        .build()
        .unwrap();

    State {
        clear_options,
        texture,
        pipeline,
        uniforms,
        vertex_buffer,
        index_buffer,
        count: 1.0,
        multi: 1.0,
    }
}

// Change the size of the pixel effect
fn update(app: &mut App, state: &mut State) {
    if state.count > 5.0 || state.count < 0.0 {
        state.multi *= -1.0;
    }

    state.count += 0.3 * state.multi * app.timer.delta_f32();
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let pixel_size = 5.0 + state.count;
    let texture_info = TextureInfo{size: pixel_size, img_color: vec3(0.5, 1.0, 0.5)};
    gfx.set_buffer_data(&state.uniforms, &texture_info);

    let mut renderer = gfx.create_renderer();

    let mut draw = gfx.create_draw();
    renderer.begin(Some(&state.clear_options));
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_texture(0, &state.texture);
    renderer.bind_buffers(&[&state.vertex_buffer, &state.index_buffer]);
    renderer.draw(0, 6);
    renderer.end();

    gfx.render(&renderer);
}
