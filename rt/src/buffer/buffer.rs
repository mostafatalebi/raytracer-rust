use std::collections::HashMap;
use image::{Rgb, RgbImage};
use crate::buffer::types::BufferIndex;
use crate::common::helpers::get_column_row_indices_from_xy;
use crate::error::error::SysError;
use crate::error::kinds::ErrorKind;
use crate::colors::types::Color;
use crate::vector::constants::BLACK;
use crate::vector::vec4f::Vec4f;

#[derive(Debug, Clone)]
pub struct Buffer {
    pub x: usize,
    pub y: usize,
    _ptr_rgb: usize,
    _buffer_iter_index: usize,
    _buffer_size: usize,
    rgba: Vec<Vec4f>,

    // a map to store per pixel processing
    // duration, so to be able to create
    // a heatmap from it
    heatmap: HashMap<usize, i64>,

    // @todo add lock and concurrency to handle concurrent buffer access
}


impl Buffer {
    pub fn new(width: usize, height: usize) -> Buffer {
        let res = width*height;
        Buffer{
            x: width,
            y: height,
            _ptr_rgb: 0,
            _buffer_iter_index: 0,
            _buffer_size: res,
            rgba: vec!(Color::r_to_n(&BLACK.to_4()); res),
            heatmap: HashMap::new(),
        }
    }

    pub fn get_size(&self) -> usize {
        self._buffer_size
    }


    // allows saving RGB at the same as well as internally increment
    // index ptr. You cannot save it at a specific index, this is used
    // for sequential insertion
    pub fn save_pixel_color(&mut self, index: usize, color: Vec4f) {
        self.rgba[index] = color;
    }

    pub fn add_heatmap_entry(&mut self, index: usize, value: i64) {
        self.heatmap.insert(index, value);
    }


    // returns a tuple of (index, [x, y])
    // index is the index of the pixel in the buffer
    // [x, y] is the pixel's coordinates in the image
    //
    pub fn get_next_pixel_indices(&mut self) -> Option<BufferIndex> {
        let next_i = self.next();

        if next_i.is_none() {
            return None;
        }
        let v = next_i.unwrap();
        let indices = get_column_row_indices_from_xy(v, self.x, self.y);

        Some(BufferIndex::new(v, indices[0], indices[1]))
    }

    // @todo temporary function
    // it must be replaced by a proper image lib
    pub fn save_as_jpeg(&self, filename: &str) -> Result<(), SysError>{
        let mut img = RgbImage::new(self.x as u32, self.y as u32);

        let mut i = 0;
        for rgb in &self.rgba {
            let index = get_column_row_indices_from_xy(i, self.x, self.y);
            let p = Rgb::from([rgb[0] as u8, rgb[1] as u8, rgb[2] as u8]);
        img.put_pixel(index[0] as u32, index[1] as u32, p);
            i += 1;
        }

        match img.save(filename) {
            Ok(res) => {

            },
            Err(err) => {
                return Err(SysError::new(ErrorKind::FailedToSaveImage, format!("failed to save image output: {}", err)))
            }
        }
        Ok(())
    }


    
}

impl Iterator for Buffer {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self._buffer_iter_index < self._buffer_size {
            let prev = self._buffer_iter_index;
            self._buffer_iter_index += 1usize;

            return Some(prev)
        }
        None
    }
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_buffer_next_pixel_indices() {
        let mut buffer = Buffer::new(6, 6);

        let v = buffer.get_next_pixel_indices();

        assert_eq!(v, Some(BufferIndex::new(0, 0,0)));

        let v = buffer.get_next_pixel_indices();

        assert_eq!(v, Some(BufferIndex::new(1, 1,0)));
    }


}