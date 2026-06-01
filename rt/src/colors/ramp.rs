use crate::colors::types::NColor3;
use crate::common::types::NormalizedF;
use crate::vector::constants::BLACK;

#[derive(Copy, Clone)]
pub struct Ramp {
    start_color: NColor3,
    end_color: NColor3,
    start_pos: NormalizedF,

    // cannot be lower than or eq to start_post
    end_pos: NormalizedF
}


impl Ramp {
    pub fn get_color(&self, pos: NormalizedF) -> NColor3 {
        BLACK
    }
}