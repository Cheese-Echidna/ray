use crate::intersections::intersection::RenderIntersection;
use crate::materials::material::RenderMaterial;
use std::fmt::Debug;

#[derive(Debug)]
pub struct RenderObject {
    pub intersector: Box<dyn RenderIntersection>,
    pub material: Box<dyn RenderMaterial>,
}

impl RenderObject {
    pub fn new(
        intersector: impl RenderIntersection + 'static,
        material: impl RenderMaterial + 'static,
    ) -> Self {
        Self {
            intersector: Box::new(intersector) as Box<dyn RenderIntersection>,
            material: Box::new(material) as Box<dyn RenderMaterial>,
        }
    }
    pub fn boxed_new(
        intersector: Box<dyn RenderIntersection>,
        material: Box<dyn RenderMaterial>,
    ) -> Self {
        Self {
            intersector,
            material,
        }
    }
}
