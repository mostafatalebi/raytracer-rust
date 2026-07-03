use crate::scene::deserializers::deserialize_degree_to_radian;
use crate::scene::deserializers::deserialize_image;
use std::f32::consts::PI;
use crossterm::style::Color::Rgb;
use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};
use crate::colors::types::{Color, NColor3};
use crate::vector::vec2f::Vec2f;
use crate::vector::vec3f::Vec3f;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Environment {
    #[serde(skip)]
    img_obj: Option<DynamicImage>,
    image_path: String,
    #[serde(skip)]
    image_width: u32,
    #[serde(skip)]
    image_height: u32,
    repeat_u: usize,
    repeat_v: usize,
    flipped_vertically: bool,
    flipped_horizontally: bool,
    // rotation is in radian
    #[serde(deserialize_with = "deserialize_degree_to_radian")]
    rotate_y: f64,
}

impl Environment {
    pub fn new_with_image(image_path: String) -> Self {
        let mut image = image::open(image_path.clone()).unwrap();
        // image = image.blur_advanced(GaussianBlurParameters::new_from_radius(10.0));
        Self {
            img_obj: Some(image.clone()),
            image_path,
            flipped_vertically: false,
            flipped_horizontally: false,
            image_width: image.width(),
            image_height: image.height(),
            repeat_u: 0,
            repeat_v: 0,
            rotate_y: 0.0,
        }
    }

    pub fn flip_h(&mut self){
        self.flipped_horizontally = !self.flipped_horizontally;
    }

    pub fn set_repeat(&mut self, u: usize, v: usize) {
        self.repeat_u = u;
        self.repeat_v = v;
    }

    pub fn flip_v(&mut self){
        self.flipped_vertically = !self.flipped_vertically;
    }

    pub fn rotate_y(&mut self, degree: f64) {
        self.rotate_y = degree.to_radians();
    }

    pub fn get_dome_color(&self, ray_dir_n: &Vec3f) -> NColor3 {
        let uv = self.find_uv_on_sphere(ray_dir_n);
        let pixels = self.uv_to_pixel(&uv, self.image_width, self.image_height);
        let color = self.img_obj.as_ref().unwrap().get_pixel(pixels.0, pixels.1);

        Color::r_to_n_normal(color[0], color[1], color[2])
    }

    #[inline(always)]
    fn find_uv_on_sphere(&self, ray_dir_n: &Vec3f) -> Vec2f {
        let phi = ray_dir_n[2].atan2(ray_dir_n[0]);
        let theta = ray_dir_n[1].asin();

        let mut u = phi / (2.0 * std::f64::consts::PI);
        let mut v = theta / std::f64::consts::PI;
        if self.flipped_horizontally {
            u = 1.0 - u;
        }
        if self.flipped_vertically {
            v = 1.0 - v;
        }

        if self.repeat_u > 0 {
            u = (u * self.repeat_u as f64).rem_euclid(1.0);
        }
        if self.repeat_v > 0 {
            v = (v * self.repeat_v as f64).rem_euclid(1.0);
        }

        u = (u + self.rotate_y).rem_euclid(1.0);
        Vec2f::new(u, v)
    }

    #[inline(always)]
    fn uv_to_pixel(&self, uv: &Vec2f, width: u32, height: u32) -> (u32, u32) {
        let x = (uv[0] + 0.5) * (width as f64);
        let y = (uv[1] + 0.5) * (height as f64);

        (x as u32 % width, y as u32 % height)
    }
}