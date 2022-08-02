mod cam2d;
mod sprite;
mod utils;
mod renderable;
mod transform2d;

use notan::egui::{self, *};
use notan::draw::*;
use notan::prelude::*;
use notan::math::{Quat, vec2};

use crate::cam2d::Camera2d;
use crate::renderable::SpriteArrayBuff;
use crate::sprite::{SpriteArray};
use crate::transform2d::Transform2d;
use crate::utils::{GalaxyConfig, generate_galaxy_vectors};

const STARS_NUM: i32 = 5000;

#[derive(AppState)]
struct State {
    pipeline: Pipeline,
    camera: Camera2d,
    sprite: SpriteArray,
    sprite_array_buff: SpriteArrayBuff,
    font: Font,
    count: f32,
    multi: f32,
    galaxy_config: GalaxyConfig,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(DrawConfig)
        .add_config(EguiConfig)
        .add_config(WindowConfig::default().size(800, 600).title("volka"))
        .event(event)
        .update(update)
        .draw(draw)
        .build()
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {

    let texture = gfx
        .create_texture()
        .from_image(include_bytes!("../assets/flare_01.png"))
        .with_premultiplied_alpha()
        .build()
        .unwrap();

    let (scr_x, scr_y) = (app.window().width() as f32, app.window().height() as f32);
    let mut camera = Camera2d::new(0.0, scr_x, scr_y, 0.0, 500.0, 1.0);
    camera.reload_view_matrix();
    const ARM_SEPARATION_DIST: f32 = 2.0 * std::f32::consts::PI / 500.0;
    let galaxy_config: GalaxyConfig = GalaxyConfig { size: STARS_NUM, arm_separation_dist: ARM_SEPARATION_DIST, arm_offset_max: 20.0, rotation_factor: 0.5, random_offset_xy: 10.0 };
    let (galaxy_transform, galaxy_offsets) = generate_galaxy_vectors(galaxy_config);
    let mut sprite = SpriteArray::new(texture, galaxy_transform);
    let sprite_array_buff = SpriteArrayBuff::new(gfx, &galaxy_offsets, &camera.view_projection);

    let pipeline = gfx
        .create_pipeline()
        .from(&sprite::SPRITE_VERTEX, &sprite::SPRITE_FRAGMENT)
        .with_vertex_info(&sprite_array_buff.vert_info)
        .with_vertex_info(&sprite_array_buff.vert_instanced_info)
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
        galaxy_config,
    }
}

fn event(state: &mut State, evt: notan::prelude::Event) {
    state.camera.on_event(&evt);
}

fn update(app: &mut App, plugins: &mut Plugins, state: &mut State) {
    state.camera.on_update(&app.keyboard, app.timer.delta_f32());

    if state.count > 5.0 || state.count < 0.0 {
        state.multi *= -1.0;
    }

    state.count += 0.3 * state.multi * app.timer.delta_f32();
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {

    // drawing galaxy
    let mut renderer = gfx.create_renderer();
    let pixel_size = state.count;

    gfx.set_buffer_data(&state.sprite_array_buff.px_ubo, &[pixel_size]);
    gfx.set_buffer_data(&state.sprite_array_buff.cam_ubo, &[state.camera.view_projection]);

    renderer.begin(Some(&ClearOptions::color(Color::BLACK)));
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_texture(0, &state.sprite.texture);
    renderer.bind_buffers(&[&state.sprite_array_buff.vbo,
        &state.sprite_array_buff.instanced_vbo,
        &state.sprite_array_buff.ebo,
        &state.sprite_array_buff.px_ubo,
        &state.sprite_array_buff.cam_ubo]);
    renderer.draw_instanced(0, 6, state.galaxy_config.size);
    renderer.end();
    gfx.render(&renderer);

    //drawing ui
    let tool_ui = plugins.egui(|ctx| {
        draw_egui_widget(ctx, gfx, state);
    });
    gfx.render(&tool_ui);

    // drawing text
    let mut draw = gfx.create_draw();
    draw.text(
        &state.font,
        &format!(
            "{} ({:.3})",
            app.timer.fps().round(),
            app.timer.delta_f32()
        ),
    )
        .position(10.0, 10.0)
        .size(18.0);
    gfx.render(&draw);
}

// Creates a widget to change the properties
fn draw_egui_widget(ctx: &Context, gfx: &mut Graphics, state: &mut State) {
    Window::new("Galaxy config")
        .resizable(false)
        .show(ctx, |ui| draw_egui_ui(ui, gfx, state));
}

// UI Description
fn draw_egui_ui(ui: &mut egui::Ui, gfx: &mut Graphics, state: &mut State) {
    Grid::new("custom_grid")
        .num_columns(2)
        // .spacing([40.0, 6.0])
        .show(ui, |ui| {
            ui.label("arm separation dist");
            ui.add(DragValue::new(&mut state.galaxy_config.arm_separation_dist));
            ui.end_row();


            ui.label("arm offset max");
            ui.add(DragValue::new(&mut state.galaxy_config.arm_offset_max));
            ui.end_row();


            ui.label("rotation factor");
            ui.add(DragValue::new(&mut state.galaxy_config.rotation_factor));
            ui.end_row();


            ui.label("random offset x-y");
            ui.add(DragValue::new(&mut state.galaxy_config.random_offset_xy));
            ui.end_row();

            ui.label("size");
            ui.add(DragValue::new(&mut state.galaxy_config.size));
            ui.end_row();

            if ui.button("GENERATE").clicked()
            {
                println!("GENERATE");
                // let (scr_x, scr_y) = (app.window().width() as f32, app.window().height() as f32);
                const ARM_SEPARATION_DIST: f32 = 2.0 * std::f32::consts::PI / 500.0;

                let galaxy_config = GalaxyConfig {
                    size: state.galaxy_config.size,
                    arm_separation_dist: state.galaxy_config.arm_separation_dist,
                    arm_offset_max: state.galaxy_config.arm_offset_max,
                    rotation_factor: state.galaxy_config.rotation_factor,
                    random_offset_xy: state.galaxy_config.random_offset_xy,
                };

                let (galaxy_transform, galaxy_offsets) = generate_galaxy_vectors(galaxy_config);
                gfx.clean();
                let sprite_array_buff = SpriteArrayBuff::new(gfx, &galaxy_offsets, &state.camera.view_projection);
                state.galaxy_config = galaxy_config;
                state.sprite.set_transform(galaxy_transform);
                state.sprite_array_buff = sprite_array_buff;
            }
            ui.end_row();
        });
}
