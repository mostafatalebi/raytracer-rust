use std::any::Any;
use std::ops::{Index, IndexMut};
use dyn_clone::DynClone;
use serde::Serialize;
use crate::colors::procedural::CheckeredTexture;
use crate::colors::ramp::Ramp;
use crate::common::params::Params;
use crate::common::types::NormalizedF;
use crate::vector::arithmetic::VectorArithmetic;
use crate::vector::constants::{BLACK, WHITE};
use crate::vector::types::Vector;
use crate::vector::vec3f::Vec3f;
use crate::vector::vec4f::Vec4f;

#[derive(Default, Clone)]
pub struct InputChannel {
    color: Option<NColor3>,
    gradient: Option<Ramp>,
    texture: Box<dyn ProceduralTexture>,
}

impl InputChannel {
    pub fn new_with_color(color: NColor3) -> Self {
        let mut inp = Self::default();
        inp.set_color(Some(color));
        inp
    }

    pub fn new_with_texture(texture: Box<dyn ProceduralTexture>) -> Self {
        let mut inp = Self::default();
        inp.set_texture(texture);
        inp
    }
    pub fn get_true_color(&self, params: Option<&Params>) -> NColor3 {
        if let Some(c) = self.color {
            return c;
        }
        self.get_texture(params)
    }
    pub fn get_color(&self, params: Option<&Params>) -> Option<NColor3> {
        self.color
    }

    pub fn get_texture(&self, params: Option<&Params>) -> NColor3 {
        self.texture.get_texture(params)
    }

    pub fn get_multiplied(&self, params: Option<&Params>) -> NColor3 {
        let mut c1 = WHITE;
        if let Some(c) = self.color {
            c1 = c;
        }
        let c2 = self.get_texture(params);

        c1 * c2
    }

    pub fn set_color(&mut self, color: Option<NColor3>) {
        self.color = color;
    }
    pub fn set_texture(&mut self, texture: Box<dyn ProceduralTexture>) {
        self.texture = texture;
    }
}

pub type NColor3 = Vec3f;
pub type NColor4 = Vec4f;


pub struct Color {

}

impl Color {
    pub fn n_to_r<T>(color: &T) -> T
        where T: Vector<f64> + Index<usize, Output = f64> + IndexMut<usize, Output=f64> + Default + Copy {
        let v = color.multiply_scalar(255.0);
        v
    }

    pub fn avg<T>(color: &T, samples_count: u64) -> T
    where T: Vector<f64> + Index<usize, Output = f64> + IndexMut<usize, Output=f64> + Default + Copy {
        let scale: f64 = 1.0 / samples_count as f64;
        let mut v = color.multiply_scalar(scale);

        v.clamp(0.0, 0.9999);
        v
    }

    pub fn r_to_n_normal(r: u8, g: u8, b: u8) -> NColor3 {
        NColor3::new(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
    }

    pub fn r_to_n<T>(color: &T) -> T
    where T: Vector<f64> + Index<usize, Output = f64> + IndexMut<usize, Output=f64> + Default + Copy {
        let v = color.divide_by_scalar(255.0);
        v
    }

    /// works only and only when the underlying type is Vec4f;
    /// it then applies the 4th index to each other element
    pub fn apply_alpha<T>(color: &T) -> T
    where T: Vector<f64> + Index<usize, Output = T> + IndexMut<usize, Output=f64> + Default + Copy {
        let v = color.divide_by_scalar(255.0);
        v
    }

    pub fn n_clamp<T>(color: &T) -> T
    where T: Vector<f64> + Index<usize, Output = f64> + IndexMut<usize, Output=f64> + Default + Copy {
        VectorArithmetic::clamp(color, 0.0, 1.0)
    }
    pub fn r_clamp<T>(color: &T) -> T
    where T: Vector<f64> + Index<usize, Output = f64> + IndexMut<usize, Output=f64> + Default + Copy {
        VectorArithmetic::clamp(color, 0.0, 255.0)
    }
}


pub trait ProceduralTexture: DynClone+Send+Sync {
    fn get_texture(&self, params: Option<&Params>) -> NColor3;
    fn equals(&self, other: &dyn ProceduralTexture) -> bool;
}

impl Default for Box<dyn ProceduralTexture> {
    fn default() -> Self {
        Box::new(CheckeredTexture::default())
    }
}

dyn_clone::clone_trait_object!(ProceduralTexture);

impl PartialEq for Box<dyn ProceduralTexture> {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other.as_ref())
    }
}

