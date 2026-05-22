use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use crate::matrix::column_major_trait::ColumnMajor;
use crate::vector::vec3f::Vec3f;


// a 3 by 3 matrix in row major format
// use build function to access it with
// column major format
#[derive(Clone, Debug)]
pub struct Matrix3x3([[f64; 3]; 3]);


impl Matrix3x3 {
    pub fn new() -> Matrix3x3 {
        Matrix3x3([[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]])
    }

    pub fn new_with(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Matrix3x3 {
        Matrix3x3([a, b, c])
    }

    pub fn from_column(a: &Vec3f, b: &Vec3f, c: &Vec3f) -> Matrix3x3 {
        Matrix3x3([[a[0], b[0], c[0]], [a[1], b[1], c[1]], [a[2], b[2], c[2]]])
    }

    pub fn new_identity() -> Matrix3x3 {
        Matrix3x3::new_with([1.0,0.0,0.0,],
                            [0.0,1.0,0.0,],
                            [0.0,0.0,1.0,])
    }

    pub fn det(&self) -> f64 {
        (self[0][0]*self[1][1]*self[2][2])
            + (self[0][1]*self[1][2]*self[2][0])
                + (self[0][2]*self[1][0]*self[2][1])
        -
            (self[0][2]*self[1][1]*self[2][0])
            -   (self[0][0]*self[1][2]*self[2][1])
            -   (self[0][1]*self[1][0]*self[2][2])
    }

    pub fn multiply(&self, other: &Matrix3x3) -> Matrix3x3 {
        let mut m3 = Matrix3x3::new();

        m3.0[0][0] = self.0[0][0] * other.0[0][0] + self.0[0][1] * other.0[1][0] + self.0[0][2] * other.0[2][0];
        m3.0[0][1] = self.0[0][0] * other.0[0][1] + self.0[0][1] * other.0[1][1] + self.0[0][2] * other.0[2][1];
        m3.0[0][2] = self.0[0][0] * other.0[0][2] + self.0[0][1] * other.0[1][2] + self.0[0][2] * other.0[2][2];

        m3.0[1][0] = self.0[1][0] * other.0[0][0] + self.0[1][1] * other.0[1][0] + self.0[1][2] * other.0[2][0];
        m3.0[1][1] = self.0[1][0] * other.0[0][1] + self.0[1][1] * other.0[1][1] + self.0[1][2] * other.0[2][1];
        m3.0[1][2] = self.0[1][0] * other.0[0][2] + self.0[1][1] * other.0[1][2] + self.0[1][2] * other.0[2][2];

        m3.0[2][0] = self.0[2][0] * other.0[0][0] + self.0[2][1] * other.0[1][0] + self.0[2][2] * other.0[2][0];
        m3.0[2][1] = self.0[2][0] * other.0[0][1] + self.0[2][1] * other.0[1][1] + self.0[2][2] * other.0[2][1];
        m3.0[2][2] = self.0[2][0] * other.0[0][2] + self.0[2][1] * other.0[1][2] + self.0[2][2] * other.0[2][2];

        m3
    }

    pub fn multiply_vec3f(&self, v: &Vec3f) -> Vec3f {
        let mut result = Vec3f::new(0.0, 0.0, 0.0);
        for i in 0..3 {
            result[i] = v[i] * self[i][0] + v[i] * self[i][1] + v[i] * self[i][2];
        }
        result
    }

    

    pub fn add_with(&self, other: &Matrix3x3) -> Matrix3x3 {
        let mut result = Matrix3x3::new();

        for i in 0..3 {
            for j in 0..3 {
                result.0[i][j] = self.0[i][j] + other.0[i][j];
            }
        }
        result
    }

    pub fn subtract_from(&self, other: &Matrix3x3) -> Matrix3x3 {
        let mut result = Matrix3x3::new();

        for i in 0..3 {
            for j in 0..3 {
                result.0[i][j] = self.0[i][j] - other.0[i][j];
            }
        }
        result
    }

    pub fn trace(&self) -> f64 {
        self[0][0] + self[1][1] + self[2][2]
    }
}

impl ColumnMajor<f64> for Matrix3x3 {
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

impl PartialEq for Matrix3x3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Index<usize> for Matrix3x3 {
    type Output = [f64; 3];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Matrix3x3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Mul for Matrix3x3 {
    type Output = Matrix3x3;
    fn mul(self, other: Self) -> Self::Output {
        self.multiply(&other)
    }

}

impl Mul<&Vec3f> for Matrix3x3 {
    type Output = Vec3f;

    fn mul(self, other: &Vec3f) -> Self::Output {
        self.multiply_vec3f(&other)
    }
}

impl Mul<&Vec3f> for &Matrix3x3 {
    type Output = Vec3f;

    fn mul(self, other: &Vec3f) -> Self::Output {
        self.multiply_vec3f(&other)
    }
}


impl Mul for &Matrix3x3 {
    type Output = Matrix3x3;
    fn mul(self, other: &Matrix3x3) -> Self::Output {
        self.multiply(other)
    }
}


impl MulAssign for Matrix3x3 {
    fn mul_assign(&mut self, other: Matrix3x3) {
        *self = self.multiply(&other);
    }
}


impl Add for Matrix3x3 {
    type Output = Matrix3x3;
    fn add(self, other: Matrix3x3) -> Self::Output {
        self.add_with(&other)
    }
}

impl Add for &Matrix3x3 {
    type Output = Matrix3x3;
    fn add(self, other: &Matrix3x3) -> Self::Output {
        self.add_with(other)
    }
}

impl AddAssign for Matrix3x3 {
    fn add_assign(&mut self, other: Matrix3x3) {
        *self = self.add_with(&other);
    }
}

impl Sub for Matrix3x3 {
    type Output = Matrix3x3;
    fn sub(self, other: Matrix3x3) -> Self::Output {
        self.subtract_from(&other)
    }
}
impl Sub for &Matrix3x3 {
    type Output = Matrix3x3;
    fn sub(self, other: &Matrix3x3) -> Self::Output {
        self.subtract_from(other)
    }
}

impl SubAssign for Matrix3x3 {
    fn sub_assign(&mut self, other: Matrix3x3) {
        *self = self.subtract_from(&other);
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::m3x3::Matrix3x3;

    #[test]
    fn test_m3x3_arithmetic() {
        let mx1 = Matrix3x3::new_with([2.0, 3.0, -2.0],
                                                [1.0, 4.0, 4.0],
                                                [-1.0, 0.0, 2.0]);

        let mx2 = Matrix3x3::new_with([4.0, 5.0, -1.0],
                                                [1.0, 5.0, 3.0],
                                                [9.0, 2.0, 4.0]);

        let expected_mut = Matrix3x3::new_with([-7.0, 21.0, -1.0],
                                                         [44.0, 33.0, 27.0],
                                                         [14.0, -1.0, 9.0]);
        assert_eq!(expected_mut, &mx1 * &mx2);

        let expected_add = Matrix3x3::new_with([6.0, 8.0, -3.0],
                                                         [2.0, 9.0, 7.0],
                                                         [8.0, 2.0, 6.0]);
        assert_eq!(expected_add, &mx1 + &mx2);

        let expected_sub = Matrix3x3::new_with([-2.0, -2.0, -1.0],
                                                         [0.0, -1.0, 1.0],
                                                         [-10.0, -2.0, -2.0]);
        assert_eq!(expected_sub, &mx1 - &mx2);
    }

    #[test]
    fn test_m3x3_det() {
        let m3x3 = Matrix3x3::new_with([-2.0, -2.0, -1.0],
                                               [0.0, -1.0, 1.0],
                                               [-10.0, -2.0, -2.0]);
        assert_eq!(22.0, m3x3.det());
    }
}