use crate::vector::types::Vector;
use crate::vector::vec3f::Vec3f;

// given current zero-based index v, and a matrix's column size m and row size n
// this function returns the exact index of the x and y
// Assuming this 3x4 matrix,
// + + + +
// + + + +
// + + ? +
// it returns (2, 2) for question mark, when invoked:
// get_column_row_indices_from_xy(10, 3, 4)
pub fn get_column_row_indices_from_xy(v: usize, m: usize, n: usize) -> [usize; 2] {
    let i = v % m;
    let j = v / m;

    [i, j]
}

pub fn calc_object_center(vertices: &Vec<Vec3f>) -> Vec3f {
    let mut center = Vec3f::new(0.0, 0.0, 0.0);

    if vertices.len() == 0 {
        return center;
    }

    for v in vertices {
        center = center.add_with(v);
    }

    center[0] /= vertices.len() as f64;
    center[1] /= vertices.len() as f64;
    center[2] /= vertices.len() as f64;
    center
}

#[cfg(test)]
mod test {
    use crate::common::helpers::{calc_object_center, get_column_row_indices_from_xy};
    use crate::vector::vec3f::Vec3f;

    #[test]
    fn test_buffer_get_pixel_indices_from_xy() {
        assert_eq!([0, 0], get_column_row_indices_from_xy(0, 6, 6));
        assert_eq!([4, 0], get_column_row_indices_from_xy(4, 6, 6));
        assert_eq!([3, 2], get_column_row_indices_from_xy(15, 6, 6));
        assert_eq!([4, 5], get_column_row_indices_from_xy(34, 6, 6));
        assert_eq!([5, 5], get_column_row_indices_from_xy(35, 6, 6));
        assert_eq!([0, 3], get_column_row_indices_from_xy(18, 6, 6));
        assert_eq!([5, 5], get_column_row_indices_from_xy(35, 6, 6));
        assert_eq!([0, 640], get_column_row_indices_from_xy(921600, 36*40, 24*40))
    }


    #[test]
    fn test_calc_object_center() {
        let mut vertices: Vec<Vec3f> = Vec::new();
        assert_eq!(Vec3f::new(0.0, 0.0, 0.0), calc_object_center(&vertices));
        assert_eq!(Vec3f::new(0.0, 0.0, 0.0), calc_object_center(&vertices));
        vertices.push(Vec3f::new(1.0, 1.0, 1.0));
        vertices.push(Vec3f::new(1.0, 1.0, 1.0));
        vertices.push(Vec3f::new(1.0, 1.0, 1.0));
        vertices.push(Vec3f::new(1.0, 1.0, 1.0));

        assert_eq!(Vec3f::new(1.0, 1.0, 1.0), calc_object_center(&vertices));
        vertices.push(Vec3f::new(6.0, 6.0, 6.0));
        assert_eq!(Vec3f::new(2.0, 2.0, 2.0), calc_object_center(&vertices));
    }
}