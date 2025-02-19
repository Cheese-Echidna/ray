use palette::LinSrgb;
use palette::named::{BLACK, WHITE};

#[derive(Debug, Copy, Clone)]
pub struct RenderMaterial {
    pub albedo: LinSrgb,
    pub emissivity: LinSrgb,
    pub roughness: f64,
    pub reflectivity: f64,
}

impl RenderMaterial {
    pub(crate) fn new(albedo: LinSrgb, emissivity: LinSrgb, roughness: f64, reflectivity: f64) -> RenderMaterial {
        Self {
            albedo,
            emissivity,
            roughness,
            reflectivity,
        }
    }

    pub(crate) fn new_void() -> Self {
        Self::new(BLACK.into(), BLACK.into(), 0.0, 0.0)
    }

    pub(crate) fn new_sun() -> Self {
        Self::new(WHITE.into(), WHITE.into(), 0.0, 0.0)
    }
}
