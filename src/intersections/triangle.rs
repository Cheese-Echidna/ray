use crate::utils::{bounce_across_normal, random_cosine_direction};
use crate::*;
use rand::random;
use crate::intersections::intersection::{RenderIntersection, OBJECT_TOLERANCE};
use crate::materials::material::RenderMaterial;

#[derive(Debug)]
pub struct Triangle {
    vertices: [Vec3; 3],
}

impl Triangle {
    pub fn new(vertices: [Vec3; 3]) -> Self {
        // Assume triangles are not degenerate, if they are, the normal() method below will panic
        Self {
            vertices,
        }
    }

    pub(crate) fn normal_raw(&self) -> Vec3 {
        let [a, b, c] = self.vertices;
        let ab = b - a;
        let ac = c - a;
        ab.cross(ac)
    }

    pub(crate) fn vertices(&self) -> [Vec3; 3] {
        self.vertices
    }

    fn normal(&self) -> Vec3 {
        self.normal_raw().normalize()
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
            // if !(self.includes_point(intersection_point)) {
            //     dbg!(&self);
            //     dbg!(intersection_point);
            // };
            // Top tip: If this keeps failing its probably because of degenerate triangles where area is close to 0
            // You could remove this check and the unwraps in the implementation in polygon methods
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

impl RenderIntersection for Triangle {
    fn intersects(&self, ray: Ray) -> Vec<Vec3> {
        self.moller_trumbore_intersection(ray).into_iter().collect()
    }

    fn normal_at(&self, impact: Vec3) -> Vec3 {
        self.normal()
    }

    fn includes_point_on_surface(&self, point: Vec3) -> bool {
        self.includes_point(point)
    }

    fn uv(&self, at: Vec3) -> Vec2 {
        todo!()
    }

    fn uv_derivatives(&self, uv: Vec2) -> (Vec3, Vec3) {
        todo!()
    }
}