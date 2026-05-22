

pub trait ColumnMajor<R> {
    fn set_cm(&mut self, c_index: usize, r_index: usize, value: R);
    fn get_cm(&self, c_index: usize, r_index: usize) -> R;

    fn cm_add(&mut self, c_index: usize, r_index: usize, value: R);
    fn cm_sub(&mut self, c_index: usize, r_index: usize, value: R);
    fn cm_mul(&mut self, c_index: usize, r_index: usize, value: R);
    fn cm_div(&mut self, c_index: usize, r_index: usize, value: R);
}