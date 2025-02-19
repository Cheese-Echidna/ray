mod camera;
mod hit;
mod intersections;
mod materials;
mod objects;
mod ray;
mod renderer;
mod scene;
mod utils;

pub use crate::{camera::*, ray::*, renderer::render2, scene::*};
pub use glam::f32::{Vec2, Vec3};
pub use palette::{convert::*, named::*, LinSrgb, Srgb};

use crate::intersections::plane::Plane;
use crate::intersections::sphere::Sphere;
use crate::materials::diffuse::Diffuse;
use crate::materials::lightsource::LightSource;
use crate::objects::RenderObject;
use crate::utils::ColourChange;

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
    let loc = Vec3::new(0.0, 4.0, 0.5);
    let camera = Camera::new(loc, Vec3::new(0.0, 0.0, 0.3));

    println!("{:?}", camera);

    let mut objects = vec![
        RenderObject::new(
            Plane::new(Vec3::Z, Vec3::new(0.0, 0.0, 0.0)),
            Diffuse::new(WHITE.to_vec3(), 0.5),
        ),
        // RenderObject::new(convert!(Sphere::new(Vec3::new(2., -2., 2.), 1.5)), RenderMaterial::light_source(WHITE.to_vec3() * 3.0)),
        // RenderObject::new(convert!(Plane::new(Vec3::X, Vec3::new(-0.5, 0.0, 0.0))), RenderMaterial::plastic(GRAY.to_vec3())),
        // RenderObject::new(convert!(Sphere::new(Vec3::Y * -0.5, 0.4)), RenderMaterial::GOLD),
        // RenderObject::new(convert!(Sphere::new(Vec3::ZERO, 0.4)), RenderMaterial::MIRROR),
    ];

    objects.push(RenderObject::new(
        Sphere::new(Vec3::new(0.0, 0.0, 1.5), 0.6),
        LightSource::new(WHITE.to_vec3()),
    ));

    (0..=4).for_each(|x| {
        let object = RenderObject::new(
            Sphere::new(Vec3::new(x as f32 - 2.0, 0.0, 0.4), 0.4),
            Diffuse::new(Vec3::new(0.29, 0.37, 0.5), x as f32 / 4.0),
        );
        objects.push(object)
    });

    let col: fn(Vec3, &Camera) -> Vec3Colour = |d, camera| {
        let a = camera.up().dot(d);
        Vec3::new(0.3 * a, 0.4 * a, a)
    };

    Scene::new(camera, col, objects)
}
