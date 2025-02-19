use crate::intersections::intersection::{RenderIntersection, OBJECT_TOLERANCE};
use crate::utils::{bounce_across_normal, random_cosine_direction};
use crate::*;
use rand::random;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub centre: Vec3,
    pub radius: Length,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: Length) -> Sphere {
        Sphere { centre, radius }
    }

    fn private_intersects(&self, ray: Ray) -> Vec<Length> {
        // Offset - the position of the sphere relative to the start of the ray
        let os = ray.start() - &self.centre;

        // Calculate a,b,c so we can plug them into the quadratic formula. Except
        // a should be the squared Euclidean distance of the ray direction,
        // but ray directions are normalised to a unit vector, so a will be 1,
        // so we can ignore it.
        let b = 2.0 * os.dot(ray.direction());
        let c = os.length_squared() - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * c;

        let intersections = if discriminant < 0.0 {
            vec![]
        } else if discriminant.abs() <= OBJECT_TOLERANCE {
            vec![-b / 2.0]
        } else {
            let root = discriminant.sqrt();
            vec![(-b - root) / 2.0, (-b + root) / 2.0]
        };
        intersections.into_iter().filter(|x| *x >= 0.0).collect()
    }
}

impl RenderIntersection for Sphere {
    fn intersects(&self, ray: Ray) -> Vec<Vec3> {
        self.private_intersects(ray)
            .into_iter()
            .map(|x| ray.pos_at_length(x))
            .collect()
    }
    fn normal_at(&self, point: Vec3) -> Vec3 {
        (point - self.centre).normalize()
    }

    fn includes_point_on_surface(&self, point: Vec3) -> bool {
        (self.centre.distance(point) - self.radius).abs() <= OBJECT_TOLERANCE
    }

    fn uv(&self, at: Vec3) -> Vec2 {
        todo!()
    }
}
