use glam::Vec3;
use crate::materials::material::RenderMaterial;
use crate::{Ray, Vec3Colour};

struct LightSource {
    colour: Vec3Colour
}

impl RenderMaterial for LightSource {
    fn scatter_ray(&self, ray: Ray, impact: Vec3) -> Option<Ray> {
        None
    }

    fn colour_ray(&self, ) -> Option<Vec3Colour> {
        self.colour
    }
}