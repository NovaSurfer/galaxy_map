use notan::math::{Mat4, Rng, vec2, Vec2};
use notan::prelude::{Color, Random};
use crate::{Transform2d};
use crate::transform2d::transform2d;

const STAR_COLORS: [Color; 12] = [
    Color::new(231.0, 236.0, 254.0, 1.0),
    Color::new(245.0, 247.0, 255.0, 1.0),
    Color::new(254.0, 254.0, 254.0, 1.0),
    Color::new(255.0, 251.0, 229.0, 1.0),
    Color::new(255.0, 243.0, 189.0, 1.0),
    Color::new(255.0, 212.0, 138.0, 1.0),
    Color::new(255.0, 163.0, 138.0, 1.0),
    Color::new(247.0, 128.0, 95.0, 1.0),
    Color::new(238.0, 79.0, 58.0, 1.0),
    Color::new(223.0, 60.0, 38.0, 1.0),
    Color::new(197.0, 51.0, 32.0, 1.0),
    Color::new(175.0, 54.0, 39.0, 1.0)
];

const SCALE_ARRAY: [f32; 4] =
[
    16.0,
    32.0,
    64.0,
    128.0,
];

#[derive(Clone, Copy)]
pub struct GalaxyConfig
{
    pub size: i32,
    pub arm_separation_dist: f32,
    pub arm_offset_max: f32,
    pub rotation_factor: f32,
    pub random_offset_xy: f32
}

pub fn generate_galaxy_vectors(galaxy_consts: GalaxyConfig) -> (Mat4, Vec<f32>)
{
    let mut rnd = Random::default();
    let mut transf = Mat4::IDENTITY;
    let vbo_size = (galaxy_consts.size * 16 + galaxy_consts.size * 3) as usize;
    let mut vbo_buffer: Vec<f32> = Vec::with_capacity(vbo_size);
    println!("capacity {}", vbo_buffer.capacity());

    for _n in 0..galaxy_consts.size
    {
        let scale = SCALE_ARRAY[rnd.gen_range(0..SCALE_ARRAY.len() - 1)];
        let mut dist: f32 = rnd.gen_range(0.0..1.0 * scale);
        dist = dist.powf(2.0);

        let mut angle = rnd.gen_range(0.0..1.0 * scale) * 2.0 * std::f32::consts::PI;

        let mut arm_offset = rnd.gen_range(0.0..1.0 * scale) * galaxy_consts.arm_offset_max;
        arm_offset = arm_offset - galaxy_consts.arm_offset_max / 2.0;
        arm_offset = arm_offset * (1.0 / dist);

        let mut sq_arm_offset = arm_offset.powf(2.0);
        if arm_offset < 0.0 {
            sq_arm_offset = sq_arm_offset * -1.0;
        }
        arm_offset = sq_arm_offset;

        let rot = dist * galaxy_consts.rotation_factor;
        angle = (angle / galaxy_consts.arm_separation_dist) * galaxy_consts.arm_separation_dist + arm_offset + rot;

        let mut x = angle.cos() * dist;
        let mut y = angle.sin() * dist;

        let rnd_offset_x: f32 = rnd.gen_range(0.0..1.0 * scale) * galaxy_consts.random_offset_xy;
        let rnd_offset_y: f32 = rnd.gen_range(0.0..1.0 * scale) * galaxy_consts.random_offset_xy;

        x += rnd_offset_x;
        y += rnd_offset_y;

        //transform
        transf = transform2d(vec2(scale, scale),
                                        rnd.gen_range(0.0..180.0),
                                        vec2(0.5 * scale + x, 0.5 * scale + y));
        vbo_buffer.extend(transf.to_cols_array().iter());

        // colors
        let rnd_color = STAR_COLORS[rnd.gen_range(0..STAR_COLORS.len() - 1)];
        let rgb = [rnd_color.r / 255.0, rnd_color.g / 255.0, rnd_color.b / 255.0];
        vbo_buffer.extend(rgb.iter());
    }

    (transf, vbo_buffer)
}
