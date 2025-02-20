use crate::hit::Hit;
use crate::*;
use glam::{UVec2, Vec2};
use objects::RenderObject;
use rand::random;

use utils::dprintln;
use crate::utils::vec_format;

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
        let samples = self.camera.samples_per_pixel;
        (0..samples)
            .map(|_x| {
                let ray = self.get_outgoing_ray(image_prop, image_dimensions);
                self.trace(ray, self.camera.max_bounces)
            })
            .sum::<Vec3>()
            / (self.camera.samples_per_pixel as f32)
    }

    fn trace(&self, ray: Ray, depth: u32) -> Vec3 {
        if depth == 0 {
            return BLACK.to_vec3();
        }

        if let Some((object, hit)) = self.intersect(ray, 0.001, None) {
            let new_colour = object
                .material
                .scatter_ray(hit)
                .map(|new_ray| self.trace(new_ray, depth - 1))
                .unwrap_or(BLACK.to_vec3());
            object.material.colour(hit, new_colour)
        } else {
            (self.background)(ray.direction(), &self.camera)
        }
    }

    fn print_trace(&self, ray: Ray, depth: u32) -> Vec3 {
        let spaces = self.camera.max_bounces - depth;

        let prefix = if spaces == 0 {
            "".to_string()
        } else {
            "|".to_string() + &"-".repeat(spaces as usize - 1)
        };

        if depth == 0 {
            dprintln!("{prefix}Depth = 0: returning black");
            return BLACK.to_vec3();
        }

        if let Some((object, hit)) = self.intersect(ray, 0.001, None) {
            dprintln!("{prefix}Object hit {}", hit);
            let scatter = object
                .material
                .scatter_ray(hit);

            if let Some(new_ray) = scatter {
                dprintln!("{prefix}New ray scatter in dir: {}", vec_format(new_ray.direction()));
            } else {
                dprintln!("{prefix}New ray not scattered");
            }

            let traced_colour = scatter.map(|new_ray| self.print_trace(new_ray, depth - 1))
                .unwrap_or(BLACK.to_vec3());

            dprintln!("{prefix}Traced colour = {}", vec_format(traced_colour));

            let colour = object.material.colour(hit, traced_colour);

            dprintln!("{prefix}New colour = {}", vec_format(colour));

            colour
        } else {
            dprintln!("{prefix}Nothing hit, returning background in direction {}", vec_format(ray.direction()));
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
        Ray::new(self.camera.location, direction)
    }

    // this shit needs optimising - O(n) for objects is mad slow
    // it also needs cleaning
    pub fn intersect(
        &self,
        ray: Ray,
        min_distance: f32,
        max_distance: Option<f32>,
    ) -> Option<(&RenderObject, Hit)> {
        self.objects
            .iter()
            .flat_map(|shape| {
                shape
                    .intersector
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
            .map(|(a, b, _)| (a, Hit::new(a, b, ray)))
    }
}
