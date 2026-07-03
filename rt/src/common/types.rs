use crate::bounding_box::aabb::{Bounded, AABB};
use crate::common::id::Id;
use crate::common::volume::Centroid;
use crate::vector::types::Vector;
use crate::vector::vec3f::Vec3f;
use crate::vector::vec3i::Vec3i;

// used for values form 0.0 to 1.0
pub type NormalizedF = f64;

pub trait ToF64 {
    fn to_f64(&self) -> f64;
}

impl ToF64 for f64 {
    fn to_f64(&self) -> f64 { *self }
}

impl ToF64 for f32 {
    fn to_f64(&self) -> f64 { *self as f64 }
}

impl ToF64 for i64 {
    fn to_f64(&self) -> f64 { *self as f64 }
}

impl ToF64 for i32 {
    fn to_f64(&self) -> f64 { *self as f64 }
}


pub struct EnrichedFace(pub Vec3i, pub [Vec3f; 3]);


impl Centroid for EnrichedFace {
    fn get_centroid(&self) -> Vec3f {
        self.1.iter().sum::<Vec3f>().divide_by_scalar(3.0)
    }
}

impl Bounded for EnrichedFace {
    fn get_bb(&self) -> AABB {
        let min = self.1[0].min(&self.1[1]).min(&self.1[2]);
        let max = self.1[0].max(&self.1[1]).max(&self.1[2]);
        AABB::new(min, max)
    }
}

impl Id for EnrichedFace {
    fn get_id(&self) -> String {
        format!("face[{:?},{:?},{:?}]", self.0[0], self.0[1], self.0[2])
    }
}