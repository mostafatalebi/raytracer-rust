use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub};
use serde::{Deserialize, Serialize};
use crate::vector::arithmetic::VectorArithmetic;
use crate::vector::types::Vector;
use crate::vector::utils::Utils;
use crate::vector::vec3f::Vec3f;

#[derive(Default, Debug, Deserialize, Serialize)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Vec4f(pub [f64; 4]);

impl Vec4f {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vec4f {
        Vec4f([x, y, z, w])
    }

    pub fn to_3(&self) -> Vec3f {
        Vec3f::new(self[0], self[1], self[2])
    }
}

impl Vector for Vec4f {
    fn size(&self) -> usize {
        4
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

    fn trunc(&self, num: i64) -> Self {
        Vec4f([f64::trunc(self[0] * num as f64) / num as f64,
            f64::trunc(self[1] * num as f64) / num as f64,
            f64::trunc(self[2] * num as f64) / num as f64,
            f64::trunc(self[3] * num as f64) / num as f64
        ])
    }

    fn hat(&self) -> Self {
        Utils::normalize(self)
    }

    fn magnitude(&self) -> f64 {
        f64::sqrt(self[0]*self[0] + self[1]*self[1]+ self[2]*self[2] + self[3]*self[3])
    }

    fn length_squared(&self) -> f64 {
        self[0]*self[0] + self[1]*self[1] + self[2]*self[2] + self[3]*self[3]
    }

    fn normalized(&self) -> Self {
        Utils::normalize(self)
    }

    fn dot(&self, other: &Self) -> f64 {
        VectorArithmetic::dot(self, other)
    }
}

impl Mul<f64> for Vec4f {
    type Output = Vec4f;
    fn mul(self, other: f64) -> Vec4f {
        self.multiply_scalar(other)
    }
}

impl Mul<f64> for &Vec4f {
    type Output = Vec4f;
    fn mul(self, other: f64) -> Vec4f {
        self.multiply_scalar(other)
    }
}

impl Mul<&Vec4f> for &Vec4f {
    type Output = Vec4f;
    fn mul(self, other: &Vec4f) -> Vec4f {
        VectorArithmetic::comp_wise_mul(self, other)
    }
}

impl Mul<Vec4f> for Vec4f {
    type Output = Vec4f;
    fn mul(self, other: Vec4f) -> Vec4f {
        VectorArithmetic::comp_wise_mul(&self, &other)
    }
}

impl Mul<&Vec4f> for f64 {
    type Output = Vec4f;
    fn mul(self, other: &Vec4f) -> Vec4f {
        other.multiply_scalar(self)
    }
}

impl PartialEq for Vec4f {
    fn eq(&self, other: &Self) -> bool {
        if self.0 != other.0 || (self[1] != other[1]) || self[2] != other[2] {
            return false;
        }
        true
    }
}

impl Index<usize> for Vec4f {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl IndexMut<usize> for Vec4f {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl Sub for &Vec4f {
    type Output = Vec4f;

    fn sub(self, rhs: Self) -> Self::Output {
        self.subtract(rhs)
    }
}

impl Sub for &mut Vec4f {
    type Output = Vec4f;

    fn sub(self, rhs: Self) -> Self::Output {
        self.subtract(rhs)
    }
}

impl Sub<&mut Vec4f> for &Vec4f {
    type Output = Vec4f;

    fn sub(self, rhs: &mut Vec4f) -> Self::Output {
        self.subtract(rhs)
    }
}

impl Sub<&Vec4f> for &mut Vec4f {
    type Output = Vec4f;

    fn sub(self, rhs: &Vec4f) -> Self::Output {
        self.subtract(rhs)
    }
}

impl Sub for Vec4f {
    type Output = Vec4f;

    fn sub(self, rhs: Self) -> Self::Output {
        return self.subtract(&rhs)
    }
}

impl Add for &Vec4f {
    type Output = Vec4f;

    fn add(self, rhs: Self) -> Self::Output {
        return self.add_with(rhs)
    }
}

impl Add for &mut Vec4f {
    type Output = Vec4f;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_with(rhs)
    }
}

impl AddAssign for Vec4f {
    fn add_assign(&mut self, rhs: Vec4f) {
        *self = self.add_with(&rhs)
    }
}

impl Add for Vec4f {
    type Output = Vec4f;

    fn add(self, rhs: Self) -> Self::Output {
        return self.add_with(&rhs)
    }
}