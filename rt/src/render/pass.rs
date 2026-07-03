use crate::colors::types::{Color, NColor3};
use crate::common::types::NormalizedF;
use crate::vector::constants::BLACK;

#[derive(Clone,Default,Debug)]
pub struct RenderPass {
    diffuse: NColor3,
    reflection: NColor3,
    shadow: NormalizedF,
}


impl RenderPass {
    
    pub fn new_from_color(diffuse: NColor3) -> Self {
        Self {
            diffuse,
            reflection: Color::r_to_n(&BLACK),
            shadow: 1.0,
        }
    }

    pub fn add_diffuse(&mut self, diffuse: &NColor3, shadow: NormalizedF) {
        self.diffuse += diffuse * (shadow as f64);
    }

    pub fn add_shadow(&mut self, shadow: NormalizedF) {
        self.shadow += shadow;
    }

    pub fn set_diffuse(&mut self, diffuse: NColor3) {
        self.diffuse = diffuse;
    }

    pub fn set_reflection(&mut self, reflection: NColor3) {
        self.reflection = reflection;
    }

    pub fn set_shadow(&mut self, shadow: NormalizedF) {
        self.shadow = shadow;
    }

    pub fn get_diffuse(&self) -> NColor3 {
        self.diffuse
    }

    pub fn get_reflection(&self) -> NColor3 {
        self.reflection
    }

    pub fn get_shadow(&self) -> NormalizedF {
        self.shadow
    }

    pub fn composite(&self) -> NColor3 {
        self.diffuse + self.reflection
    }
}

