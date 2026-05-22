use std::ops::{Add, Index, IndexMut, Mul, Sub};
use crate::vector::types::Vector;

pub struct VectorArithmetic {

}

impl VectorArithmetic {
    pub fn subtract<V, T>(a: &V, b: &V) -> V
    where V: Index<usize, Output = T> + IndexMut<usize> + Default + Vector,
          T: Sub<Output=T> + Copy {
        let mut r = V::default();
        let a_len = a.size();
        let b_len = b.size();
        if a_len == b_len {
            let mut i = 0;
            while i < a_len {
                r[i] = a[i] - b[i];
                i += 1;
            }
        }
        r
    }

    pub fn add<V, T>(a: &V, b: &V) -> V
    where V: Index<usize, Output = T> + IndexMut<usize> + Default + Vector,
          T: Add<Output=T> + Copy {
        let mut r = V::default();
        let a_len = a.size();
        let b_len = b.size();
        if a_len == b_len {
            let mut i = 0;
            while i < a_len {
                r[i] = a[i] + b[i];
                i += 1;
            }
        }
        r
    }

    pub fn multiply_scalar<V, T>(a: &V, b: T) -> V
    where V: Index<usize, Output = T> + IndexMut<usize> + Default + Vector,
          T: Mul<Output=T> + Copy {
        let mut r = V::default();
        let a_len = a.size();
        let mut i = 0;
        while i < a_len {
            r[i] = a[i] * b;
            i += 1;
        }
        r
    }

    pub fn comp_wise_mul<V, T>(a: &V, b: &V) -> V
    where V: Index<usize, Output = T> + IndexMut<usize> + Default + Vector, T: Mul<Output = T> + Copy {
        let mut r = V::default();
        let a_len = a.size();
        let mut i = 0;
        while i < a_len {
            r[i] = a[i] * b[i];
            i += 1;
        }
        r
    }

    pub fn cross3<V, T>(a: &V, b: &V) -> V
    where V: Index<usize, Output = T> + IndexMut<usize> + Default + Vector,
          T: Sub<Output=T> + Mul<Output=T> + Copy {

        let mut r = V::default();

        let a_len = a.size();
        if a_len == 3 {
            r[0] = a[1]*b[2] - a[2]*b[1];
            r[1] = a[2]*b[0] - a[0]*b[2];
            r[2] = a[0]*b[1] - a[1]*b[0];
        }
        r
    }


    // dot product of two vectors
    pub fn dot<T>(a: &T, b: &T) -> f64
    where T: Index<usize, Output = f64> + IndexMut<usize, Output=f64> + Default + Vector {
        let a_len = a.size();
        let mut i = 0;
        let mut sum = 0f64;
        while i < a_len {
            let s: f64 = a[i] * b[i];
            sum = sum + s;
            i += 1;
        }
        sum
    }

    pub fn distance<T>(a: &T, b: &T) -> f64
    where T: Index<usize, Output = f64> + IndexMut<usize, Output=f64> + Default + Vector {
        let a_len = a.size();
        let mut i = 0;
        let mut r = 0f64;
        while i < a_len {
            r = r + (b[i]-a[i])*(b[i]-a[i]);
            i += 1;
        }
        r.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::vec3f::Vec3f;
    use super::*;
    #[test]
    fn test_cross3() {
        let a = Vec3f([1.0, 2.0, 3.0]);
        let b = Vec3f([4.0, 5.0, 6.0]);

        let c = VectorArithmetic::cross3(&a, &b);

        assert_eq!(c, Vec3f([-3.0, 6.0, -3.0]));
    }

    #[test]
    fn test_component_wise_vector_mul() {
        let a = Vec3f([1.0, 2.0, 3.0]);
        let b = Vec3f([4.0, 5.0, 6.0]);
        let c = Vec3f([4.0, 10.0, 18.0]);

        assert_eq!(c, VectorArithmetic::comp_wise_mul(&a, &b))
    }
}