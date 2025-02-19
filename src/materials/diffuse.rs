use crate::hit::Hit;
use crate::materials::material::RenderMaterial;
use crate::utils::random_point_on_unit_sphere;
use crate::{Ray, Vec3Colour};

#[derive(Debug)]
pub struct Diffuse {
    base_colour: Vec3Colour,
    roughness: f32,
}

impl Diffuse {
    pub(crate) fn new(base_colour: Vec3Colour, roughness: f32) -> Self {
        Self {
            base_colour,
            roughness,
        }
    }
}

impl RenderMaterial for Diffuse {
    fn scatter_ray(&self, hit: Hit) -> Option<Ray> {
        let dir = hit.normal + random_point_on_unit_sphere() * self.roughness;
        Some(Ray::new(hit.impact, dir))
    }

    fn colour(&self, hit: Hit, future_colour: Vec3Colour) -> Vec3Colour {
        self.base_colour * future_colour
    }
}
