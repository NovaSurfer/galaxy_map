mod cam2d;
mod sprite;
mod utils;
mod renderable;
mod transform2d;

use notan::draw::*;
use notan::math::{Quat, vec2};
use notan::prelude::*;

use crate::cam2d::Camera2d;
use crate::renderable::SpriteArrayBuff;
use crate::sprite::Sprite;
use crate::transform2d::Transform2d;
use crate::utils::generate_galaxy_vectors;

#[derive(AppState)]
struct State {
    clear_options: ClearOptions,
    pipeline: Pipeline,
    camera: Camera2d,
    sprite: Sprite,
    sprite_array_buff: SpriteArrayBuff,
    font: Font,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(DrawConfig)
        .add_config(WindowConfig::default().size(800, 600).title("vt"))
        .event(event)
        .update(update)
        .draw(draw)
        .build()
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let clear_options = ClearOptions::color(Color::BLACK);

    let texture = gfx
        .create_texture()
        .from_image(include_bytes!("../assets/flare_01.png"))
        .with_premultiplied_alpha()
        .build()
        .unwrap();

    let camera = Camera2d::new(0.0,
                               app.window().width() as f32,
                               app.window().height() as f32,
                               0.0,
                               50.0,
                               50.0);

    const ARM_SEPARATION_DIST: f32 = 2.0 * std::f32::consts::PI / 5.0;
    let galaxy_offsets = generate_galaxy_vectors(500000, ARM_SEPARATION_DIST, 0.5, 5.0, 0.02);
    let transform = Transform2d::new(vec2(400.0, 300.0), vec2(16.0, 16.0), 0.0);
    let mut sprite = Sprite::new(texture, transform);
    let mvp = camera.view_projection * sprite.transform.constructed();
    let sprite_array_buff = SpriteArrayBuff::new(gfx, &galaxy_offsets, mvp);


    let pipeline = gfx
        .create_pipeline()
        .from(&sprite::SPRITE_VERTEX, &sprite::SPRITE_FRAGMENT)
        .with_vertex_info(&sprite_array_buff.vert_info)
        .with_vertex_info(&sprite_array_buff.vert_offset_info)
        .with_color_blend(BlendMode::NORMAL)
        .with_texture_location(0, "u_texture")
        .build()
        .unwrap();

    let font = gfx
        .create_font(include_bytes!("../assets/Ubuntu-B.ttf"))
        .unwrap();

    State {
        clear_options,
        pipeline,
        camera,
        sprite,
        sprite_array_buff,
        font,
    }
}

fn event(state: &mut State, evt: Event) {
    state.camera.on_event(&evt);
}

fn update(app: &mut App, state: &mut State) {
    state.camera.on_update(&app.keyboard, app.timer.delta_f32());
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {

    // drawing galaxy
    let mut renderer = gfx.create_renderer();
    let mvp = state.camera.view_projection * state.sprite.transform.constructed();
    gfx.set_buffer_data(&state.sprite_array_buff.ubo, &[mvp]);

    renderer.begin(Some(&state.clear_options));
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_texture(0, &state.sprite.texture);
    renderer.bind_buffers(&[&state.sprite_array_buff.vbo, &state.sprite_array_buff.ebo, &state.sprite_array_buff.ubo, &state.sprite_array_buff.offset_vbo]);
    renderer.draw_instanced(0, 6, 500000);
    renderer.end();
    gfx.render(&renderer);

    // drawing text
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

    gfx.render(&draw);
}
