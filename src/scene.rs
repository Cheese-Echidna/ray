use crate::*;
use crate::lights::LightSource;
use crate::object::RenderObject;

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub background: Srgb,
    pub shapes: Vec<Box<dyn RenderObject>>,
    pub lights: Vec<LightSource>
}

impl Scene {
    pub fn new(camera: Camera, background:Srgb, shapes: Vec<Box<dyn RenderObject>>) -> Scene {
        Scene {camera, background, shapes, lights: vec![LightSource::new(Vec3::new(-2.0, 2., 2.), 0.7)] }
    }

    pub fn trace(&self, image_prop: Vec2) -> Srgb {
        self.camera.trace(self, image_prop).unwrap_or(self.background)
    }
}

pub fn bounce_across_normal(incoming: Vec3, normal: Vec3) -> Vec3 {
    (incoming - 2.0 * incoming.dot(normal) * normal).normalize()
}