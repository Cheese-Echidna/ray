mod camera;
mod hit;
mod intersections;
mod materials;
mod objects;
mod ray;
mod renderer;
mod scene;
pub mod utils;

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
use crate::intersections::accelerated_polygon::AcceleratedPolygon;

pub type Length = f32;
pub type Angle = f32;
pub type Vec3Colour = Vec3;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn main() {
    let scene = default_scene();
    render2(scene).unwrap();
}

fn default_scene() -> Scene {
    let camera = Camera::new_with_control(
        Vec3::new(10.0, 20.0, 10.),
        Vec3::new(0.0, 0.0, 5.0),
        75.,
        10,
        10,
    );

    println!("{:?}", camera);

    let material_ground = Diffuse::new(Vec3::new(0.8, 0.8, 0.0), 1.0);
    let material_center = Diffuse::new(Vec3::new(0.1, 0.2, 0.5),0.5);

    let mut objects = vec![
        RenderObject::new(
            AcceleratedPolygon::new_from_stl("dog.stl", 0.1, Vec3::new(10.0, 0.0, 0.0)).unwrap(),
            material_center,
        ),
        RenderObject::new(
            Plane::new(Vec3::Z, Vec3::new(0., 0., 0.0)),
            material_ground,
        ),
    ];

    let col: fn(Vec3, &Camera) -> Vec3Colour = |d, camera| {
        let a = camera.up().dot(d).clamp(0.0, 1.0);
        Vec3::new(0.8 - 0.3 * a, 0.8 - 0.3 * a, 1.0) * 0.5
    };

    Scene::new(camera, col, objects)
}

fn coloured_spheres() -> Scene {
    let loc = Vec3::new(0.0, 2.0, 0.05);
    let camera = Camera::new_with_control(loc, Vec3::new(0.0, -1.0, 0.0), 75., 5, 1);

    let material_ground = Diffuse::new(Vec3::new(0.8, 0.8, 0.0), 1.0);
    let material_center = Diffuse::new(Vec3::new(0.1, 0.2, 0.5), 1.0);
    let material_right = Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0);

    let mut objects = vec![
        // RenderObject::new(
        //     Plane::new(Vec3::Z, Vec3::new(0.0, 0.0, 0.0)),
        //     Diffuse::new(WHITE.to_vec3(), 0.5),
        // ),
        RenderObject::new(
            Plane::new(Vec3::Z, Vec3::new(0., 0., -0.5)),
            // Sphere::new(Vec3::new(0.0, -1.0, -100.5), 100.0),
            material_ground,
        ),
        RenderObject::new(Sphere::new(Vec3::new(0.0, -0.4, 0.), 0.5), material_center),
        RenderObject::new(
            Sphere::new(Vec3::new(1.0, 0.0, 0.0), 0.5),
            materials::clear::Clear::GLASS,
        ),
        // RenderObject::new(Sphere::new(Vec3::new(1.0, -1.0, 0.0), 0.4), materials::clear::Clear::INV_GLASS),
        RenderObject::new(Sphere::new(Vec3::new(-1.0, 0.0, 0.0), 0.5), material_right),
    ];

    let col: fn(Vec3, &Camera) -> Vec3Colour = |d, camera| {
        let a = camera.up().dot(d).clamp(0.0, 1.0);
        Vec3::new(0.8 - 0.3 * a, 0.8 - 0.3 * a, 1.0) * 0.5
    };

    Scene::new(camera, col, objects)
}
