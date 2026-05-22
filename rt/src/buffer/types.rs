use std::ops::{Index, IndexMut};

// 0 -> buffer index
// 1 -> i, column index of the image
// 2 -> j, row index of the image
#[derive(Debug, Clone, Default)]
pub struct BufferIndex([usize; 3]);

impl BufferIndex {
    pub fn new(flat: usize, i: usize, j: usize) -> BufferIndex {
        BufferIndex([flat, i, j])
    }
}

impl Index<usize> for BufferIndex {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for BufferIndex {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl PartialEq for BufferIndex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}