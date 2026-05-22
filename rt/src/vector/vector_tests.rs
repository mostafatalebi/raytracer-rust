use std::ops::{Sub};

#[cfg(test)]
mod tests {
    use crate::vector::arithmetic::VectorArithmetic;
    use crate::vector::vec3f::Vec3f;
    use super::*;

    #[test]
    fn vec_util_sub() {
        let r = VectorArithmetic::subtract(&Vec3f::new(2.0, 1.0, 1.0), &Vec3f::new(1.0, 1.0, 1.0));
        assert_eq!(r, Vec3f::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn vec_util_add() {
        let r = VectorArithmetic::add(&Vec3f::new(2.0, 1.0, 4.0), &Vec3f::new(1.0, 1.0, 1.0));
        assert_eq!(r, Vec3f::new(3.0, 2.0, 5.0));
    }

    #[test]
    fn vec_util_multiply_scalar() {
        let r = VectorArithmetic::multiply_scalar(&Vec3f::new(2.0, 1.0, 4.0), 5.0);
        assert_eq!(r, Vec3f::new(10.0, 5.0, 20.0));
    }

    #[test]
    fn vec_util_cross3() {
        let r = VectorArithmetic::cross3(&Vec3f::new(2.0, 1.0, 4.0), &Vec3f::new(3.0, 0.0, 2.0));
        assert_eq!(r, Vec3f::new(2.0, 8.0, -3.0));
    }

}