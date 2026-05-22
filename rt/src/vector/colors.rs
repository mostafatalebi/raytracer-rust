use crate::vector::vec3f::Vec3f;
use crate::vector::vec4f::Vec4f;

pub type Rgba = Vec4f;
pub type Rgb = Vec3f;

// it uses the range 0-1.0 to represent colors
// that are commonly represented as 0-250 range
pub type NormalizedColor = Vec4f;


impl NormalizedColor {
    pub fn to_rgba(&self) -> Rgba {
        Rgba::new(self[0] * 250.0, self[1] * 250.0, self[2] * 250.0, self[3])
    }

    pub fn clamp(&mut self) {
        self[0] = self[0].clamp(0.0, 1.0);
        self[1] = self[1].clamp(0.0, 1.0);
        self[2] = self[2].clamp(0.0, 1.0);
        self[3] = self[3].clamp(0.0, 1.0)
    }

    pub fn clamped(&mut self) {
        Self::new(self[0].clamp(0.0, 1.0),
                          self[1].clamp(0.0, 1.0),
                          self[2].clamp(0.0, 1.0),
                          self[3].clamp(0.0, 1.0));
    }
}

impl Rgba {
    pub fn to_normalized_color(&self) -> NormalizedColor {
        NormalizedColor::new(self[0]/250.0, self[1]/250.0, self[2]/250.0, self[3])
    }
    pub fn to_rgb(&self) -> Rgb {
        Rgb::new(self[0], self[1], self[2])
    }
}

impl Rgb {
    pub fn to_normalized_color(&self) -> NormalizedColor {
        NormalizedColor::new(self[0]/250.0, self[1]/250.0, self[2]/250.0, 1.0)
    }
    pub fn to_rgba(&self) -> Rgba {
        Rgba::new(self[0], self[1], self[2], 1.0)
    }
}


