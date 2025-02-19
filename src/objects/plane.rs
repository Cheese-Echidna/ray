use std::fmt::{Debug, Formatter};
use palette::Vec3;
use crate::objects::object::RenderObject;
use crate::{Ray, Vec3};
use crate::objects::material::RenderMaterial;
use crate::objects::object;

#[derive(Debug)]
pub struct Plane {
    normal: Vec3,
    a: Vec3,
    material: RenderMaterial,
}

impl Plane {
    pub fn new(normal: Vec3, point: Vec3, material: RenderMaterial) -> Plane {
        let normal = normal.normalize();
        Self {
            normal,
            a: point,
            material,
        }
    }
}


impl RenderObject for Plane {
    /// Return all intersection points (0 or 1) of the ray with this plane.
    fn intersects(&self, ray: Ray) -> Vec<Vec3> {
        let denom = self.normal.dot(ray.direction());

        // If the denominator is near zero, the ray is parallel to the plane
        if denom.abs() < object::OBJECT_TOLERANCE {
            return Vec::new();
        }

        let t = (self.a - ray.start()).dot(self.normal) / denom;

        // For a typical path tracer, we often only consider intersections "in front of" the ray
        // (i.e., t >= 0). If t < 0, the intersection is behind the ray origin.
        if t >= 0.0 {
            vec![ray.start() + ray.direction() * t]
        } else {
            Vec::new()
        }
    }

    /// Return the material of this plane (no dependence on impact point here).
    fn material(&self) -> RenderMaterial {
        self.material
    }

    /// Return the normal at a given impact point on the plane.
    /// (For a plane, the normal is the same everywhere.)
    fn normal_at(&self, _impact: Vec3) -> Vec3 {
        self.normal
    }

    /// Generate a random point on the surface of the plane.
    /// For an infinite plane, this is not well-defined. Implementation below is a placeholder.
    fn random_point_on_surface(&self) -> Vec3 {
        self.a
        // NOT RANDOM!
    }

    /// Check if a given point lies on this plane (within some small epsilon).
    fn includes_point_on_surface(&self, point: Vec3) -> bool {
        let dist = (point - self.a).dot(self.normal).abs();
        dist < object::OBJECT_TOLERANCE
    }
}