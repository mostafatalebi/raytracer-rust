use std::ops::Sub;
use serde::{Deserialize, Serialize};
use crate::common::volume::Centroid;
use crate::vector::vec3f::Vec3f;

/// Axis Aligned Bounding Box
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct AABB {
    pub min: Vec3f,
    pub max: Vec3f
}

impl AABB {

    pub fn new(min: Vec3f, max: Vec3f) -> AABB {
        Self {
            min, max,
        }
    }

    pub fn calc_bounding_volume_for_polygons(vertices: &Vec<Vec3f>) -> Self {
        let mut min = vertices[0];
        let mut max = vertices[0];

        for (k, vertex) in vertices.iter().enumerate() {
            min[0] = min[0].min(vertex[0]);
            min[1] = min[1].min(vertex[1]);
            min[2] = min[2].min(vertex[2]);

            max[0] = max[0].max(vertex[0]);
            max[1] = max[1].max(vertex[1]);
            max[2] = max[2].max(vertex[2]);
        }

        Self {min, max}
    }

    pub fn calc_bounding_volume_for_proc_sphere(center: &Vec3f, radius: f64) -> Self {
        let min = center - radius;
        let max = center + radius;


        Self {min, max }
    }

    pub fn get_longest_axis(&self) -> usize {
        let dx = self.max[0] - self.min[0];
        let dy = self.max[1] - self.min[1];
        let dz = self.max[2] - self.min[2];
        if dx > dy && dy > dz {
            return 0;
        } else if dy > dz {
            return 1;
        }
        2
    }

    pub fn expand(&self, b: &Self) -> Self {
        AABB {
                min: Vec3f::new(self.min[0].min(b.min[0]),
                                self.min[1].min(b.min[1]),
                                self.min[2].min(b.min[2])),
                max: Vec3f::new(self.max[0].max(b.max[0]),
                                self.max[1].max(b.max[1]),
                                self.max[2].max(b.max[2])),

        }
    }

    pub fn get_min_max(&self) -> (Vec3f, Vec3f) {
        (self.min, self.max)
    }
}

pub trait Bounded {
    fn get_bb(&self) -> AABB;
}


impl Centroid for AABB {
    fn get_centroid(&self) -> Vec3f {
        Vec3f::new((self.max[0] - self.min[0])*0.5,
                   (self.max[1] - self.min[1])*0.5,
                   (self.max[2] - self.min[2])*0.5)
    }
}