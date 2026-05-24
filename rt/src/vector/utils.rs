use std::ops::{Index, IndexMut, Mul, Sub};
use crate::common::constants::EPS;
use crate::vector::types::{Vec2i, Vector};
use crate::vector::vec2f::Vec2f;

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
    pub fn normalize<T, R>(v: &T) -> T
    where T: Index<usize, Output = R>+IndexMut<usize, Output = R> + Default + Vector + Clone
    {
        let magnitude = v.magnitude();
        if magnitude < EPS {
            return v.clone();
        }
        let mut r = v.multiply_scalar(1f64/magnitude);

        r
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