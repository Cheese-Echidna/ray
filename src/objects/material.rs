use crate::Vec3;
use palette::named::{BLACK, WHITE};
use crate::utils::ColourChange;

#[derive(Debug, Copy, Clone)]
pub struct RenderMaterial {
    pub albedo: Vec3,
    pub emissivity: Vec3,
    pub roughness: f32,
    pub reflectivity: f32,
}

impl RenderMaterial {
    pub(crate) fn new(albedo: Vec3, emissivity: Vec3, roughness: f32, reflectivity: f32) -> RenderMaterial {
        Self {
            albedo,
            emissivity,
            roughness,
            reflectivity,
        }
    }

    pub(crate) fn new_void() -> Self {
        Self::new(BLACK.to_vec3(), BLACK.to_vec3(), 0.0, 0.0)
    }

    pub(crate) fn new_sun() -> Self {
        Self::new(WHITE.to_vec3(), WHITE.to_vec3(), 0.0, 0.0)
    }
}
