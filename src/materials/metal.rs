use crate::hit::Hit;
use crate::materials::material::RenderMaterial;
use crate::{Ray, Vec3Colour};
use crate::utils::{bounce_across_normal, random_point_on_unit_sphere};

#[derive(Debug, Copy, Clone)]
pub(crate) struct Metal {
    base_colour: Vec3Colour,
    roughness: f32
}

impl Metal {
    pub(crate) fn new(base_colour: Vec3Colour, roughness: f32) -> Self {
        Self {
            base_colour,
            roughness,
        }
    }
}

impl RenderMaterial for Metal {
    fn scatter_ray(&self, hit: Hit) -> Option<Ray> {
        let new_dir = bounce_across_normal(hit.ray.direction(), hit.normal);
        let rand_unit_sph = random_point_on_unit_sphere();
        let new_dir_fuzzed = new_dir + rand_unit_sph * self.roughness;
        Some(Ray::new(hit.impact, new_dir_fuzzed))
    }

    fn colour(&self, _hit: Hit, future_colour: Vec3Colour) -> Vec3Colour {
        future_colour * self.base_colour
    }
}

// impl RenderMaterial for Diffuse {
//     fn scatter_ray(&self, hit: Hit) -> Option<Ray> {
//         let dir = hit.normal + random_point_on_unit_sphere() * self.roughness;
//         Some(Ray::new(hit.impact, dir))
//     }
//
//     fn colour(&self, _hit: Hit, future_colour: Vec3Colour) -> Vec3Colour {
//         self.base_colour * future_colour
//     }
// }