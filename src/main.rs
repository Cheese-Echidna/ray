mod camera;
mod hit;
mod intersections;
mod materials;
mod objects;
mod ray;
mod renderer;
mod scene;
pub mod utils;

use crate::intersections::accelerated_polygon::AcceleratedPolygon;
use crate::intersections::plane::Plane;
use crate::intersections::polygon::Polygon;
use crate::intersections::sphere::Sphere;
use crate::intersections::triangle::Triangle;
use crate::materials::diffuse::Diffuse;
use crate::materials::metal::Metal;
use crate::objects::RenderObject;
use crate::utils::ColourChange;
pub use crate::{camera::*, ray::*, renderer::render2, scene::*};
pub use glam::f32::{Vec2, Vec3};
pub use palette::{convert::*, named::*, Srgb};
use crate::materials::texture::Texture;

pub type Length = f32;
pub type Angle = f32;
pub type Vec3Colour = Vec3;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn main() {
    let scene = coloured_spheres();
    render2(scene).unwrap();
}

fn dog_scene() -> Scene {
    let camera = Camera::new_with_control(
        Vec3::new(10.0, 20.0, 10.),
        Vec3::new(0.0, 0.0, 5.0),
        75.,
        10,
        30,
    );

    println!("{:?}", camera);

    let material_ground = Diffuse::new(Vec3::new(0.8, 0.8, 0.0), 1.0);
    let material_center = Metal::new(Vec3::splat(0.8), 0.0);

    let mut objects = vec![
        RenderObject::new(
            AcceleratedPolygon::new_from_stl("dog.stl", 0.1, Vec3::new(10.0, 0.0, 0.0)).unwrap(),
            material_center,
        ),
        RenderObject::new(
            Sphere::new(Vec3::new(5.0, 15.0, 9.0), 2.0),
            materials::clear::Clear::GLASS,
        ),
        RenderObject::new(Plane::new(Vec3::Z, Vec3::new(0., 0., 0.0)), material_ground),
    ];

    let col: fn(Vec3, &Camera) -> Vec3Colour = |d, camera| {
        let a = camera.up().dot(d).clamp(0.0, 1.0);
        Vec3::new(0.8 - 0.3 * a, 0.8 - 0.3 * a, 1.0) * 0.5
    };

    Scene::new(camera, col, objects)
}

fn coloured_spheres() -> Scene {
    let loc = Vec3::new(0.0, 2.0, 0.0);
    let camera = Camera::new_with_control(loc, Vec3::new(0.0, -1.0, 0.0), 75., 5, 5);

    let material_center = Texture::new("assets/earthmap.jpg", None::<&str>, Vec2::splat(1.0));
    let mirror = Metal::new(Vec3::splat(1.0), 0.0);
    let mirror2 = Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0);

    let objects = vec![
        RenderObject::new(Sphere::new(Vec3::new(0., 0., 0.), 0.6), material_center),
    ];

    let col: fn(Vec3, &Camera) -> Vec3Colour = |d, camera| {
        let a = camera.up().dot(d).clamp(0.0, 1.0);
        Vec3::new(0.8 - 0.3 * a, 0.8 - 0.3 * a, 1.0) * 0.5
    };

    Scene::new(camera, col, objects)
}
