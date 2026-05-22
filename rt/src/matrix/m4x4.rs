use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use crate::matrix::column_major_trait::ColumnMajor;
use crate::matrix::m3x3::Matrix3x3;
use crate::vector::vec3f::Vec3f;
use crate::vector::vec4f::Vec4f;

// a 3 by 3 matrix in row major format
// use build function to access it with
// column major format
#[derive(Clone, Debug)]
pub struct Matrix4x4([[f64; 4]; 4]);


impl Matrix4x4 {
    pub fn new() -> Matrix4x4 {
        Matrix4x4([[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]])
    }

    pub fn from_column(a: [f64; 4], b: [f64; 4], c: [f64; 4], d: [f64; 4]) -> Self {
        Self([[a[0], b[0], c[0], d[0]], [a[1], b[1], c[1], d[1]], [a[2], b[2], c[2], d[2]], [a[3], b[3], c[3], d[3]]])
    }

    pub fn new_with(a: [f64; 4], b: [f64; 4], c: [f64; 4], d: [f64; 4]) -> Matrix4x4 {
        Matrix4x4([a, b, c, d])
    }

    pub fn new_identity() -> Matrix4x4 {
        Matrix4x4::new_with([1.0,0.0,0.0,0.0],
                            [0.0,1.0,0.0,0.0],
                            [0.0,0.0,1.0,0.0],
                            [0.0,0.0,0.0,1.0])
    }

    pub fn det(&self) -> f64 {
        self[0][0]*(
            self[1][1]*(
                self[2][2]*self[3][3] -
                    self[2][3]*self[3][2]
            )
                - self[1][2]*(
                self[2][1]*self[3][3] -
                    self[2][3]*self[3][1]
            )
                + self[1][3]*(
                self[2][1]*self[3][2] -
                    self[2][2]*self[3][1]
            )
        )
            - self[0][1]*(
            self[1][0]*(
                self[2][2]*self[3][3] -
                    self[2][3]*self[3][2]
            )
                - self[1][2]*(
                self[2][0]*self[3][3] -
                    self[2][3]*self[3][0]
            )
                + self[1][3]*(
                self[2][0]*self[3][2] -
                    self[2][2]*self[3][0]
            )
        )
            + self[0][2]*(
            self[1][0]*(
                self[2][1]*self[3][3] -
                    self[2][3]*self[3][1]
            )
                - self[1][1]*(
                self[2][0]*self[3][3] -
                    self[2][3]*self[3][0]
            )
                + self[1][3]*(
                self[2][0]*self[3][1] -
                    self[2][1]*self[3][0]
            )
        )
            - self[0][3]*(
            self[1][0]*(
                self[2][1]*self[3][2] -
                    self[2][2]*self[3][1]
            )
                - self[1][1]*(
                self[2][0]*self[3][2] -
                    self[2][2]*self[3][0]
            )
                + self[1][2]*(
                self[2][0]*self[3][1] -
                    self[2][1]*self[3][0]
            )
        )
    }
    pub fn multiply(&self, other: &Matrix4x4) -> Matrix4x4 {
        let mut m4 = Matrix4x4::new();

        m4.0[0][0] = self[0][0]*other[0][0] + self[0][1]*other[1][0] + self[0][2]*other[2][0] + self[0][3]*other[3][0];
        m4.0[0][1] = self[0][0]*other[0][1] + self[0][1]*other[1][1] + self[0][2]*other[2][1] + self[0][3]*other[3][1];
        m4.0[0][2] = self[0][0]*other[0][2] + self[0][1]*other[1][2] + self[0][2]*other[2][2] + self[0][3]*other[3][2];
        m4.0[0][3] = self[0][0]*other[0][3] + self[0][1]*other[1][3] + self[0][2]*other[2][3] + self[0][3]*other[3][3];

        m4.0[1][0] = self[1][0]*other[0][0] + self[1][1]*other[1][0] + self[1][2]*other[2][0] + self[1][3]*other[3][0];
        m4.0[1][1] = self[1][0]*other[0][1] + self[1][1]*other[1][1] + self[1][2]*other[2][1] + self[1][3]*other[3][1];
        m4.0[1][2] = self[1][0]*other[0][2] + self[1][1]*other[1][2] + self[1][2]*other[2][2] + self[1][3]*other[3][2];
        m4.0[1][3] = self[1][0]*other[0][3] + self[1][1]*other[1][3] + self[1][2]*other[2][3] + self[1][3]*other[3][3];

        m4.0[2][0] = self[2][0]*other[0][0] + self[2][1]*other[1][0] + self[2][2]*other[2][0] + self[2][3]*other[3][0];
        m4.0[2][1] = self[2][0]*other[0][1] + self[2][1]*other[1][1] + self[2][2]*other[2][1] + self[2][3]*other[3][1];
        m4.0[2][2] = self[2][0]*other[0][2] + self[2][1]*other[1][2] + self[2][2]*other[2][2] + self[2][3]*other[3][2];
        m4.0[2][3] = self[2][0]*other[0][3] + self[2][1]*other[1][3] + self[2][2]*other[2][3] + self[2][3]*other[3][3];

        m4.0[3][0] = self[3][0]*other[0][0] + self[3][1]*other[1][0] + self[3][2]*other[2][0] + self[3][3]*other[3][0];
        m4.0[3][1] = self[3][0]*other[0][1] + self[3][1]*other[1][1] + self[3][2]*other[2][1] + self[3][3]*other[3][1];
        m4.0[3][2] = self[3][0]*other[0][2] + self[3][1]*other[1][2] + self[3][2]*other[2][2] + self[3][3]*other[3][2];
        m4.0[3][3] = self[3][0]*other[0][3] + self[3][1]*other[1][3] + self[3][2]*other[2][3] + self[3][3]*other[3][3];

        m4
    }

    pub fn multiply_vec4f(&self, v: &Vec4f) -> Vec4f {
        let mut result = Vec4f::new(0.0, 0.0, 0.0, 0.0);
        for i in 0..3 {
            result[i] = v[i]*self[i][0] + v[i]*self[i][1] + v[i]*self[i][2];
        }
        result
    }

    

    pub fn add_with(&self, other: &Matrix4x4) -> Matrix4x4 {
        let mut result = Matrix4x4::new();

        for i in 0..3 {
            for j in 0..3 {
                result.0[i][j] = self.0[i][j] + other.0[i][j];
            }
        }
        result
    }

    pub fn subtract_from(&self, other: &Matrix4x4) -> Matrix4x4 {
        let mut result = Matrix4x4::new();

        for i in 0..3 {
            for j in 0..3 {
                result.0[i][j] = self.0[i][j] - other.0[i][j];
            }
        }
        result
    }

    pub fn from_m3x3(m: &Matrix3x3, right_most_column: &Vec3f, bottom_most_row: &Vec3f, bottom_most_right_most: f64) -> Matrix4x4 {
        let mut m4 = Self::new();

        for i in 0..3 {
            for j in 0..3 {
                m4[i][j] = m[i][j]
            }
        }

        // right most column, which means editing every row
        // at its last index (since we are accessing barebone row-major)
        m4[0][3] = right_most_column[0];
        m4[1][3] = right_most_column[1];
        m4[2][3] = right_most_column[2];

        m4[3][0] = bottom_most_row[0];
        m4[3][1] = bottom_most_row[1];
        m4[3][2] = bottom_most_row[2];

        m4[3][3] = bottom_most_right_most;
        
        m4
    }
}

impl ColumnMajor<f64> for Matrix4x4 {
    fn set_cm(&mut self, c_index: usize, r_index: usize, value: f64) {
        self[r_index][c_index] = value;
    }

    fn get_cm(&self, c_index: usize, r_index: usize) -> f64 {
        self[r_index][c_index]
    }

    fn cm_add(&mut self, c_index: usize, r_index: usize, value: f64) {
        self[r_index][c_index] += value;
    }

    fn cm_sub(&mut self, c_index: usize, r_index: usize, value: f64) {
        self[r_index][c_index] -= value;
    }

    fn cm_mul(&mut self, c_index: usize, r_index: usize, value: f64) {
        self[r_index][c_index] *= value;
    }
    fn cm_div(&mut self, c_index: usize, r_index: usize, value: f64) {
        self[r_index][c_index] /= value;
    }

}

impl PartialEq for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Index<usize> for Matrix4x4 {
    type Output = [f64; 4];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Matrix4x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, other: Self) -> Self::Output {
        self.multiply(&other)
    }

}

impl Mul<&Vec4f> for Matrix4x4 {
    type Output = Vec4f;

    fn mul(self, other: &Vec4f) -> Self::Output {
        self.multiply_vec4f(&other)
    }
}

impl Mul<&Vec4f> for &Matrix4x4 {
    type Output = Vec4f;

    fn mul(self, other: &Vec4f) -> Self::Output {
        self.multiply_vec4f(&other)
    }
}


impl Mul for &Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, other: &Matrix4x4) -> Self::Output {
        self.multiply(other)
    }
}


impl MulAssign for Matrix4x4 {
    fn mul_assign(&mut self, other: Matrix4x4) {
        *self = self.multiply(&other);
    }
}


impl Add for Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, other: Matrix4x4) -> Self::Output {
        self.add_with(&other)
    }
}

impl Add for &Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, other: &Matrix4x4) -> Self::Output {
        self.add_with(other)
    }
}

impl AddAssign for Matrix4x4 {
    fn add_assign(&mut self, other: Matrix4x4) {
        *self = self.add_with(&other);
    }
}

impl Sub for Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, other: Matrix4x4) -> Self::Output {
        self.subtract_from(&other)
    }
}
impl Sub for &Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, other: &Matrix4x4) -> Self::Output {
        self.subtract_from(other)
    }
}

impl SubAssign for Matrix4x4 {
    fn sub_assign(&mut self, other: Matrix4x4) {
        *self = self.subtract_from(&other);
    }
}

#[cfg(test)]
mod tests {
}