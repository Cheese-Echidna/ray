use crate::*;
use glam::DVec3;
use crate::lights::LightSource;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    start: Vec3,
    direction: Vec3,
}

impl Ray {
    pub(crate) fn direction(&self) -> DVec3 {
        self.direction
    }
    pub(crate) fn start(&self) -> Vec3 {
        self.start
    }
}

impl Ray {
    pub fn new(start: Vec3, direction: Vec3) -> Ray {
        Ray {
            start,
            direction: direction.normalize(),
        }
    }

    // this shit needs optimising - O(n) deez nuts
    // it also needs cleaning
    // and there's the code duplication with trace_light_sources
    // todo: improve
    pub fn trace<'a>(&self, scene: &'a Scene, min_distance: f64) -> Option<(&'a Box<dyn RenderObject>, Vec3)> {
        scene
            .shapes
            .iter()
            .filter_map(|shape| shape.intersects(*self).map(|x| (shape, x)))
            .map(|(object, intersection)| (object, intersection, intersection.distance(self.start)))
            .filter(|(_, _, dist)| *dist >= min_distance)
            .min_by(|(_, _, x_dist), (_, _, y_dist)| x_dist.total_cmp(y_dist))
            .map(|(a, b, c)| (a, b))
    }

    pub fn trace_light_sources<'a>(&self, scene: &'a Scene, min_distance: f64) -> Option<(&'a LightSource, Vec3)> {
        scene
            .lights
            .iter()
            .filter_map(|shape| shape.intersects(*self).map(|x| (shape, x)))
            .map(|(object, intersection)| (object, intersection, intersection.distance(self.start)))
            .filter(|(_, _, dist)| *dist >= min_distance)
            .min_by(|(_, _, x_dist), (_, _, y_dist)| x_dist.total_cmp(y_dist))
            .map(|(a, b, c)| (a, b))
    }

    pub fn pos_at_length(&self, l: Length) -> Vec3 {
        self.start + self.direction * l
    }
}
