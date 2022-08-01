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

const STARS_NUM: i32 = 500000;

#[derive(AppState)]
struct State {
    clear_options: ClearOptions,
    pipeline: Pipeline,
    camera: Camera2d,
    sprite: Sprite,
    sprite_array_buff: SpriteArrayBuff,
    font: Font,
    count: f32,
    multi: f32,
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

    const ARM_SEPARATION_DIST: f32 = 2.0 * std::f32::consts::PI / 1.0;
    let galaxy_offsets = generate_galaxy_vectors(STARS_NUM, ARM_SEPARATION_DIST, 0.0, 0.0, 0.0);
    let transform = Transform2d::new(vec2(400.0, 300.0), vec2(32.0, 32.0), 0.0);
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
        count: 1.0,
        multi: 1.0,
    }
}

fn event(state: &mut State, evt: Event) {
    state.camera.on_event(&evt);
}

fn update(app: &mut App, state: &mut State) {
    state.camera.on_update(&app.keyboard, app.timer.delta_f32());

    if state.count > 5.0 || state.count < 0.0 {
        state.multi *= -1.0;
    }

    state.count += 0.3 * state.multi * app.timer.delta_f32();
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {

    // drawing galaxy
    let mut renderer = gfx.create_renderer();
    let mvp = state.camera.view_projection * state.sprite.transform.constructed();
    let pixel_size = 5.0 + state.count;
    gfx.set_buffer_data(&state.sprite_array_buff.ubo, &[mvp]);
    gfx.set_buffer_data(&state.sprite_array_buff.px_ubo, &[pixel_size]);

    renderer.begin(Some(&state.clear_options));
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_texture(0, &state.sprite.texture);
    renderer.bind_buffers(&[&state.sprite_array_buff.vbo,
        &state.sprite_array_buff.offset_vbo,
        &state.sprite_array_buff.ebo, &state.sprite_array_buff.ubo,
        &state.sprite_array_buff.px_ubo]);
    renderer.draw_instanced(0, 6, STARS_NUM);
    renderer.end();
    gfx.render(&renderer);
    // drawing text
    let mut draw = gfx.create_draw();
    draw.text(
        &state.font,
        &format!(
            "{} -> {} ({:.3})",
            app.timer.fps().round(),
            STARS_NUM,
            app.timer.delta_f32()
        ),
    )
        .position(10.0, 10.0)
        .size(24.0);

    gfx.render(&draw);
}
