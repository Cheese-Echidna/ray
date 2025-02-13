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
pub(crate) use crate::utils::dprintln;
pub(crate) use crate::utils::convert;

pub type Length = f64;
pub type Angle = f64;

const WIDTH:usize = 1000;
const HEIGHT:usize = 1000;
const SCALE:usize = 1;

fn main() {
    let scene = coloured_spheres();
    render(scene);
}

fn coloured_spheres() -> Scene {
    let loc = 4.0 * Vec3::new(1.0,1.0,1.0);
    let ball_pos = Vec3::new(-3.4322, -2.0, 1.12);
    let camera = Camera::new(loc, ball_pos);

    println!("{:?}", camera);


    // Sphere::new(Vec3::new(-4.0, 0.0, 4.0), 1.0, YELLOW.into()),
    // Sphere::new(3.0 * Vec3::X, 0.5, RED.into()),
    // Sphere::new(3.0 * Vec3::Y, 0.5, GREEN.into()),
    // Sphere::new(Vec3::new(-1., 2., 1.), 0.5, BLUE.into(), 0.0),
    // Sphere::new(Vec3::ZERO, 20.0, WHITE.into(), 0.01)

    let wall_1 = Polygon::new_square(Vec3::new(0., -5., 0.), Vec3::X, Vec3::Z, 20.0, RED.into(), 0.0, 0.0);
    let shapes = vec![
        // convert!(Sphere::new(Vec3::ZERO, 2.0, WHITE.into(), 0.0, 0.05)), // Sphere
        convert!(Sphere::new(ball_pos, 2.0, WHITE.into(), 0.5, 0.0)), // LIGHT
        // convert!(Sphere::new(Vec3::ZERO, 50.0, WHITE.into(), 0.0, 0.0)), // LIGHT
        convert!(wall_1),

    ];
    let col:LinSrgb<f32> = LinSrgb::new(0.1, 0.1, 0.1);

    Scene::new(camera, col, shapes)
}
