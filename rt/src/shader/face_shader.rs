use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::common::dummy_utils::pick_random_color;
use crate::error::error::SysError;
use crate::error::kinds::ErrorKind;
use crate::light::light::LightEnum;
use crate::ray::types::RayContext;
use crate::shader::shader::BaseShader;
use crate::vector::colors::{Color, NColor3};
use crate::vector::constants::WHITE;
#[derive(Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct FaceShader {
    shader_type: String,
    id: String,
    opacity: f32,

    #[serde(skip)]
    random_color: Vec<NColor3>,
    cached_color_per_face: HashMap<String, NColor3>
}

impl FaceShader {
    pub fn new(id: &str, opacity: f32) -> Self {
        let mut fs = FaceShader{
            shader_type: "flat".to_string(),
            id: id.to_string(),
            opacity,
            random_color: Vec::new(),
            cached_color_per_face: HashMap::new()
        };

        fs.fill_random_colors();
        fs
    }

    pub fn fill_random_colors(&mut self) {
        for i in 0..50 {
            self.random_color.push(pick_random_color());
        }
    }
}

#[typetag::serde]
impl BaseShader for FaceShader {

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn compute(&self, collision: &RayContext, lights: &LightEnum) -> Result<NColor3, SysError> {
        // let key = format!("object::{oid:?}::face::{fid:?}", oid=collision.collided_object_index, fid=collision.collided_face_index);
        // if collision.collided_object_index.is_none()  {
        //     return Err(SysError::new_str(ErrorKind::GeometryNotFound, "object id not found"))
        // } else if collision.collided_face_index.is_none()  {
        //     return Err(SysError::new_str(ErrorKind::GeometryNotFound, "face id not found"))
        // }
        //
        //
        // if self.cached_color_per_face.contains_key(&key) {
        //     return Ok(*self.cached_color_per_face.get(&key).unwrap())
        // }
        //
        //
        //
        // if self.random_color.is_empty() {
        //     self.fill_random_colors();
        // }
        //
        // let random_color = self.random_color.pop();
        //
        // if let Some(c) = random_color {
        //     self.cached_color_per_face.insert(key, c.to_normalized_color());
        //     return Ok(c.to_normalized_color());
        // }
        //
        //
        Ok(Color::r_to_n(&WHITE))
    }


}