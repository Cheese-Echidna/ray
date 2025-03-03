use crate::intersections::intersection::{RenderIntersection, OBJECT_TOLERANCE};
use crate::*;
use std::path::Path;
use tinystl::StlData;
use crate::intersections::triangle::Triangle;

#[derive(Debug)]
pub struct Polygon {
    triangles: Vec<Triangle>,
}

impl Polygon {
    pub fn new(triangles: Vec<[Vec3; 3]>) -> Polygon {
        let triangles = triangles
            .into_iter()
            .map(|tri| Triangle::new(tri))
            .collect();
        Self { triangles }
    }

    pub fn from_triangles(triangles: Vec<Triangle>) -> Self {
        Self {
            triangles,
        }
    }

    pub fn new_from_vertices_and_indies(vertices: Vec<Vec3>, indices: Vec<[usize; 3]>) -> Polygon {
        let triangles = indices
            .into_iter()
            .map(|tri_index| Triangle::new(tri_index.map(|x| vertices[x])))
            .collect();
        Self { triangles }
    }

    pub fn new_from_polygons(polygons: Vec<Polygon>) -> Self {
        Self {
            triangles: polygons
                .into_iter()
                .map(|x| x.triangles)
                .flatten()
                .collect(),
        }
    }

    pub fn new_square(
        center: Vec3,
        along_plane_1: Vec3,
        along_plane_2: Vec3,
        width: f32,
    ) -> Polygon {
        let corners = [(-1., 1.), (-1., -1.), (1., -1.), (1., 1.)]
            .iter()
            .map(|(a, b)| width * (along_plane_1 * a + along_plane_2 * b) + center)
            .collect();
        Self::new_from_vertices_and_indies(corners, vec![[0, 1, 2], [0, 2, 3]])
    }

    pub fn stl_to_points(path: impl AsRef<Path>, scale: f32, offset: Vec3) -> Option<Vec<Triangle>> {
        let data = StlData::read_from_file(path).ok()?;

        let f = |x| Vec3::from_array(x) * scale + offset;
        let triangles = data
            .triangles
            .into_iter()
            .map(|x| [f(x.v1), f(x.v2), f(x.v3)])
            .map(|x| Triangle::new(x))
            .filter(|x| x.normal_raw().length() * 0.5 > 0.01)
            .collect::<Vec<_>>();
        Some(triangles)
    }

    pub fn new_from_stl(path: impl AsRef<Path>, scale: f32, offset: Vec3) -> Option<Self> {
        let triangles = Self::stl_to_points(path, scale, offset)?;
        Some(Self::from_triangles(triangles))
    }
}

impl RenderIntersection for Polygon {
    fn intersects(&self, ray: Ray) -> Vec<Vec3> {
        self.triangles
            .iter()
            .map(|triangle| triangle.intersects(ray))
            .flatten()
            .collect()
    }

    fn normal_at(&self, impact: Vec3) -> Vec3 {
        self.triangles
            .iter()
            .find(|triangle| triangle.includes_point(impact))
            // TODO: FIX vvvvv - Seriously problematic
            .unwrap_or(&self.triangles[0])
            .normal_at(impact)
    }

    fn includes_point_on_surface(&self, point: Vec3) -> bool {
        self.triangles.iter().any(|x| x.includes_point(point))
    }

    fn uv(&self, at: Vec3) -> Vec2 {
        todo!()
    }
}
