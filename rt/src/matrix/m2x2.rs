use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Debug)]
pub struct Matrix2x2([[f64; 2]; 2]);


impl Matrix2x2 {
    pub fn new() -> Matrix2x2 {
        Matrix2x2([[0.0, 0.0], [0.0, 1.0]])
    }

    pub fn new_with(a: [f64; 2], b: [f64; 2]) -> Matrix2x2 {
        Matrix2x2([a, b])
    }

    pub fn new_identity() -> Matrix2x2 {
        Matrix2x2::new_with([1.0,0.0],
                            [0.0,1.0,])
    }

    pub fn det(&self) -> f64 {
        (self[0][0]*self[1][1]) - (self[0][1]*self[1][0])
    }

    pub fn multiply(&self, other: &Matrix2x2) -> Matrix2x2 {
        let mut m2 = Matrix2x2::new();

        m2[0][0] = self[0][0] * other[0][0] + self[0][1] * other[1][0];
        m2[0][1] = self[0][0] * other[0][1] + self[0][1] * other[1][1];

        m2[1][0] = self[1][0] * other[0][0] + self[1][1] * other[1][0];
        m2[1][1] = self[1][0] * other[0][1] + self[1][1] * other[1][1];

        m2
    }

    pub fn add_with(&self, other: &Matrix2x2) -> Matrix2x2 {
        let mut result = Matrix2x2::new();

        for i in 0..2 {
            for j in 0..2 {
                result.0[i][j] = self.0[i][j] + other.0[i][j];
            }
        }
        result
    }

    pub fn subtract_from(&self, other: &Matrix2x2) -> Matrix2x2 {
        let mut result = Matrix2x2::new();

        for i in 0..2 {
            for j in 0..2 {
                result.0[i][j] = self.0[i][j] - other.0[i][j];
            }
        }
        result
    }
}

impl PartialEq for Matrix2x2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Index<usize> for Matrix2x2 {
    type Output = [f64; 2];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Matrix2x2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Mul for Matrix2x2 {
    type Output = Matrix2x2;
    fn mul(self, other: Matrix2x2) -> Self::Output {
        self.multiply(&other)
    }
}

impl Mul for &Matrix2x2 {
    type Output = Matrix2x2;
    fn mul(self, other: &Matrix2x2) -> Self::Output {
        self.multiply(other)
    }
}


impl MulAssign for Matrix2x2 {
    fn mul_assign(&mut self, other: Matrix2x2) {
        *self = self.multiply(&other);
    }
}


impl Add for Matrix2x2 {
    type Output = Matrix2x2;
    fn add(self, other: Matrix2x2) -> Self::Output {
        self.add_with(&other)
    }
}

impl Add for &Matrix2x2 {
    type Output = Matrix2x2;
    fn add(self, other: &Matrix2x2) -> Self::Output {
        self.add_with(other)
    }
}

impl AddAssign for Matrix2x2 {
    fn add_assign(&mut self, other: Matrix2x2) {
        *self = self.add_with(&other);
    }
}

impl Sub for Matrix2x2 {
    type Output = Matrix2x2;
    fn sub(self, other: Matrix2x2) -> Self::Output {
        self.subtract_from(&other)
    }
}
impl Sub for &Matrix2x2 {
    type Output = Matrix2x2;
    fn sub(self, other: &Matrix2x2) -> Self::Output {
        self.subtract_from(other)
    }
}

impl SubAssign for Matrix2x2 {
    fn sub_assign(&mut self, other: Matrix2x2) {
        *self = self.subtract_from(&other);
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::m2x2::Matrix2x2;

    #[test]
    fn test_m2x2_arithmetic() {
        let mx1 = Matrix2x2::new_with([2.0, 3.0,],
                                                [1.0, 4.0]);

        let mx2 = Matrix2x2::new_with([4.0, 5.0],
                                                [1.0, 5.0]);

        let expected_mut = Matrix2x2::new_with([11.0, 25.0],
                                                         [8.0, 25.0]);
        assert_eq!(expected_mut, &mx1 * &mx2);

        let expected_add = Matrix2x2::new_with([6.0, 8.0],
                                                         [2.0, 9.0]);
        assert_eq!(expected_add, &mx1 + &mx2);

        let expected_sub = Matrix2x2::new_with([-2.0, -2.0],
                                                         [0.0, -1.0]);
        assert_eq!(expected_sub, &mx1 - &mx2);
    }

    #[test]
    fn test_m2x2_det() {
        let m2x2 = Matrix2x2::new_with([-2.0, -2.0],
                                                 [0.0, -1.0]);
        assert_eq!(2.0, m2x2.det());
    }
}