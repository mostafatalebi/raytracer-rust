use crate::common::params::Value;
use crate::object::geometry::Geometry;
use crate::object::geometry::GeometrySubType::Sphere;
use crate::object::geometry::GeometryType::Procedural;
use crate::vector::types::Vector;
use crate::vector::vec3f::Vec3f;

pub fn create_procedural_sphere(center: Vec3f, radius: f64) -> Geometry {
    let mut geo = Geometry::default();

    geo.geometry_type = Procedural;
    geo.geometry_subtype = Sphere;
    geo.transform.local.translate = center;
    geo.set_centroid_manually(center);
    geo.data.params.set("radius".to_string(), Value::from_f64(radius));
    geo
}

pub fn get_sphere_normal(intersection_coordinate: &Vec3f, origin: &Vec3f) -> Vec3f {
    return intersection_coordinate.subtract(origin).normalized()
}