use crate::intersections::intersection::{RenderIntersection, OBJECT_TOLERANCE};
use crate::utils::{build_orthonormal_basis, scalar_projection};
use crate::{Ray, Vec2, Vec3};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Plane {
    normal: Vec3,
    centre: Vec3,
}

impl Plane {
    pub fn new(normal: Vec3, centre: Vec3) -> Plane {
        let normal = normal.normalize();
        Self { normal, centre }
    }
}

impl RenderIntersection for Plane {
    /// Return all intersection points (0 or 1) of the ray with this plane.
    fn intersects(&self, ray: Ray) -> Vec<Vec3> {
        let denom = self.normal.dot(ray.direction());

        // If the denominator is near zero, the ray is parallel to the plane
        if denom.abs() < OBJECT_TOLERANCE {
            return Vec::new();
        }

        let t = (self.centre - ray.start()).dot(self.normal) / denom;

        // For a typical path tracer, we often only consider intersections "in front of" the ray
        // (i.e., t >= 0). If t < 0, the intersection is behind the ray origin.
        if t >= 0.0 {
            vec![ray.start() + ray.direction() * t]
        } else {
            Vec::new()
        }
    }

    /// Return the normal at a given impact point on the plane.
    /// (For a plane, the normal is the same everywhere.)
    fn normal_at(&self, _impact: Vec3) -> Vec3 {
        self.normal
    }

    /// Check if a given point lies on this plane (within some small epsilon).
    fn includes_point_on_surface(&self, point: Vec3) -> bool {
        let dist = (point - self.centre).dot(self.normal).abs();
        dist < OBJECT_TOLERANCE
    }

    fn uv(&self, at: Vec3) -> Vec2 {
        let from_center = at - self.centre;
        assert!(from_center.dot(self.normal) < OBJECT_TOLERANCE);
        let (x, y, _n) = build_orthonormal_basis(self.normal);
        Vec2::new(
            scalar_projection(from_center, x),
            scalar_projection(from_center, y),
        )
    }
}
