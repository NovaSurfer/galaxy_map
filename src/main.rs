mod cam2d;

use std::cmp::Ordering;
use std::os::linux::raw::stat;
use notan::draw::*;
use notan::log;
use notan::math::{Mat4, Quat, vec2, Vec3, vec3, vec4};
use notan::prelude::*;
use crate::cam2d::{Camera2d};


//language=glsl
const FRAGMENT: ShaderSource = notan::fragment_shader! {
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
const VERTEX: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450 core
    layout (location = 0) in vec2 l_pos;
    layout (location = 1) in vec2 l_uv;

    layout (location = 0) out vec2 v_texcoord;

    layout(set = 0, binding = 1) uniform MVP {
        mat4 mvp;
    };

    void main()
    {
        v_texcoord = l_uv;
        gl_Position = mvp * vec4(l_pos, 0.0, 1.0);
    }
    "#
};

#[uniform]
struct TextureInfo
{
    img_color: Vec3
}

#[uniform]
#[derive(Copy, Clone)]
struct Mvp
{
    mvp: Mat4
}

#[derive(AppState)]
struct State {
    clear_options: ClearOptions,
    texture: Texture,
    pipeline: Pipeline,
    vbo: Buffer,
    ebo: Buffer,
    ubo: Buffer,
    zoom: f32,
    // ubo_mvp: Mat4,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(DrawConfig)
        .add_config(WindowConfig::default().size(640, 480).title("vt"))
        .update(update)
        .draw(draw)
        .build()
}

fn fscale(scale: Vec3) -> Mat4
{
    Mat4::from_cols(
        vec4(scale.x, 0.0, 0.0, 0.0),
        vec4(0.0, scale.y, 0.0, 0.0),
        vec4(0.0, 0.0, scale.z, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0)
    )
}

fn axis_angle(axis: Vec3, angle: f32) -> Mat4
{
    // degrees to radians
    let a = angle * 0174533.0;
    let c = angle.cos();
    let s = angle.sin();
    let t = 1.0 - angle.cos();
    let sq_len = axis.length_squared();
    let cc = &1.0;

    let mut v = axis;
    if sq_len.partial_cmp(cc).unwrap() != Ordering::Equal {
        v = v.normalize();
    }

    Mat4::from_cols(
        vec4(t * (v.x * v.x) + c, t * v.x * v.y + s * v.z, t * v.x * v.z - s * v.y, 0.0),
        vec4(t * v.x * v.y - s * v.z, t * (v.y * v.y) + c, t * v.y * v.z + s * v.x, 0.0),
        vec4(t * v.x * v.z + s * v.y, t * v.y * v.z - s * v.x, t * (v.z * v.z) + c, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0)
    )
}


fn transform(scale: Vec3, rot_axis: Vec3, rot_angle: f32, translate: Vec3) -> Mat4
{
    fscale(scale) * axis_angle(rot_axis, rot_angle) * Mat4::from_translation(translate)
}

fn trs(scale: Vec3, rot_axis: Vec3, rot_angle: f32, translate: Vec3) -> Mat4
{
    Mat4::from_scale_rotation_translation(scale, Quat::from_axis_angle(rot_axis, rot_angle), translate)
}


fn ortho(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32) -> Mat4
{
    let _11 = 2.0 / (right - left);
    let _22 = 2.0 / (top - bottom);
    let _33 = 1.0 / (far - near);
    let _41 = (left + right) / (left - right);
    let _42 = (top + bottom) / (bottom - top);
    let _43 = (near) / (near - far);

    Mat4::from_cols(
        vec4(_11, 0.0, 0.0, 0.0),
        vec4(0.0, _22, 0.0, 0.0),
        vec4(0.0, 0.0, _33, 0.0),
        vec4(_41, _42, _43, 1.0),
    )
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let clear_options = ClearOptions::color(Color::GRAY);
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

    #[rustfmt::skip]
        let vertices = [
        0.5, 0.5, 1.0, 1.0,
        0.5, -0.5, 1.0, 0.0,
        -0.5, -0.5, 0.0, 0.0,
        -0.5, 0.5, 0.0, 1.0,
    ];
    let vertex_buffer = gfx
        .create_vertex_buffer()
        .with_info(&vertex_info)
        .with_data(&vertices)
        .build()
        .unwrap();

    let indices = [0, 1, 3, 1, 2, 3];
    let index_buffer = gfx
        .create_index_buffer()
        .with_data(&indices)
        .build()
        .unwrap();

    let ortho = Mat4::orthographic_rh_gl(0.0, app.window().width() as f32, app.window().height() as f32, 0.0, -1.0, 1.0);
    let transf = trs(
        vec3(512.0, 512.0, 1.0),
        vec3(0.0, 0.0, 1.0),
        0.0,
        vec3(0.5 * 512.0, 0.5 * 512.0, 0.0));
    let mvp = ortho * transf;
    let _mvp = Mvp{mvp};


    let uniform_buffer = gfx
        .create_uniform_buffer(1, "MVP")
        .with_data(&_mvp)
        .build()
        .unwrap();

    State {
        clear_options,
        texture,
        pipeline,
        vbo: vertex_buffer,
        ebo: index_buffer,
        ubo: uniform_buffer,
        zoom: 5.0
    }
}

fn update(app: &mut App, state: &mut State) {

    if app.keyboard.is_down(KeyCode::W) {
        state.zoom += 100.0 * app.timer.delta_f32();
    }

    if app.keyboard.is_down(KeyCode::S) {
        state.zoom -= 100.0 * app.timer.delta_f32();
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let proj = ortho(0.0,
                                         app.window().width() as f32,
                                         app.window().height() as f32,
                                         0.0,
                                         -1.0,
                                         1.0);

    let camera = Camera2d::new(0.0,
                          app.window().width() as f32,
                          app.window().height() as f32,
                          0.0);


    // let sl = Mat4::from_scale(vec3(state.zoom, state.zoom, 0.0));
    // let cent = vec3(app.window().width() as f32 / 2.0, app.window().height() as f32 / 2.0, 0.0);
    // let view = Mat4::from_translation(-cent) * sl * Mat4::from_translation(cent);

    let transf = trs(
        vec3(128.0, 128.0, 1.0),
        vec3(0.0, 0.0, 1.0),
        0.0,
        vec3(0.5 * 128.0 + 320.0, 0.5 * 128.0 + 240.0, 0.0));
    let mvp = camera.projection * transf;
    let _mvp = Mvp{mvp};

    let mut renderer = gfx.create_renderer();
    gfx.set_buffer_data(&state.ubo, &_mvp);

    renderer.begin(Some(&state.clear_options));
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_texture(0, &state.texture);
    renderer.bind_buffers(&[&state.vbo, &state.ebo, &state.ubo]);
    renderer.draw(0, 6);
    renderer.end();

    gfx.render(&renderer);
}