use std::iter::Sum;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::common::constants::EPS;
use crate::error::error::SysError;
use crate::error::kinds::ErrorKind;
use crate::vector::arithmetic::VectorArithmetic;
use crate::vector::types::{DirVec, Vector};
use crate::vector::utils::Utils;
use crate::vector::vec4f::Vec4f;

// a 3-dimensional position vector holding x,y,z
#[derive(Debug, Default, Deserialize, Serialize, Clone, Copy)]
pub struct Vec3f(pub [f64; 3]);

impl Vector<f64> for Vec3f {

    fn size(&self) -> usize {
        3
    }

    #[inline(always)]
    fn subtract(&self, other: &Self) -> Self {
        VectorArithmetic::subtract(self, other)
    }

    #[inline(always)]
    fn add_with(&self, other: &Self) -> Self {
        VectorArithmetic::add(self, other)
    }

    #[inline(always)]
    fn multiply_scalar(&self, other: f64) -> Self {
        VectorArithmetic::multiply_scalar(self, other)
    }

    #[inline(always)]
    fn divide_by_scalar(&self, other: f64) -> Self {
        VectorArithmetic::divide_by_scalar(self, other)
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

    #[inline(always)]
    fn normalized(&self) -> Self {
        let magnitude = self.magnitude();
        if magnitude < EPS {
            return self.clone();
        }

        let mut r = self * (1f64/magnitude);

        r
    }

    fn dot(&self, other: &Self) -> f64 {
        VectorArithmetic::dot(self, other)
    }

    fn clamp(&mut self, min: f64, max: f64) {
        self[0] = self[0].clamp(min, max);
        self[1] = self[1].clamp(min, max);
        self[2] = self[1].clamp(min, max);
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

    #[inline(always)]
    pub fn cross3(&self, rhs: &Self) -> Self {
        Vec3f([
            self[1]* rhs[2] - self[2]* rhs[1],
            self[2]* rhs[0] - self[0]* rhs[2],
            self[0]* rhs[1] - self[1] * rhs[0]
        ])
    }

    fn hat(&self) -> Self {
        Utils::normalize(self)
    }

    pub fn to_4(&self) -> Vec4f {
        Vec4f::new(self[0], self[1], self[2], 1.0)
    }

    pub fn create_by<F: Fn() -> f64>(f: F) -> Vec3f {
        Vec3f::new(f(), f(), f())
    }

    pub fn reset(&mut self) {
        self[0] = 0.0;
        self[1] = 0.0;
        self[2] = 0.0;
    }

    pub fn min(&self, another: &Vec3f) -> Vec3f {
        Vec3f::new(self[0].min(another[0]), self[1].min(another[1]), self[2].min(another[2]))
    }
    pub fn max(&self, another: &Vec3f) -> Vec3f {
        Vec3f::new(self[0].max(another[0]), self[1].max(another[1]), self[2].max(another[2]))
    }
}

impl Mul<f64> for Vec3f {
    type Output = Vec3f;
    #[inline(always)]
    fn mul(self, rhs: f64) -> Vec3f {
        Vec3f([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
        ])
    }
}

impl Mul<f64> for &Vec3f {
    type Output = Vec3f;
    #[inline(always)]
    fn mul(self, rhs: f64) -> Vec3f {
        Vec3f([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
        ])
    }
}

impl Mul<&Vec3f> for f64 {
    type Output = Vec3f;
    #[inline(always)]
    fn mul(self, rhs: &Vec3f) -> Vec3f {
        Vec3f([
            self * rhs[0],
            self * rhs[1],
            self * rhs[2],
        ])
    }
}

impl Mul<&Vec3f> for &Vec3f {
    type Output = Vec3f;
    #[inline(always)]
    fn mul(self, rhs: &Vec3f) -> Vec3f {
        Vec3f([
            self[0] * rhs[0],
            self[1] * rhs[1],
            self[2] * rhs[2],
        ])
    }
}

impl Mul<Vec3f> for Vec3f {
    type Output = Vec3f;
    #[inline(always)]
    fn mul(self, rhs: Vec3f) -> Vec3f {
        Vec3f([
            self[0] * rhs[0],
            self[1] * rhs[1],
            self[2] * rhs[2],
        ])
    }
}

impl Add for &Vec3f {
    type Output = Vec3f;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3f([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

impl Add for &mut Vec3f {
    type Output = Vec3f;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3f([
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
        ])
    }
}

impl AddAssign for Vec3f {

    #[inline(always)]
    fn add_assign(&mut self, rhs: Vec3f) {
        *self = Vec3f([
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
        ])
    }
}

impl Add for Vec3f {
    type Output = Vec3f;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3f([
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
        ])
    }
}


impl Add<&Vec3f> for f64 {
    type Output = Vec3f;
    #[inline(always)]
    fn add(self, rhs: &Vec3f) -> Self::Output {
        Vec3f([
            self + rhs[0],
            self + rhs[1],
            self + rhs[2],
        ])
    }
}
impl Add<f64> for Vec3f {
    type Output = Vec3f;
    #[inline(always)]
    fn add(self, rhs: f64) -> Self::Output {
        Vec3f([
            self[0] + rhs,
            self[1] + rhs,
            self[2] + rhs,
        ])
    }
}

impl Add<f64> for &Vec3f {
    type Output = Vec3f;
    #[inline(always)]
    fn add(self, rhs: f64) -> Self::Output {
        Vec3f([
            self[0] + rhs,
            self[1] + rhs,
            self[2] + rhs,
        ])
    }
}
impl Add<i64> for &Vec3f {
    type Output = Vec3f;

    #[inline(always)]
    fn add(self, rhs: i64) -> Self::Output {
        Vec3f([
            self[0] + rhs as f64,
            self[1] + rhs as f64,
            self[2] + rhs as f64,
        ])
    }
}

impl Sub<&Vec3f> for f64 {
    type Output = Vec3f;
    #[inline(always)]
    fn sub(self, rhs: &Vec3f) -> Self::Output {
        Vec3f([
            self - rhs.0[0],
            self - rhs.0[1],
            self - rhs.0[2],
        ])
    }
}
impl Sub<f64> for Vec3f {
    type Output = Vec3f;
    #[inline(always)]
    fn sub(self, rhs: f64) -> Self::Output {
        Vec3f([
            self.0[0] - rhs,
            self.0[1] - rhs,
            self.0[2] - rhs,
        ])
    }
}

impl Sub<f64> for &Vec3f {
    type Output = Vec3f;
    #[inline(always)]
    fn sub(self, rhs: f64) -> Self::Output {
        Vec3f([
            self.0[0] - rhs,
            self.0[1] - rhs,
            self.0[2] - rhs,
        ])
    }
}



impl Sub for &Vec3f {
    type Output = Vec3f;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3f([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl Sub for &mut Vec3f {
    type Output = Vec3f;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3f([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl Sub<&mut Vec3f> for &Vec3f {
    type Output = Vec3f;
    #[inline(always)]
    fn sub(self, rhs: &mut Vec3f) -> Self::Output {
        Vec3f([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl Sub<&Vec3f> for &mut Vec3f {
    type Output = Vec3f;
    #[inline(always)]
    fn sub(self, rhs: &Vec3f) -> Self::Output {
        Vec3f([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        return VectorArithmetic::subtract(&self, &rhs)
    }
}


impl Sum<Vec3f> for Vec3f {
    fn sum<I: Iterator<Item = Vec3f>>(iter: I) -> Self {
        iter.fold(Vec3f::new(0.0, 0.0, 0.0), |a, b| a + b)
    }
}


impl<'a> Sum<&'a Vec3f> for Vec3f {
    fn sum<I: Iterator<Item = &'a Vec3f>>(iter: I) -> Self {
        iter.fold(Vec3f::new(0.0, 0.0, 0.0), |a, b| a + *b)
    }
}

impl TryFrom<String> for Vec3f {
    type Error = SysError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let vertex_coordinate: Vec<_> = s.trim().split(' ').collect();

        if vertex_coordinate.len() != 3 {
            return Err(SysError::new_str(ErrorKind::Unparsable, "bad vertex coordinate"))
        }

        let v1 = f64::from_str(&vertex_coordinate[0]);
        let v2 = f64::from_str(&vertex_coordinate[1]);
        let v3 = f64::from_str(&vertex_coordinate[2]);

        if v1.is_ok() && v2.is_ok() && v3.is_ok() {
            return Ok(Self([v1.unwrap(), v2.unwrap(), v3.unwrap()]))
        }

        Err(SysError::new_str(ErrorKind::Unparsable, "cannot parse vertex coordinate from string to f64"))
    }

}


impl TryFrom<Vec<f64>> for Vec3f {
    type Error = SysError;
    fn try_from(v: Vec<f64>) -> Result<Self, Self::Error> {
        if v.len() != 3 {
            return Err(SysError::new_str(ErrorKind::Unparsable, "[TryFrom<Vec<f64>>] too many entries for Vec3f"))
        }

        Ok(Self([v[0], v[1], v[2]]))
    }
}


