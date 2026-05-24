use std::ops::{Index, IndexMut, Mul};
use serde::{Deserialize, Serialize};
use zeroable_vec::Zeroable;
use crate::vector::arithmetic::VectorArithmetic;
use crate::vector::types::{DirVec, Vector};
use crate::vector::utils::Utils;
use crate::vector::vec3f::Vec3f;

// A 2-dimensional position vector holding x,y
#[derive(Default, Deserialize, Serialize, Clone, Debug)]
pub struct Vec2f(pub [f64; 2]);


impl Vec2f {
    pub fn size(&self) -> usize {
        2
    }
    pub fn new(x: f64, y: f64) -> Vec2f {
        Vec2f([x, y])
    }

    // Pos2d can also be used as DirVec,
    // we separate them for the sake of readability
    pub fn get_as_dir_vec(&self) -> DirVec {
        DirVec{origin: self.0[0], direction: self.0[1]}
    }

    fn get_pos_3d(&self) -> Vec3f {
        Vec3f::new(self.0[0], self.0[1], 0.0)
    }


}

impl Mul<f64> for Vec2f {
    type Output = Vec2f;
    fn mul(self, other: f64) -> Vec2f {
        self.multiply_scalar(other)
    }
}


impl Vector for Vec2f {
    fn size(&self) -> usize {
        2
    }

    fn subtract(&self, other: &Self) -> Self {
        VectorArithmetic::subtract(self, other)
    }

    fn add_with(&self, other: &Self) -> Self {
        VectorArithmetic::add(self, other)
    }

    fn multiply_scalar(&self, other: f64) -> Self {
        VectorArithmetic::multiply_scalar(self, other)
    }

    fn divide_by_scalar(&self, other: f64) -> Self {
        VectorArithmetic::divide_by_scalar(self, other)
    }

    fn hat(&self) -> Self {
        Utils::normalize(self)
    }

    fn trunc(&self, num: i64) -> Self {
        Vec2f([f64::trunc(self[0] * num as f64) / num as f64, f64::trunc(self[1] * num as f64) / num as f64])
    }

    fn magnitude(&self) -> f64 {
        f64::sqrt(self[0]*self[0] + self[1]*self[1])
    }

    fn length_squared(&self) -> f64 {
        self[0]*self[0] + self[1]*self[1]
    }

    fn normalized(&self) -> Self {
        Utils::normalize(self)
    }

    fn dot(&self, other: &Self) -> f64 {
        VectorArithmetic::dot(self, other)
    }
}


impl PartialEq for Vec2f {
    fn eq(&self, other: &Self) -> bool {
        if self.0 != other.0 || (self[1] != other[1]) || self[2] != other[2] {
            return false;
        }
        true
    }
}


impl Index<usize> for Vec2f {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl IndexMut<usize> for Vec2f {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}
