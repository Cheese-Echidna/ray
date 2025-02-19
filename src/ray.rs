use crate::*;
use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub(crate) start: Vec3,
    direction: Vec3,
    pub(crate) current_ior: f32,
}

impl Ray {
    pub(crate) fn direction(&self) -> Vec3 {
        self.direction
    }
    pub(crate) fn start(&self) -> Vec3 {
        self.start
    }
}

impl Ray {
    pub fn new(start: Vec3, direction: Vec3, current_ior: f32) -> Ray {
        Ray {
            start,
            direction: direction.normalize(),
            current_ior,
        }
    }

    pub fn new_from_to(from: Vec3, to: Vec3, current_ior: f32) -> Self {
        Self::new(from, to - from, current_ior)
    }

    pub fn pos_at_length(&self, l: Length) -> Vec3 {
        self.start + self.direction * l
    }
}
