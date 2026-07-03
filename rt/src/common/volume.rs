use crate::bounding_box::aabb::AABB;
use crate::vector::vec3f::Vec3f;

pub trait Centroid {
    fn get_centroid(&self) -> Vec3f;
}



