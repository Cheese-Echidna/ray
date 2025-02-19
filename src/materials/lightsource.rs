use crate::hit::Hit;
use crate::materials::material::RenderMaterial;
use crate::{Ray, Vec3Colour};

#[derive(Debug)]
pub struct LightSource {
    colour: Vec3Colour,
}

impl LightSource {
    pub fn new(colour: Vec3Colour) -> Self {
        Self { colour }
    }
}

impl RenderMaterial for LightSource {
    fn scatter_ray(&self, hit: Hit) -> Option<Ray> {
        None
    }

    fn colour(&self, hit: Hit, _future_colour: Vec3Colour) -> Vec3Colour {
        self.colour
    }
}
