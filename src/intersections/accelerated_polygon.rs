use crate::intersections::aabb::AABB;
use crate::intersections::intersection::RenderIntersection;
use crate::intersections::triangle::Triangle;
use crate::*;
use std::path::Path;

#[derive(Debug)]
pub struct AcceleratedPolygon {
    polygon: Polygon,
    bounds: AABB,
}

impl AcceleratedPolygon {
    fn from_triangles(triangles: Vec<Triangle>) -> Self {
        let bounds = AABB::new(&triangles);
        let polygon = Polygon::from_triangles(triangles);
        Self { polygon, bounds }
    }
    pub fn new_from_stl(path: impl AsRef<Path>, scale: f32, offset: Vec3) -> Option<Self> {
        let polygon = Polygon::stl_to_points(path, scale, offset)?;
        Some(Self::from_triangles(polygon))
    }
}

impl RenderIntersection for AcceleratedPolygon {
    fn intersects(&self, ray: Ray) -> Vec<Vec3> {
        if !self.bounds.intersects(ray) {
            return vec![];
        }
        self.polygon.intersects(ray)
    }

    fn normal_at(&self, impact: Vec3) -> Vec3 {
        self.polygon.normal_at(impact)
    }

    fn includes_point_on_surface(&self, point: Vec3) -> bool {
        if !self.bounds.includes(point) {
            return false;
        }
        self.polygon.includes_point_on_surface(point)
    }

    fn uv(&self, at: Vec3) -> Vec2 {
        todo!()
    }
}
