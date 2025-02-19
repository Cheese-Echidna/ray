use crate::intersections::intersection::{RenderIntersection, OBJECT_TOLERANCE};
use objects::RenderObject;
use crate::*;
use glam::{UVec2, Vec2};
use rand::random;
use rayon::prelude::*;

const TRACES_PER_PIXEL: usize = 10;

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub background: fn(direction: Vec3, camera: &Camera) -> Vec3,
    pub objects: Vec<RenderObject>,
}

impl Scene {
    pub fn new(
        camera: Camera,
        background: fn(Vec3, &Camera) -> Vec3,
        objects: Vec<RenderObject>,
    ) -> Scene {
        Scene {
            camera,
            background,
            objects,
        }
    }

    pub fn trace_from_image_prop(&self, image_prop: UVec2, image_dimensions: UVec2) -> Vec3 {
        (0..TRACES_PER_PIXEL)
            .map(|x| {
                let ray = self.get_outgoing_ray(image_prop, image_dimensions);
                self.trace(ray, 10)
            })
            .sum::<Vec3>()
            / (TRACES_PER_PIXEL as f32)
    }

    fn trace(&self, ray: Ray, depth: u32) -> Vec3 {
        if depth == 0 {
            return BLACK.to_vec3();
        }

        if let Some((object, impact)) = self.intersect(ray, 0.001, None) {
            RED.to_vec3()
        } else {
            (self.background)(ray.direction(), &self.camera)
        }
    }

    fn get_outgoing_ray(&self, current_pixel: UVec2, image_dimensions: UVec2) -> Ray {
        let rand_x: f32 = random();
        let rand_y: f32 = random();

        // Convert pixel indices + random offset into [0..1] normalized coordinates
        let image_prop = Vec2::new(
            (current_pixel.x as f32 + rand_x) / image_dimensions.x as f32,
            (current_pixel.y as f32 + rand_y) / image_dimensions.y as f32,
        );

        // Shift range from [0..1] to [-0.5..0.5] horizontally and vertically
        // Note that y is inverted because screen coords typically go down but we want up in camera space.
        let image_prop = Vec2::new(image_prop.x - 0.5, 0.5 - image_prop.y);

        let [x, y] = image_prop.to_array();

        // Compute aspect ratio based on image dimensions
        let aspect_ratio = image_dimensions.x as f32 / image_dimensions.y as f32;

        // Horizontal field of view
        let tan_fov = (self.camera.hoz_fov.to_radians() / 2.0).tan();

        // Calculate horizontal and vertical offsets
        let right_offset = self.camera.right() * (x * aspect_ratio * tan_fov);
        let up_offset = self.camera.up() * (y * tan_fov);

        // Compute direction
        let direction = (self.camera.forward() + right_offset + up_offset).normalize();

        // Construct the ray with the camera's location as origin.
        Ray::new(self.camera.location, direction, 1.0)
    }

    // this shit needs optimising - O(n) for objects is mad slow
    // it also needs cleaning
    pub fn intersect(
        &self,
        ray: Ray,
        min_distance: f32,
        max_distance: Option<f32>,
    ) -> Option<(&RenderObject, Vec3)> {
        self.objects
            .iter()
            .flat_map(|shape| {
                shape
                    .intersects(ray)
                    .into_iter()
                    .map(|x| (shape, x))
                    .collect::<Vec<_>>()
            })
            .map(|(object, intersection)| (object, intersection, intersection.distance(ray.start)))
            .filter(|(_, _, dist)| *dist >= min_distance)
            .filter(|(_, _, dist)| match max_distance {
                None => true,
                Some(max_distance) => *dist <= max_distance,
            })
            .filter(|(_, _, dist)| dist.is_finite())
            .min_by(|(_, _, x_dist), (_, _, y_dist)| x_dist.total_cmp(y_dist))
            .map(|(a, b, _)| (a, b))
    }
}
