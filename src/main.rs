mod ray;
mod camera;
mod scene;
mod renderer;
mod sphere;
mod object;
mod triangle;
mod polygon;
mod utils;
// mod lights;

use std::iter::Zip;
use glam::DVec3;
pub use palette::{Srgb, named::*, convert::*, LinSrgb};
pub use glam::f64::{DVec3 as Vec3, DVec2 as Vec2};
use winit::event::VirtualKeyCode::B;
pub use crate::{ray::*, scene::*, camera::*, renderer::render, sphere::*};
use crate::object::RenderObject;
use crate::polygon::Polygon;
use crate::renderer::render2;
pub(crate) use crate::utils::dprintln;
pub(crate) use crate::utils::convert;

pub type Length = f64;
pub type Angle = f64;

const WIDTH:usize = 1200;
const HEIGHT:usize = 1200;
const SCALE:usize = 1;

fn main() {
    let scene = coloured_spheres();
    render2(scene).unwrap();
}

fn coloured_spheres() -> Scene {
    let loc = 4.0 * Vec3::new(1.0,1.0,1.0);
    let camera = Camera::new(loc, Vec3::Z);

    println!("{:?}", camera);


    // Sphere::new(Vec3::new(-4.0, 0.0, 4.0), 1.0, YELLOW.into()),
    // Sphere::new(3.0 * Vec3::X, 0.5, RED.into()),
    // Sphere::new(3.0 * Vec3::Y, 0.5, GREEN.into()),
    // Sphere::new(Vec3::new(-1., 2., 1.), 0.5, BLUE.into(), 0.0),
    // Sphere::new(Vec3::ZERO, 20.0, WHITE.into(), 0.01)

    // let floor = Polygon::new_square(Vec3::ZERO, Vec3::X, Vec3::Y, 200.0, RED.into(), 0.0, 0.03);
    let shapes = vec![
        convert!(Sphere::new(Vec3::new(2.0, 4.0, 4.0), 1.0, WHITE.into(), 1.0, 0.03)), // LIGHT
        convert!(Sphere::new(Vec3::ZERO, 2.0, LinSrgb::new(1.0, 0.4, 0.4), 0.1, 0.0)), // RED
        convert!(Sphere::new(Vec3::new(2.0, 0.0, 3.0), 1.0, LinSrgb::new(0.4, 1.0, 0.4), 0.1, 0.04)), // GREEN
        // convert!(floor),

    ];
    let col:LinSrgb<f32> = BLACK.into();

    Scene::new(camera, col, shapes)
}
