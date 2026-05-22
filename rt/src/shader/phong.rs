use serde::{Deserialize, Serialize};
use crate::error::error::SysError;
use crate::light::light::LightEnum;
use crate::ray::types::RayCollision;
use crate::vector::vec3f::Vec3f;
use crate::shader::shader::BaseShader;
use crate::vector::colors::{NormalizedColor, Rgba};

#[derive(Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct PhongShader {
    shader_type: String,
    id: String,
    color: Vec3f,
    opacity: f32,
}

#[typetag::serde]
impl BaseShader for PhongShader {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn compute(&self, collision: &RayCollision, lights: &LightEnum) -> Result<NormalizedColor, SysError> {
        Ok(Rgba::default())
    }
}