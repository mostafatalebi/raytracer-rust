use std::ops::{Index, IndexMut};
use crate::vector::arithmetic::VectorArithmetic;
use crate::vector::types::Vector;
use crate::vector::vec3f::Vec3f;
use crate::vector::vec4f::Vec4f;

pub type NColor3 = Vec3f;

pub struct Color {

}

impl Color {
    pub fn n_to_r<T>(color: &T) -> T
        where T: Vector + Index<usize, Output = f64> + IndexMut<usize, Output=f64> + Default + Copy {
        let v = color.multiply_scalar(255.0);
        v
    }

    pub fn r_to_n<T>(color: &T) -> T
    where T: Vector + Index<usize, Output = f64> + IndexMut<usize, Output=f64> + Default + Copy {
        let v = color.divide_by_scalar(255.0);
        v
    }

    /// works only and only when the underlying type is Vec4f;
    /// it then applies the 4th index to each other element
    pub fn apply_alpha<T>(color: &T) -> T
    where T: Vector + Index<usize, Output = T> + IndexMut<usize, Output=f64> + Default + Copy {
        let v = color.divide_by_scalar(255.0);
        v
    }

    pub fn n_clamp<T>(color: &T) -> T
    where T: Vector + Index<usize, Output = f64> + IndexMut<usize, Output=f64> + Default + Copy {
        VectorArithmetic::clamp(color, 0.0, 1.0)
    }
    pub fn r_clamp<T>(color: &T) -> T
    where T: Vector + Index<usize, Output = f64> + IndexMut<usize, Output=f64> + Default + Copy {
        VectorArithmetic::clamp(color, 0.0, 255.0)
    }
}