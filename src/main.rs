mod camera;
mod ray;
mod renderer;
mod scene;
mod utils;
mod objects;

use crate::object::RenderIntersection;
use crate::renderer::render2;
pub(crate) use crate::utils::convert;
pub(crate) use crate::utils::dprintln;
pub use crate::{camera::*, ray::*, renderer::render, scene::*};
pub use crate::objects::*;
pub use glam::f32::{Vec2, Vec3};
use glam::FloatExt;
pub use palette::{convert::*, named::*, LinSrgb, Srgb};
use crate::objects::material::RenderMaterial;
use crate::objects::object::RenderObject;
use crate::objects::plane::Plane;
use crate::objects::sphere::Sphere;
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

// TODO: Refractive materials dont work because we never reset the refractive index.

fn coloured_spheres() -> Scene {
    let loc = 3.0 * Vec3::new(1.0, 0.0, 0.2);
    let camera = Camera::new(loc, Vec3::ZERO);

    println!("{:?}", camera);

    let shapes = vec![
        // RenderObject::new(convert!(Plane::new(Vec3::Z, Vec3::NEG_Z)), RenderMaterial::plastic(DARKGREEN.to_vec3())),
        RenderObject::new(convert!(Sphere::new(Vec3::Y * -0.5, 0.4)), RenderMaterial::MIRROR),
        RenderObject::new(convert!(Sphere::new(Vec3::Y * 0.5, 0.4)), RenderMaterial::plastic(RED.to_vec3())),
        RenderObject::new(convert!(Sphere::new(Vec3::new(3., -3., 3.), 1.5)), RenderMaterial::light_source(WHITE.to_vec3())),
        // RenderObject::new(convert!(Sphere::new(Vec3::ZERO, 0.4)), RenderMaterial::MIRROR),
    ];

    let col: fn(Vec3, &Camera) -> Vec3Colour = |d, camera| {
        Vec3::new(0.2, 0.2, 1.5) * (d.dot(camera.up()) * 0.5 + 0.5) * 0.2
    };

    Scene::new(camera, col, shapes)
}
