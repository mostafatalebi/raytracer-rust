


pub struct RenderRegion {
    u: usize,
    v: usize,
    width: usize,
    height: usize,
}

impl RenderRegion {
    pub fn new(u: usize, v: usize, w: usize, h: usize) -> Self {
        Self { u, v, width: w, height: h }
    }

    pub fn is_in_region(&self, u: usize, v: usize) -> bool {
        let u_end = self.u + self.width;
        let v_end = self.v + self.height;
        if ( u > self.u && u < u_end ) && (v > self.v && v < v_end)  {
            return true;
        }

        false
    }
}