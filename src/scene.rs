use crate::objects::object::{RenderIntersection, RenderObject, OBJECT_TOLERANCE};
use crate::*;
use glam::Vec2;
use rayon::prelude::*;

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

    pub fn trace_from_image_prop(&self, image_prop: Vec2) -> Vec3 {
        let ray = self.get_outgoing_ray(image_prop);
        self.trace(ray, 10)
    }

    fn trace(&self, ray: Ray, depth: u32) -> Vec3 {
        if depth == 0 {
            return BLACK.to_vec3();
        }

        if let Some((object, impact)) = self.intersect(ray, 0.001, None) {
            // If emissive, return emission immediately
            if object.is_emitter() {
                return object.emission() * object.ray_normal_closeness(impact, ray.direction()).powf(0.5);
            };

            // lection = reflection
            // raction = refraction

            let lection_ray = object.scatter_ray(impact, ray.direction(), ray.current_ior);
            let lection_colour = self.trace(lection_ray, depth - 1);

            let (raction_ray_opt, fresnel) =
                object.refract_ray(impact, ray.direction(), ray.current_ior);

            let raction_colour = raction_ray_opt
                .map(|ract_ray| self.trace(ract_ray, depth - 1))
                .unwrap_or(Vec3::ZERO);

            let lection_weight = fresnel * (1.0 - object.transmission());
            let raction_weight = (1.0 - fresnel) * object.transmission();
            // let lection_colour_blended =
                // lection_colour.lerp(object.colour(), 1.0 - object.metallic());

            let colour = lection_weight * lection_colour + raction_weight * raction_colour;

            colour
        } else {
            (self.background)(ray.direction(), &self.camera)
        }
    }

    // fn incident_light(&self, point: Vec3) -> Vec3 {
    //     self.objects
    //         .iter()
    //         .filter(|x| x.is_emitter())
    //         .filter_map(|x| {
    //             let random_point = x.random_point_on_surface();
    //             let ray = Ray::new_from_to(point, random_point);
    //             let intersection = self.intersect(ray, OBJECT_TOLERANCE, None);
    //             let value = |impact: Vec3| {
    //                 x.emission() * x.ray_normal_closeness(impact, ray.direction()) as f32
    //             };
    //             if let Some((_object, intersection)) = intersection {
    //                 if x.includes_point_on_surface(intersection) {
    //                     Some(value(intersection))
    //                 } else {
    //                     None
    //                 }
    //             } else {
    //                 Some(value(random_point))
    //             }
    //         })
    //         .fold(Vec3::new(0., 0., 0.), |acc, x| acc + x)
    // }

    fn get_outgoing_ray(&self, image_prop: Vec2) -> Ray {
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

        let ray = Ray::new(self.camera.location, direction, 1.000293);
        ray
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
