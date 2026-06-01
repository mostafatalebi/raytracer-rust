use std::ops::{Index, IndexMut, Sub};
use serde::{Deserialize, Serialize};
use crate::vector::utils::Utils;
use crate::vector::arithmetic::VectorArithmetic;
use crate::vector::vec2f::Vec2f;
use crate::vector::vec3f::Vec3f;

pub trait Vector {
    fn size(&self) -> usize;
    fn subtract(&self, other: &Self) -> Self;
    fn add_with(&self, other: &Self) -> Self;
    fn multiply_scalar(&self, other: f64) -> Self;
    fn divide_by_scalar(&self, other: f64) -> Self;

    // truncates each component of the vector to a floating
    // number with fewer decimals, depending on the num provided
    fn trunc(&self, num: i64) -> Self;
    
    // aka unit-vector
    fn hat(&self) -> Self;

    fn magnitude(&self) -> f64;

    fn length_squared(&self) -> f64 ;

    fn normalized(&self) -> Self;

    fn dot(&self, other: &Self) -> f64;
}


pub trait ZeroableVector {
    fn zero() -> Self;
}




#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct Vec2i(pub [i64; 2]);

impl Vec2i {
    pub fn new(x: i64, y: i64) -> Vec2i {
        Vec2i([x, y])
    }

    pub fn get_as_float(&self) -> Vec2f {
        Vec2f::new(self[0] as f64, self[1] as f64)
    }
}

impl Vector for Vec2i {
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
        VectorArithmetic::multiply_scalar(self, other as i64)
    }

    fn divide_by_scalar(&self, other: f64) -> Self {
        // not supported
        self.clone()
    }

    fn hat(&self) -> Self {
        // not implementable
        Vec2i::new(0,0)
    }

    fn trunc(&self, num: i64) -> Self {
        self.clone()
    }

    fn magnitude(&self) -> f64 {
        f64::sqrt((self[0] * self[0] + self[1] * self[1]) as f64)
    }

    fn length_squared(&self) -> f64 {
        (self[0]*self[0] + self[1]*self[1]) as f64
    }

    fn normalized(&self) -> Self {
        Utils::normalize(self)
    }

    fn dot(&self, other: &Self) -> f64 {
        VectorArithmetic::dot(self, other)
    }
}

impl PartialEq for Vec2i {
    fn eq(&self, other: &Self) -> bool {
        if self.0 != other.0 || (self[1] != other[1]) || self[2] != other[2] {
            return false;
        }
        true
    }
}

impl Index<usize> for Vec2i {
    type Output = i64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl IndexMut<usize> for Vec2i {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}


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

impl Vector for Vec3i {
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
        VectorArithmetic::multiply_scalar(self, other as i64)
    }

    fn divide_by_scalar(&self, other: f64) -> Self {
        // not supported
        self.clone()
    }

    fn hat(&self) -> Self {
        // not implementable
        Vec3i::new(0,0, 0)
    }

    fn trunc(&self, num: i64) -> Self {
        self.clone()
    }

    fn magnitude(&self) -> f64 {
        f64::sqrt((self[0] * self[0] + self[1] * self[1]) as f64)
    }

    fn length_squared(&self) -> f64 {
        (self[0]*self[0] + self[1]*self[1] + self[2]*self[2]) as f64
    }

    fn normalized(&self) -> Self {
        Utils::normalize(self)
    }

    fn dot(&self, other: &Self) -> f64 {
        VectorArithmetic::dot(self, other)
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


pub type Slope = f64;
pub type Distance = f64;


// DirVec should have the same amount of data
// as Pos2d; they are the same, DirVec is used
// for better readability and type checking;
// and as the name suggest, it is used as direction
// vector
pub struct DirVec {
    pub origin: f64,
    pub direction: f64,
}

impl Sub for Vec2f {
    type Output = Self;
    fn sub(self, other: Self) -> Vec2f {
        Vec2f([self.0[0] - other.0[0], self.0[1] - other.0[1]])
    }
}



pub const SENSOR_SIZE_35: Vec2i = Vec2i([36, 24]);
pub const SENSOR_SQUARE_66: Vec2i = Vec2i([60, 60]);



