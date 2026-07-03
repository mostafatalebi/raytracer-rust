use std::ops::{Index, IndexMut};
use serde::{Deserialize, Serialize};
use crate::common::id::Id;
use crate::vector::arithmetic::VectorArithmetic;
use crate::vector::types::Vector;
use crate::vector::vec3f::Vec3f;

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Vec3i(pub [i64; 3]);

impl Vec3i {
    pub fn new(x: i64, y: i64, z:i64) -> Vec3i {
        Vec3i([x, y, z])
    }

    pub fn get_as_float(&self) -> Vec3f {
        Vec3f::new(self[0] as f64, self[1] as f64, self[2] as f64)
    }

    pub fn reverse(&mut self) -> Self {
        self.0.reverse();
        self.clone()
    }

    pub fn reset(&mut self) {
        self[0] = 0;
        self[1] = 0;
        self[2] = 0;
    }
}

impl Index<usize> for Vec3i {
    type Output = i64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl IndexMut<usize> for Vec3i {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl Vector<i64> for Vec3i {
    fn size(&self) -> usize {
        2
    }

    fn subtract(&self, other: &Self) -> Self {
        VectorArithmetic::subtract(self, other)
    }

    fn add_with(&self, other: &Self) -> Self {
        VectorArithmetic::add(self, other)
    }

    fn multiply_scalar(&self, other: i64) -> Self {
        VectorArithmetic::multiply_scalar(self, other)
    }

    fn divide_by_scalar(&self, other: i64) -> Self {
        VectorArithmetic::divide_by_scalar(self, other)
    }

    fn trunc(&self, num: i64) -> Self {
        self.clone()
    }

    fn hat(&self) -> Self {
        // not implementable
        Vec3i::new(0,0, 0)
    }

    fn magnitude(&self) -> f64 {
        f64::sqrt((self[0] * self[0] + self[1] * self[1]) as f64)
    }

    fn length_squared(&self) -> f64 {
        (self[0]*self[0] + self[1]*self[1] + self[2]*self[2]) as f64
    }

    fn normalized(&self) -> Self {
        // not implemented
        self.clone()
    }

    fn dot(&self, other: &Self) -> f64 {
        VectorArithmetic::dot(self, other)
    }

    fn clamp(&mut self, min: i64, max: i64) {
        self[0] = self[0].clamp(min, max);
        self[1] = self[1].clamp(min, max);
        self[2] = self[1].clamp(min, max);
    }
}

impl PartialEq for Vec3i {
    fn eq(&self, other: &Self) -> bool {
        if self.0 != other.0 || (self[1] != other[1]) || self[2] != other[2] {
            return false;
        }
        true
    }
}

impl From<Vec<i64>> for Vec3i {
    fn from(v: Vec<i64>) -> Self {
        Vec3i::new(v[0], v[1], v[2])
    }
}

impl Id for Vec3i {
    fn get_id(&self) -> String {
        format!("face[{:?},{:?},{:?}]", self[0], self[1], self[2])
    }
}