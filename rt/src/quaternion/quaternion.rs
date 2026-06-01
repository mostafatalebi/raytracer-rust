use std::ops::{Div, Mul, MulAssign};
use serde::{Deserialize, Serialize};
use crate::common::constants::EPS;
use crate::matrix::m3x3::Matrix3x3;
use crate::matrix::m4x4::Matrix4x4;
use crate::vector::types::Vector;
use crate::vector::vec3f::Vec3f;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Quaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


impl Quaternion {

    pub fn new(angle_rad: f64, axis: Vec3f) -> Self {
        let mut q = Self::default();
        let axis = axis.normalized();
        q.w = (angle_rad/2.0).cos();
        let s = (angle_rad/2.0).sin();

        q.x = s * axis[0];
        q.y = s * axis[1];
        q.z = s * axis[2];

        q
    }

    pub fn new_from_m3x3(m: &Matrix3x3) -> Self {

        let t = m.trace();

        if t > EPS {
            let s = (t + 1.0).sqrt() * 2.0;
            return Self {
                w: 0.25 * s,
                x: (m[2][1] - m[1][2]) / s,
                y: (m[0][2] - m[2][0]) / s,
                z: (m[1][0] - m[0][1]) / s,
            };
        } else if m[0][0] > m[1][1]
            && m[0][0] > m[2][2] {
            let s = (1.0 + m[0][0] - m[1][1] - m[2][2]).sqrt() * 2.0;
            return Self {
                w: (m[2][1] - m[1][2]) / s,
                x: 0.25 * s,
                y: (m[0][1] - m[1][0]) / s,
                z: (m[0][2] - m[2][0]) / s,
            };
        } else if m[1][1] > m[2][2] {
            let s = (1.0 + m[1][1] - m[0][0] - m[2][2]).sqrt() * 2.0;
            return Self {
                w: (m[0][2] - m[2][0]) / s,
                x: (m[0][1] - m[1][0]) / s,
                y: 0.25 * s,
                z: (m[1][2] - m[2][1]) / s,
            }
        } else {
            let s = (1.0 + m[2][2] - m[0][0] - m[1][1]).sqrt() * 2.0;
            return Self {
                w: (m[1][0] - m[0][1]) / s,
                x: (m[0][2] - m[2][0]) / s,
                y: (m[1][2] - m[2][1]) / s,
                z: 0.25 * s,
            }
        }
    }
    
    

    pub fn new_from_euler(euler: &Vec3f) -> Self {
        let qx = euler[0].to_radians() * 0.5;
        let qy = euler[1].to_radians() * 0.5;
        let qz = euler[2].to_radians() * 0.5;
        let qx_sin = qx.sin();
        let qx_cos = qx.cos();
        let qy_sin = qy.sin();
        let qy_cos = qy.cos();
        let qz_sin = qz.sin();
        let qz_cos = qz.cos();

        Quaternion{
            x: qx_sin * qy_cos * qz_cos + qx_cos * qy_sin * qz_sin,
            y: qx_cos * qy_sin *qz_cos - qx_sin * qy_cos * qz_sin,
            z: qx_cos * qy_cos * qz_sin + qx_sin * qy_sin * qz_cos,
            w: qx_cos * qy_cos * qz_cos - qx_sin * qy_sin * qz_sin,
        }
    }


    pub fn multiply(&self, b: &Quaternion) -> Quaternion {
        Quaternion{
            x: self.w * b.x + self.x * b.w + self.y * b.z - self.z * b.y,
            y: self.w * b.y - self.x * b.z + self.y * b.w + self.z * b.x,
            z: self.w * b.z + self.x * b.y - self.y * b.x + self.z * b.w,
            w: self.w * b.w - self.x * b.x - self.y * b.y - self.z * b.z,
        }
    }

    pub fn divide(&self, b: f64) -> Quaternion {
        Self{
            x: self.x / b,
            y: self.y / b,
            z: self.z / b,
            w: self.w / b,
        }
    }

    pub fn conjugate(&self) -> Quaternion {
        Quaternion{
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    pub fn inverse(&self) -> Quaternion {
        self.conjugate() / self.mag_sq()
    }

    pub fn mag_sq(&self) -> f64 {
          self.x * self.x
        + self.y * self.y
        + self.z * self.z
        + self.w * self.w
    }

    pub fn normalize(&self) -> Quaternion {
        let magnitude = self.mag_sq().sqrt();
        Self{
            w: self.w/magnitude,
            x: self.x/magnitude,
            y: self.y/magnitude,
            z: self.z/magnitude,
        }
    }

    pub fn to_euler(&self) -> Vec3f {
         Vec3f::new(self.x.atan2(self.w), self.y.asin(), self.z.atan2(self.w))
    }

    pub fn rotate_vec3f(&self, v: &Vec3f) ->  Vec3f {
        let qv = Vec3f::new(self.x, self.y, self.z);

        let t = qv.cross3(v) * 2.0;

        v + &(t * self.w) + qv.cross3(&t)
    }

    pub fn look_at(from: &Vec3f, to: &Vec3f, world_up: &Vec3f) -> Quaternion {
        let dir     = to - from;
        let dir_length = dir.magnitude();
        if dir_length < EPS {
            return Quaternion::default();
        }

        let forward = dir.normalized();
        let right   = forward.cross3(&world_up).normalized();
        let up      = right.cross3(&forward).normalized();

        let m3x3 = Matrix3x3::from_column(&right, &up, &forward);
        Quaternion::new_from_m3x3(&m3x3)
    }

    // converts the entire Quaternion to the M4x4 matrix; it doesn't
    // take into account the translation and scale. Be careful!
    pub fn to_m4x4(&self) -> Matrix4x4 {
        Matrix4x4::from_column([self.x, self.y, self.z, self.w], [-self.y, self.x, self.w, -self.z], [-self.z, -self.w, self.x, self.y], [-self.w, self.x, -self.y, self.z])
    }

    pub fn to_m3x3(&self) -> Matrix3x3 {
        let c00 = 1.0-2.0*(self.y*self.y + self.z*self.z);
        let c01 = 2.0*(self.x*self.y + self.w*self.z);
        let c02 = 2.0*(self.x*self.z - self.w*self.y);
        let c10 = 2.0*(self.x*self.y - self.w*self.z);
        let c11 = 1.0-2.0*(self.x*self.x + self.z*self.z);
        let c12 = 2.0*(self.y*self.z + self.w*self.x);
        let c20 = 2.0*(self.x*self.z + self.w*self.y);
        let c21 = 2.0*(self.y*self.z - self.w*self.x);
        let c22 = 1.0-2.0*(self.x*self.x + self.y*self.y);
        Matrix3x3::from_column(&Vec3f::new(c00, c01, c02), &Vec3f::new(c10, c11, c12), &Vec3f::new(c20, c21, c22))
    }

}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        self.multiply(&rhs)
    }
}

impl Mul<&Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: &Quaternion) -> Self::Output {
        self.multiply(rhs)
    }
}

impl Mul<&Vec3f> for &Quaternion {
    type Output = Vec3f;

    fn mul(self, rhs: &Vec3f) -> Self::Output {
        todo!()
    }
}

impl MulAssign<&Quaternion> for Quaternion {
    fn mul_assign(&mut self, rhs: &Quaternion) {
        *self = self.multiply(rhs)
    }
}


impl Div<f64> for &Quaternion {
    type Output = Quaternion;

    fn div(self, rhs: f64) -> Self::Output {
        self.divide(rhs)
    }
}

impl Div<f64> for Quaternion {
    type Output = Quaternion;

    fn div(self, rhs: f64) -> Self::Output {
        self.divide(rhs)
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self{
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

}