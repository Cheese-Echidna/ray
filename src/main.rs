mod camera;
mod ray;
mod renderer;
mod scene;
mod utils;
mod objects;

use crate::renderer::render2;
pub(crate) use crate::utils::convert;
pub(crate) use crate::utils::dprintln;
pub use crate::{camera::*, ray::*, renderer::render, scene::*};
pub use crate::objects::{sphere::Sphere,object::RenderObject};
pub use glam::f32::{Vec2, Vec3};
pub use palette::{convert::*, named::*, LinSrgb, Srgb};
use crate::objects::material::RenderMaterial;
use crate::utils::ColourChange;

pub type Length = f32;
pub type Angle = f32;
pub type Vec3Colour = Vec3;

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

    let col: fn(Vec3, &Camera) -> Vec3Colour = |d, camera| {
        BLUE.to_vec3() * d.dot(camera.up()) + WHITE.to_vec3() * 0.2
    };

    Scene::new(camera, col, shapes)
}
