use notan::math::Rng;
use notan::prelude::Color;
use crate::Random;

pub const STAR_COLORS: [Color; 12] = [
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

pub fn generate_galaxy_vectors(size: i32, arm_separation_dist: f32, arm_offset_max: f32, rotation_factor: f32, random_offset_xy: f32) -> Vec<f32>
{
    let mut rnd = Random::default();
    (0..size).into_iter().flat_map(|_| {
        let mut dist: f32 = rnd.gen_range(0.0..30.0);
        dist = dist.powf(2.0);

        let mut angle = rnd.gen_range(0.0..30.0) * 2.0 * std::f32::consts::PI;

        let mut arm_offset = rnd.gen_range(0.0..30.0) * arm_offset_max;
        arm_offset = arm_offset - arm_offset_max / 2.0;
        arm_offset = arm_offset * (1.0 / dist);

        let mut sq_arm_offset = arm_offset.powf(2.0);
        if arm_offset < 0.0 {
            sq_arm_offset = sq_arm_offset * -1.0;
        }
        arm_offset = sq_arm_offset;

        let rot = dist * rotation_factor;
        angle = (angle / arm_separation_dist) * arm_separation_dist + arm_offset + rot;

        let mut x = angle.cos() * dist;
        let mut y = angle.sin() * dist;

        let rnd_offset_x: f32 = rnd.gen_range(0.0..30.0) * random_offset_xy;
        let rnd_offset_y: f32 = rnd.gen_range(0.0..30.0) * random_offset_xy;

        x += rnd_offset_x;
        y += rnd_offset_y;

        // colors
        let rnd_color = STAR_COLORS[rnd.gen_range(0..STAR_COLORS.len() - 1)];

        [x, y, rnd_color.r / 255.0, rnd_color.g / 255.0, rnd_color.b / 255.0]
    }).collect::<Vec<f32>>()
}
