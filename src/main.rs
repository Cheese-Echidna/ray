mod camera;
mod ray;
mod renderer;
mod scene;
mod utils;
mod intersections;
mod materials;
mod objects;

pub(crate) use crate::utils::convert;
pub(crate) use crate::utils::dprintln;
pub use crate::{camera::*, ray::*, renderer::render2, scene::*};
pub use crate::intersections::*;
pub use glam::f32::{Vec2, Vec3};
use glam::FloatExt;
pub use palette::{convert::*, named::*, LinSrgb, Srgb};

use crate::intersections::plane::Plane;
use crate::intersections::sphere::Sphere;
use crate::utils::{scalar_projection, ColourChange};

pub type Length = f32;
pub type Angle = f32;
pub type Vec3Colour = Vec3;

const WIDTH: usize = 1600;
const HEIGHT: usize = 1200;

fn main() {
    let scene = coloured_spheres();
    render2(scene).unwrap();
}

// TODO: Refractive materials dont work because we never reset the refractive index.

fn coloured_spheres() -> Scene {
    let loc = 3.0 * Vec3::new(1.0, 0.0, -0.2);
    let camera = Camera::new(loc, Vec3::ZERO);

    println!("{:?}", camera);

    let shapes = vec![
        // RenderObject::new(convert!(Plane::new(Vec3::Z, Vec3::new(0.0, 0.0, -0.5))), RenderMaterial::plastic(GRAY.to_vec3())),
        // RenderObject::new(convert!(Sphere::new(Vec3::Z * 0.4, 0.4)), RenderMaterial::MIRROR),
        // RenderObject::new(convert!(Sphere::new(Vec3::new(2., -2., 2.), 1.5)), RenderMaterial::light_source(WHITE.to_vec3() * 3.0)),
        // RenderObject::new(convert!(Plane::new(Vec3::X, Vec3::new(-0.5, 0.0, 0.0))), RenderMaterial::plastic(GRAY.to_vec3())),
        // RenderObject::new(convert!(Sphere::new(Vec3::Y * -0.5, 0.4)), RenderMaterial::GOLD),
        // RenderObject::new(convert!(Sphere::new(Vec3::ZERO, 0.4)), RenderMaterial::MIRROR),
    ];

    let col: fn(Vec3, &Camera) -> Vec3Colour = |d, camera| {
        // let a = 0.5*(scalar_projection(d, camera.up()) + 1.0);
        // Vec3::new(0.0, 0.0, a)
        BLACK.to_vec3()
    };

    Scene::new(camera, col, shapes)
}
