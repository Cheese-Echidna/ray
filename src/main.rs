mod camera;
mod ray;
mod renderer;
mod scene;
mod utils;
mod objects;
// mod lights;

use crate::renderer::render2;
pub(crate) use crate::utils::convert;
pub(crate) use crate::utils::dprintln;
pub use crate::{camera::*, ray::*, renderer::render, scene::*};
pub use crate::objects::{sphere::Sphere,object::RenderObject};
pub use glam::f64::{DVec2 as Vec2, DVec3 as Vec3};
use glam::DVec3;
pub use palette::{convert::*, named::*, LinSrgb, Srgb};
use crate::objects::material::RenderMaterial;

pub type Length = f64;
pub type Angle = f64;

const WIDTH: usize = 1200;
const HEIGHT: usize = 1200;
const SCALE: usize = 1;

fn main() {
    let scene = coloured_spheres();
    render2(scene).unwrap();
}

fn coloured_spheres() -> Scene {
    let loc = 4.0 * Vec3::new(1.0, 1.0, -1.0);
    let camera = Camera::new(loc, Vec3::ZERO);

    println!("{:?}", camera);

    let shapes = vec![];

    let col: fn(DVec3, &Camera) -> LinSrgb = |d, camera| {
        let into = |x| <palette::rgb::Rgb<palette::encoding::Srgb, u8> as Into<LinSrgb<f32>>>::into(x);
        into(BLUE) * (d.dot(camera.up()) as f32) + into(WHITE) * 0.2
    };

    Scene::new(camera, col, shapes)
}
