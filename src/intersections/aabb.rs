use crate::intersections::triangle::Triangle;
use crate::Ray;
use glam::Vec3;

#[derive(Debug)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(triangles: &Vec<Triangle>) -> Self {
        let points = triangles
            .iter()
            .map(|x| x.vertices())
            .flatten()
            .collect::<Vec<_>>();

        assert!(!triangles.is_empty());

        let min = points.iter().fold(Vec3::INFINITY, |acc, x| acc.min(*x));

        let max = points.iter().fold(Vec3::NEG_INFINITY, |acc, x| acc.max(*x));

        Self { min, max }
    }

    pub fn intersects(&self, ray: Ray) -> bool {
        // Compute inverse of each ray direction component
        let inv_dir_x = 1.0 / ray.direction().x;
        let inv_dir_y = 1.0 / ray.direction().y;
        let inv_dir_z = 1.0 / ray.direction().z;

        // X slab
        let mut tmin = (self.min.x - ray.start.x) * inv_dir_x;
        let mut tmax = (self.max.x - ray.start.x) * inv_dir_x;
        if inv_dir_x < 0.0 {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        // Y slab
        let mut tymin = (self.min.y - ray.start.y) * inv_dir_y;
        let mut tymax = (self.max.y - ray.start.y) * inv_dir_y;
        if inv_dir_y < 0.0 {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        // If the intervals for x and y do not overlap, there is no intersection.
        if tmin > tymax || tymin > tmax {
            return false;
        }

        // Update tmin and tmax to contain the overlap of x and y intervals.
        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        // Z slab
        let mut tzmin = (self.min.z - ray.start.z) * inv_dir_z;
        let mut tzmax = (self.max.z - ray.start.z) * inv_dir_z;
        if inv_dir_z < 0.0 {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        // Check for overlap with the z interval.
        if tmin > tzmax || tzmin > tmax {
            return false;
        }

        // Optionally update tmin and tmax with z interval (not strictly needed for just a hit test)
        if tzmin > tmin {
            tmin = tzmin;
        }
        if tzmax < tmax {
            tmax = tzmax;
        }

        // If tmax < 0, the entire intersection is behind the ray.
        if tmax < 0.0 {
            return false;
        }

        true
    }

    pub fn includes(&self, point: Vec3) -> bool {
        self.min.cmple(point).all() && point.cmple(self.max).all()
    }
}
