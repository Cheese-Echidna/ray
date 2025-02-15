use crate::object::RenderObject;
use crate::*;
use glam::DVec2;
use rayon::prelude::*;

const DIRECT_LIGHT_FACTOR: f32 = 0.1;

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub background: LinSrgb,
    pub objects: Vec<Box<dyn RenderObject>>,
}

impl Scene {
    pub fn new(camera: Camera, background: LinSrgb, objects: Vec<Box<dyn RenderObject>>) -> Scene {
        Scene {
            camera,
            background,
            objects,
        }
    }

    pub fn trace_from_image_prop(&self, image_prop: Vec2) -> LinSrgb {
        let ray = self.get_outgoing_ray(image_prop);
        self.trace(ray, 10)
    }

    fn trace(&self, ray: Ray, depth: u32) -> LinSrgb {
        if depth == 0 {
            return BLACK.into();
        }

        // 1. Find closest intersection
        if let Some((object, impact)) = self.intersect(ray, 0.001, None) {
            // 2. Get emission
            let emitted = object.emission(impact, ray.direction());

            let num_outgoing_rays = 2;

            let outgoing_rays = (0..num_outgoing_rays).into_iter().map(|x| {
                object.scatter_ray(impact, ray.direction())
            }).collect::<Vec<Ray>>();

            // Recursively gather color from the scattered ray(s)
            let incoming_colour = outgoing_rays.into_par_iter().map(|scattered_ray| {
                self.trace(scattered_ray, depth - 1)
            }).collect::<Vec<_>>().into_iter().fold(LinSrgb::new(0.0, 0.0, 0.0), |acc, x| acc + x) / (num_outgoing_rays as f32);

            let attenuation = object.attenuation_colour(impact, ray.direction());

            // Final color = emission + attenuation * color_from_next_bounce
            emitted + attenuation * incoming_colour
        } else {
            // If it doesn't hit anything, return background
            self.background
        }
    }

    fn sample_light_contribution(&self, point_to_sample_at: Vec3) -> LinSrgb {
        self.objects.iter().filter_map(|light_object| {
            let point_on_light_source = light_object.random_point_on_surface();
            let ray = Ray::new_from_to(point_to_sample_at, point_on_light_source);
            match self.intersect(ray, 0.0001, Some(point_to_sample_at.distance(point_on_light_source) - 0.0001)) {
                None => {
                    // no intersections between point on surface and point of light
                    Some(light_object.emission(point_on_light_source, ray.direction()))
                }
                Some((new_object, new_intersect)) => {
                    // Something between the two points, therefore shadow
                    Some(new_object.emission(new_intersect, ray.direction()))
                }
            }
        }).fold(BLACK.into(), |acc, x| acc + x)
    }

    fn get_outgoing_ray(&self, image_prop: DVec2) -> Ray {
        // x and y are camera coords
        // both range from -0.5 to 0.5
        let [x, y] = image_prop.to_array();

        let aspect_ratio = 1.0; // Assuming a square image
        let tan_fov = (self.camera.hoz_fov.to_radians() / 2.0).tan();

        // Calculate horizontal and vertical offsets
        let right_offset = self.camera.right() * (x * aspect_ratio * tan_fov);
        let up_offset = self.camera.up() * (y * tan_fov);

        // Calculate the direction vector
        let direction = (self.camera.forward() + right_offset + up_offset).normalize();

        let ray = Ray::new(self.camera.location, direction);
        ray
    }

    // this shit needs optimising - O(n) for objects is mad slow
    // it also needs cleaning
    pub fn intersect<'a>(
        &'a self,
        ray: Ray,
        min_distance: f64,
        max_distance: Option<f64>,
    ) -> Option<(&'a Box<dyn RenderObject>, Vec3)> {
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
                None => { true }
                Some(max_distance) => { *dist <= max_distance }
            })
            .filter(|(_, _, dist)| dist.is_finite())
            .min_by(|(_, _, x_dist), (_, _, y_dist)| x_dist.total_cmp(y_dist))
            .map(|(a, b, c)| (a, b))
    }
}

