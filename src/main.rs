mod ray;
mod camera;
mod scene;
mod renderer;
mod sphere;
mod object;
mod lights;

pub use palette::{Srgb, named::*, convert::*};
pub use glam::f64::{DVec3 as Vec3, DVec2 as Vec2};
use palette::WithAlpha;
pub use crate::{ray::*, scene::*, camera::*, renderer::render, sphere::*};
use crate::object::RenderObject;

pub type Length = f64;
pub type Angle = f64;

const WIDTH:usize = 1000;
const HEIGHT:usize = 1000;
const THRESHOLD:f64 = 0.000001;

fn main() {
    let scene = coloured_spheres();
    render(scene);
}

fn coloured_spheres() -> Scene {
    let loc = 6.0 * Vec3::new(1.0,1.0,1.0);
    let camera = Camera::new_looking_in_dir(loc, -Vec3::new(1.0,1.0,1.0));

    println!("{:?}", camera);

    let shapes = vec![
        // Sphere::new(Vec3::new(-4.0, 0.0, 4.0), 1.0, YELLOW.into()),
        Sphere::new(Vec3::ZERO, 2.0, WHITE.into()),
        // Sphere::new(3.0 * Vec3::X, 0.5, RED.into()),
        // Sphere::new(3.0 * Vec3::Y, 0.5, GREEN.into()),
        // Sphere::new(Vec3::Z * 3.0, 0.5, BLUE.into()),
    ].into_iter().map(|x| Box::new(x) as Box<dyn RenderObject>).collect::<Vec<Box<dyn RenderObject>>>();
    let bg = BLACK.into();

    Scene::new(camera, bg, shapes)
}