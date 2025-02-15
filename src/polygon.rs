use crate::triangle::Triangle;
use crate::*;
use rand::Rng;

#[derive(Debug)]
pub struct Polygon {
    triangles: Vec<Triangle>,
}

impl Polygon {
    pub fn new(
        triangles: Vec<[Vec3; 3]>,
        colour: LinSrgb,
        emissivity: f32,
        roughness: f64,
    ) -> Polygon {
        let triangles = triangles
            .into_iter()
            .map(|tri| Triangle::new(tri, colour, emissivity, roughness))
            .collect();
        Self { triangles }
    }

    pub fn new_from_vertices_and_indies(
        vertices: Vec<Vec3>,
        indices: Vec<[usize; 3]>,
        colour: LinSrgb,
        emissivity: f32,
        roughness: f64,
    ) -> Polygon {
        let triangles = indices
            .into_iter()
            .map(|tri_index| {
                let verts = tri_index.map(|x| vertices[x]);
                Triangle::new(verts, colour, emissivity, roughness)
            })
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
        width: f64,
        colour: LinSrgb,
        emissivity: f32,
        roughness: f64,
    ) -> Polygon {
        let corners = [(-1., 1.), (-1., -1.), (1., -1.), (1., 1.)]
            .iter()
            .map(|(a, b)| width * (along_plane_1 * a + along_plane_2 * b) + center)
            .collect();
        Self::new_from_vertices_and_indies(
            corners,
            vec![[0, 1, 2], [0, 2, 3]],
            colour,
            emissivity,
            roughness,
        )
    }
}

impl RenderObject for Polygon {
    fn intersects(&self, ray: Ray) -> Vec<Vec3> {
        self.triangles
            .iter()
            .map(|triangle| triangle.intersects(ray))
            .flatten()
            .collect()
    }

    fn attenuation_colour(&self, impact: DVec3, direction: DVec3) -> LinSrgb {
        self.triangles
            .iter()
            .find(|x| x.includes_point(impact))
            .unwrap()
            .attenuation_colour(impact, direction)
    }

    fn scatter_ray(&self, impact: DVec3, direction: DVec3) -> Ray {
        self.triangles
            .iter()
            .find(|x| x.includes_point(impact))
            .unwrap()
            .scatter_ray(impact, direction)
    }

    fn emission(&self, impact: Vec3, direction: Vec3) -> LinSrgb {
        self.triangles
            .iter()
            .find(|x| x.includes_point(impact))
            .unwrap()
            .emission(impact, direction)
        // unwrap is scary but should never fail because intersects is already true
    }

    fn random_point_on_surface(&self) -> DVec3 {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..self.triangles.len());
        let point = self.triangles[i].random_point_on_surface();
        assert!(self.triangles[i].includes_point_on_surface(point));
        point
    }

    fn includes_point_on_surface(&self, point: DVec3) -> bool {
        self.triangles.iter().any(|x| x.includes_point(point))
    }
}
