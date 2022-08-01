use notan::math::{Mat4, Quat, Rng, Vec2, Vec3};
use notan::math::rand::thread_rng;
use crate::Random;


pub fn transform2d(scale: Vec2, rot_angle: f32, translate: Vec2) -> Mat4
{
    Mat4::from_scale_rotation_translation(Vec3::new(scale.x, scale.y, 0.0),
                                          Quat::from_axis_angle(Vec3::Z, rot_angle),
                                          Vec3::new(translate.x, translate.y, 0.0))
}

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

        [x, y]
    }).collect::<Vec<f32>>()
}
