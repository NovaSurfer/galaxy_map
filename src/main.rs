mod cam2d;
mod sprite;

use std::cmp::Ordering;
use std::os::linux::raw::stat;
use notan::draw::*;
use notan::graphics::crevice::internal::bytemuck::offset_of;
use notan::log;
use notan::math::{Mat4, Quat, vec2, Vec2, Vec3, vec3, vec4};
use notan::prelude::*;
use crate::cam2d::{Camera2d};


//language=glsl
const FRAGMENT: ShaderSource = fragment_shader! {
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
const VERTEX: ShaderSource = vertex_shader! {
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


#[derive(Copy, Clone)]
struct Mvp
{
    mvp: Mat4,
}

#[derive(AppState)]
struct State {
    clear_options: ClearOptions,
    texture: Texture,
    pipeline: Pipeline,
    vbo: Buffer,
    ebo: Buffer,
    ubo: Buffer,
    camera: Camera2d,
    offset_vbo: Buffer,
    font: Font
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(DrawConfig)
        .add_config(WindowConfig::default().size(640, 480).title("vt"))
        .event(event)
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
    let clear_options = ClearOptions::color(Color::BLACK);
    let texture = gfx
        .create_texture()
        .from_image(include_bytes!("../assets/flare_01.png"))
        .with_premultiplied_alpha()
        .build()
        .unwrap();


    let vertex_info = VertexInfo::new()
        .attr(0, VertexFormat::Float32x2)
        .attr(1, VertexFormat::Float32x2);

    let vertex_offset_info = VertexInfo::new()
        .attr(2, VertexFormat::Float32x2)
        .step_mode(VertexStepMode::Instance);

    let pipeline = gfx
        .create_pipeline()
        .from(&VERTEX, &FRAGMENT)
        .with_vertex_info(&vertex_info)
        .with_vertex_info(&vertex_offset_info)
        .with_color_blend(BlendMode::NORMAL)
        .with_texture_location(0, "u_texture")
        .build()
        .unwrap();

    let vertex_buffer = gfx
        .create_vertex_buffer()
        .with_info(&vertex_info)
        .with_data(&sprite::VERTICES)
        .build()
        .unwrap();

    let index_buffer = gfx
        .create_index_buffer()
        .with_data(&sprite::INDICES)
        .build()
        .unwrap();

    let mut camera = Camera2d::new(0.0,
                                   app.window().width() as f32,
                                   app.window().height() as f32,
                                   0.0,
                                   50.0,
                                   50.0);
    let transf = trs(
        vec3(1.0, 1.0, 1.0),
        vec3(0.0, 0.0, 1.0),
        0.0,
        vec3(0.5 * 1.0+320.0, 0.5 * 1.0+240.0, 0.0));
    let mvp = camera.view_projection * transf;
    let _mvp = Mvp{mvp};


    let uniform_buffer = gfx
        .create_uniform_buffer(1, "MVP")
        .with_data(&[_mvp.mvp])
        .build()
        .unwrap();


    // let offset = vec![vec2(1.0, 0.0), vec2(2.0, 0.0), vec2(3.0,0.0)];
    // let mut f32_offset: Vec<f32> = Vec::new();
    // for v in offset
    // {
    //     f32_offset.push(v.x);
    //     f32_offset.push(v.y);
    //
    // }
    // let f32_offset_arr:[f32; 6] = f32_offset[..].try_into().unwrap();

    let numArms : f32 = 5.0;
    let armSeparationDistance = 2.0 * std::f32::consts::PI / numArms;
    let armOffsetMax = 0.5;
    let rotationFactor = 5.0;
    let randomOffsetXY = 0.02;

    let mut rng = Random::default();
    let colors = (0..500000)
        .into_iter()
        .flat_map(|_| {
            let mut dist: f32 = rng.gen_range(0.0..50.0);
            dist = dist.powf(2.0);
            let mut angle = rng.gen_range(0.0..50.0) * 2.0 * std::f32::consts::PI;
            let mut arm_offset = rng.gen_range(0.0..50.0) * armOffsetMax;
            arm_offset = arm_offset - armOffsetMax / 2.0;
            arm_offset = arm_offset * (1.0 / dist);
            let mut sqArmOffset = arm_offset.powf(2.0);
            if arm_offset < 1.0{
                sqArmOffset = sqArmOffset * -1.0;
            }
            arm_offset = sqArmOffset;
            let rot = dist * rotationFactor;
            angle = (angle / armSeparationDistance) * armSeparationDistance + arm_offset + rot;
            let mut x = angle.cos() * dist;
            let mut y = angle.sin() * dist;
            let rnd_offset_x: f32 = rng.gen_range(0.0..50.0) * randomOffsetXY;
            let rnd_offset_y: f32 = rng.gen_range(0.0..50.0) * randomOffsetXY;
            x += rnd_offset_x;
            y += rnd_offset_y;

            [
                x,
                y,
            ]
        })
        .collect::<Vec<f32>>();

    let offset_buffer = gfx
        .create_vertex_buffer()
        .with_info(&vertex_offset_info)
        .with_data(&colors)
        .build()
        .unwrap();
    let font = gfx
        .create_font(include_bytes!("../assets/Ubuntu-B.ttf"))
        .unwrap();

    State {
        clear_options,
        texture,
        pipeline,
        vbo: vertex_buffer,
        ebo: index_buffer,
        ubo: uniform_buffer,
        camera,
        offset_vbo: offset_buffer,
        font
    }
}

fn event(state: &mut State, evt: Event) {
    state.camera.on_event(&evt);
}

fn update(app: &mut App, state: &mut State) {
   state.camera.on_update(&app.keyboard, app.timer.delta_f32());
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {

    let transf = trs(
        vec3(16.0, 16.0, 1.0),
        vec3(0.0, 0.0, 1.0),
        45.0,
        vec3(0.5 * 16.0+320.0, 0.5 * 16.0+240.0, 0.0));
    let mvp = state.camera.view_projection * transf;
    // let mut offset: [Vec2; 3] = [vec2(0.0, 0.0), vec2(100.0, 100.0), vec2(200.0,200.0)];
    // let mvp= Mat4::IDENTITY;
    let _mvp = Mvp{mvp};

    let mut renderer = gfx.create_renderer();
    gfx.set_buffer_data(&state.ubo, &[_mvp.mvp]);

    renderer.begin(Some(&state.clear_options));
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_texture(0, &state.texture);
    renderer.bind_buffers(&[&state.vbo, &state.ebo, &state.ubo, &state.offset_vbo]);
    renderer.draw_instanced(0, 6, 500000);
    renderer.end();

    let mut draw = gfx.create_draw();
    draw.text(
        &state.font,
        &format!(
            "{} -> {} ({:.6})",
            app.timer.fps().round(),
            500000,
            app.timer.delta_f32()
        ),
    )
        .position(10.0, 10.0)
        .size(24.0);

    gfx.render(&renderer);
    gfx.render(&draw);
}
