use crate::objects::object::OBJECT_TOLERANCE;
use crate::utils::{bounce_across_normal, random_cosine_direction};
use crate::*;
use rand::random;
use crate::objects::material::RenderMaterial;

#[derive(Debug)]
pub struct Triangle {
    vertices: [Vec3; 3],
    material: RenderMaterial
}

impl Triangle {
    pub fn new(vertices: [Vec3; 3], material: RenderMaterial) -> Self {
        // Assume triangles are not degenerate, if they are, the normal() method below will panic
        Self {
            vertices,
            material,
        }
    }
    fn normal(&self) -> Vec3 {
        let [a, b, c] = self.vertices;
        let ab = b - a;
        let ac = c - a;
        let normal = ab.cross(ac).normalize();
        normal
    }

    // https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    fn moller_trumbore_intersection(&self, ray: Ray) -> Option<Vec3> {
        let (origin, direction) = (ray.start, ray.direction());
        let [tri_a, tri_b, tri_c] = self.vertices;
        let e1 = tri_b - tri_a;
        let e2 = tri_c - tri_a;

        let ray_cross_e2 = direction.cross(e2);
        let det = e1.dot(ray_cross_e2);

        if det > -f32::EPSILON && det < f32::EPSILON {
            return None; // This ray is parallel to this triangle.
        }

        let inv_det = 1.0 / det;
        let s = origin - tri_a;
        let u = inv_det * s.dot(ray_cross_e2);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let s_cross_e1 = s.cross(e1);
        let v = inv_det * direction.dot(s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = inv_det * e2.dot(s_cross_e1);

        if t > f32::EPSILON {
            // ray intersection
            let intersection_point = origin + direction * t;
            assert!(self.includes_point(intersection_point));
            Some(intersection_point)
        } else {
            // This means that there is a line intersection but not a ray intersection.
            None
        }
    }

    pub fn get_area_diff_point(&self, point: Vec3) -> Option<f32> {
        let [a, b, c] = self.vertices;
        let ab = b - a;
        let ac = c - a;
        let normal = ab.cross(ac);

        let dist_from_plane = normal.dot(point - a).abs();
        if dist_from_plane > 0.0001 {
            return None;
        }

        // 3. Compute the area of the original triangle ABC.
        //    The area of a triangle can be computed as half the length of the cross product.
        let area_abc = normal.length() * 0.5;

        // 4. Compute the area of the sub-triangles PAB, PBC, and PCA.
        let area_pab = (a - point).cross(b - point).length() * 0.5;
        let area_pbc = (b - point).cross(c - point).length() * 0.5;
        let area_pca = (c - point).cross(a - point).length() * 0.5;

        // 5. If the sum of the sub-triangle areas matches the original triangle's area
        //    (within `epsilon`), then `point` is inside or on the boundary of the triangle.
        let area_sum = area_pab + area_pbc + area_pca;
        Some((area_sum - area_abc).abs())
    }

    pub fn includes_point(&self, point: Vec3) -> bool {
        self.get_area_diff_point(point)
            .is_some_and(|x| x < OBJECT_TOLERANCE)
    }
}

impl RenderObject for Triangle {
    fn intersects(&self, ray: Ray) -> Vec<Vec3> {
        self.moller_trumbore_intersection(ray).into_iter().collect()
    }

    fn material(&self) -> RenderMaterial {
        self.material
    }

    fn normal_at(&self, impact: Vec3) -> Vec3 {
        self.normal()
    }

    fn random_point_on_surface(&self) -> Vec3 {
        let [a, b, c] = self.vertices;
        let ab = b - a;
        let ac = c - a;
        let (mut u, mut v): (f32, f32) = (random(), random());
        if u + v > 1. {
            (u, v) = (1. - u, 1. - v);
        }

        let point = a + ab * u + ac * v;

        assert!(self.includes_point_on_surface(point));
        point
    }

    fn includes_point_on_surface(&self, point: Vec3) -> bool {
        self.includes_point(point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_point_on_triangle() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(1.0, 0.0, 0.0);
        let c = Vec3::new(0.0, 1.0, 0.0);

        let triangle = Triangle::new([a, b, c], RenderMaterial::new_void());

        // A point clearly inside the triangle
        let p1 = Vec3::new(0.25, 0.25, 0.0);
        assert!(triangle.includes_point(p1));

        // A point on an edge
        let p2 = Vec3::new(0.5, 0.0, 0.0);
        assert!(triangle.includes_point(p2));

        // A point not in the plane (z != 0)
        let p3 = Vec3::new(0.25, 0.25, 1.0);
        assert!(!triangle.includes_point(p3));

        // A point out of bounds
        let p4 = Vec3::new(1.0, 1.0, 0.0);
        assert!(!triangle.includes_point(p4));
    }
}
