use std::ops::{Div, Index, IndexMut, Mul, Sub};
use std::process::Output;
use crate::common::constants::EPS;
use crate::common::types::ToF64;
use crate::vector::types::{Vec2i, Vec3i, Vector};
use crate::vector::vec2f::Vec2f;
use crate::vector::vec3f::Vec3f;

pub struct Utils {}


impl Utils {

    // given the sensor size (35mm) and a focal lens,
    // it calculates horizontal and vertical FOV in radians
    // For each horizontal and vertical:
    // fov = 2 x arctan(sensor_size/2 x focal_length)
    // Results are in radian up to three decimal places.
    pub fn calc_fov(sensor: &Vec2i, focal_length: &f64) -> Vec2f {
        Vec2f::new(
            2.0 * ((sensor[0] as f64 / (2.0 * focal_length)).atan()),
            2.0 * ((sensor[1] as f64 / (2.0 * focal_length)).atan()),
        )
    }

    // converts a vector into a unit vector
    pub fn normalize<V, R>(v: &V) -> V
    where V: Index<usize, Output = R>+IndexMut<usize, Output = R> + Default + Vector<f64> + Clone,
          R: ToF64 + Copy
    {
        let magnitude = v.magnitude();
        if magnitude < EPS {
            return v.clone();
        }

        let mut r = v.multiply_scalar(1f64/magnitude);

        r
    }


    pub fn calc_vertices_normal(bc_u: f64, bc_v: f64, normals: &Vec<Vec3f>) -> Vec3f {
        let w = 1.0 - bc_u - bc_v;

        ((normals[0] * w) + (normals[1] * bc_u) + (normals[2] * bc_v)).normalized()
    }

}



#[cfg(test)]
mod tests {
    use crate::vector::types::SENSOR_SIZE_35;
    use crate::vector::vec3f::Vec3f;
    use super::*;

    #[test]
    fn test_utils_calc_fov(){
        let sensor = SENSOR_SIZE_35;
        let focal_length = 50.0;

        let fov = Utils::calc_fov(&sensor, &focal_length);
        assert_eq!(fov[0], 0.691);
        assert_eq!(fov[1], 0.471);
    }

    #[test]
    fn test_utils_normalize_f() {
        let normalized = Utils::normalize(&Vec3f::new(3.0, 0.0, 4.0));
        assert_eq!(normalized, Vec3f::new(0.6, 0.0, 0.8));
    }

}