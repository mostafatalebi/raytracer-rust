use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub};
use serde::{Deserialize, Serialize};
use zeroable_vec::Zeroable;
use crate::vector::arithmetic::VectorArithmetic;
use crate::vector::types::{DirVec, Vector};
use crate::vector::utils::Utils;

// a 3-dimensional position vector holding x,y,z
#[derive(Debug, Default, Deserialize, Serialize, Clone, Copy)]
pub struct Vec3f(pub [f64; 3]);

impl Vector for Vec3f {

    fn size(&self) -> usize {
        3
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


    fn trunc(&self, num: i64) -> Self {
        Vec3f([f64::trunc(self[0] * num as f64) / num as f64,
            f64::trunc(self[1] * num as f64) / num as f64,
            f64::trunc(self[2] * num as f64) / num as f64
        ])
    }

    fn hat(&self) -> Self {
        Utils::normalize(self)
    }

    fn magnitude(&self) -> f64 {
        f64::sqrt(self[0]*self[0] + self[1]*self[1] + self[2]*self[2])
    }

    fn length_squared(&self) -> f64 {
        self[0]*self[0] + self[1]*self[1] + self[2]*self[2]
    }

    fn normalized(&self) -> Self {
        Utils::normalize(self)
    }
}

impl Mul<f64> for Vec3f {
    type Output = Vec3f;
    fn mul(self, other: f64) -> Vec3f {
        self.multiply_scalar(other)
    }
}

impl Mul<f64> for &Vec3f {
    type Output = Vec3f;
    fn mul(self, other: f64) -> Vec3f {
        self.multiply_scalar(other)
    }
}

impl Mul<&Vec3f> for &Vec3f {
    type Output = Vec3f;
    fn mul(self, other: &Vec3f) -> Vec3f {
        VectorArithmetic::comp_wise_mul(self, other)
    }
}

impl PartialEq for Vec3f {
    fn eq(&self, other: &Self) -> bool {
        if self.0 != other.0 || (self[1] != other[1]) || self[2] != other[2] {
            return false;
        }
        true
    }
}

impl Index<usize> for Vec3f {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl IndexMut<usize> for Vec3f {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl Vec3f {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3f {
        Vec3f([x, y, z])
    }

    // Pos2d can also be used as DirVec,
    // we separate them for the sake of readability
    pub fn get_as_dir_vec(&self) -> DirVec {
        DirVec{origin: self.0[0], direction: self.0[1]}
    }

    pub fn cross3(&self, other: &Self) -> Self {
        VectorArithmetic::cross3(self, other)
    }

    fn hat(&self) -> Self {
        Utils::normalize(self)
    }
}

impl Sub for &Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: Self) -> Self::Output {
         self.subtract(rhs)
    }
}

impl Sub for &mut Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: Self) -> Self::Output {
        self.subtract(rhs)
    }
}

impl Sub<&mut Vec3f> for &Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: &mut Vec3f) -> Self::Output {
        self.subtract(rhs)
    }
}

impl Sub<&Vec3f> for &mut Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: &Vec3f) -> Self::Output {
        self.subtract(rhs)
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: Self) -> Self::Output {
        return self.subtract(&rhs)
    }
}

impl Add for &Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: Self) -> Self::Output {
        return self.add_with(rhs)
    }
}

impl Add for &mut Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_with(rhs)
    }
}

impl AddAssign for Vec3f {
    fn add_assign(&mut self, rhs: Vec3f) {
        *self = self.add_with(&rhs)
    }
}

impl Add for Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: Self) -> Self::Output {
        return self.add_with(&rhs)
    }
}