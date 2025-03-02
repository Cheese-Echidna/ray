use glam::Vec3;
use rand::random;
use crate::hit::Hit;
use crate::materials::material::RenderMaterial;
use crate::{utils, Ray, Vec3Colour};
use crate::utils::{bounce_across_normal, reflectance};

#[derive(Debug)]
pub(crate) struct Clear {
    colour: Vec3Colour,
    refractive_index: f32,
    roughness: f32
}

impl Clear {
    fn new(colour: Vec3Colour, refractive_index: f32, roughness: f32) -> Self {
        Self {
            colour,
            refractive_index,
            roughness,
        }
    }
    pub(crate) const GLASS: Self = Self {
        colour: Vec3::new(1.0, 1.0, 1.0),
        refractive_index: 1.5,
        roughness: 0.0,
    };
    pub(crate) const INV_GLASS: Self = Self {
        colour: Vec3::new(1.0, 1.0, 1.0),
        refractive_index: 1.0/1.5,
        roughness: 0.0,
    };
    pub(crate) const AIR_BUBBLE: Self = Self {
        colour: Vec3::new(1.0, 1.0, 1.0),
        refractive_index: 1.00 / 1.3333,
        roughness: 0.0,
    };
}

impl RenderMaterial for Clear {
    fn scatter_ray(&self, hit: Hit) -> Option<Ray> {

        let ri = if hit.on_outside() {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let cos_theta = (-hit.ray.direction()).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || reflectance(cos_theta, ri) > random::<f32>() {
            // This means we reflect
            bounce_across_normal(hit.ray.direction(), hit.normal)
        } else {
            // this means we refract
            hit.ray.direction().refract(hit.normal, ri)
        };
        // let direction = hit.ray.direction().refract(hit.normal, ri);
        Some(Ray::new(hit.impact, direction))
    }

    fn colour(&self, hit: Hit, future_colour: Vec3Colour) -> Vec3Colour {
        future_colour
    }
}