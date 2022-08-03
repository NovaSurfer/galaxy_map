use notan::math::{vec2, Mat4, Quat, Rng, Vec2, Vec3};
use notan::prelude::{Color, Random};

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
    Color::new(175.0, 54.0, 39.0, 1.0),
];

const SCALE_ARRAY: [f32; 4] = [16.0, 32.0, 64.0, 128.0];

#[derive(Clone, Copy)]
pub struct GalaxyConfig {
    pub size: i32,
    pub arm_numb: f32,
    pub arm_separation_dist: f32,
    pub arm_offset_max: f32,
    pub rotation_factor: f32,
    pub random_offset_xy: f32,
}

pub fn transform2d(scale: Vec2, rot_angle: f32, translate: Vec2) -> Mat4 {
    Mat4::from_scale_rotation_translation(
        Vec3::new(scale.x, scale.y, 0.0),
        Quat::from_axis_angle(Vec3::Z, rot_angle),
        Vec3::new(translate.x, translate.y, 0.0),
    )
}

pub fn generate_galaxy_vectors(galaxy_consts: GalaxyConfig) -> (Mat4, Vec<f32>) {
    let mut rnd = Random::default();
    let mut transf = Mat4::IDENTITY;
    let vbo_size = (galaxy_consts.size * 16 + galaxy_consts.size * 3) as usize;
    let mut vbo_buffer: Vec<f32> = Vec::with_capacity(vbo_size);

    for _n in 0..galaxy_consts.size {
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
        angle = (angle / galaxy_consts.arm_separation_dist) * galaxy_consts.arm_separation_dist
            + arm_offset
            + rot;

        let mut x = angle.cos() * dist;
        let mut y = angle.sin() * dist;

        let rnd_offset_x: f32 = rnd.gen_range(0.0..1.0 * scale) * galaxy_consts.random_offset_xy;
        let rnd_offset_y: f32 = rnd.gen_range(0.0..1.0 * scale) * galaxy_consts.random_offset_xy;

        x += rnd_offset_x;
        y += rnd_offset_y;

        //transform
        transf = transform2d(
            vec2(scale, scale),
            rnd.gen_range(0.0..180.0),
            vec2(0.5 * scale + x, 0.5 * scale + y),
        );
        vbo_buffer.extend(transf.to_cols_array().iter());

        // colors
        let rnd_color = STAR_COLORS[rnd.gen_range(0..STAR_COLORS.len() - 1)];
        let rgb = [
            rnd_color.r / 255.0,
            rnd_color.g / 255.0,
            rnd_color.b / 255.0,
        ];
        vbo_buffer.extend(rgb.iter());
    }

    // returns transform matrix that will be later multiplied with camera's view_projection
    // vbo_buffer contains all positions for all starts and their colors
    (transf, vbo_buffer)
}

// TODO: Make quad-tree first, generate galaxy within quad-tree boundary.

// #[derive(Clone, Copy)]
// pub struct Rect
// {
//     pub origin: Vec2,
//     pub size: Vec2,
// }
//
// impl Rect
// {
//     pub const fn new(origin: Vec2, size: Vec2) -> Self
//     {
//         Self {
//             origin,
//             size,
//         }
//     }
//
//     pub fn rect_rect(self, other: Rect) -> bool
//     {
//         let amin = Rect::get_min(self);
//         let amax = Rect::get_max(self);
//
//         let bmin = Rect::get_min(other);
//         let bmax = Rect::get_max(other);
//
//         // https://github.com/rust-lang/rust/issues/57241 - ?
//         let over_x = (bmin.x <= amax.x) && (amin.x <= bmax.x);
//         let over_y = (bmin.y <= amax.y) && (amin.y <= bmax.y);
//
//         over_x && over_y
//     }
//
//     fn get_min(rect: Rect) -> Vec2
//     {
//
//         let p1 = rect.origin;
//         let p2 = rect.origin + rect.size;
//
//         vec2(f32::min(p1.x, p2.x), f32::min(p1.y, p2.y))
//     }
//
//     fn get_max(rect: Rect) -> Vec2
//     {
//         let p1 = rect.origin;
//         let p2 = rect.origin + rect.size;
//
//         vec2(f32::max(p1.x, p2.x), f32::max(p1.y, p2.y))
//     }
// }
