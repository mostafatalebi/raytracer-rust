use serde::{Deserialize, Serialize};
use crate::error::error::SysError;
use crate::light::light::LightEnum;
use crate::ray::types::RayCollision;
use crate::shader::face_shader::FaceShader;
use crate::shader::lambert::LambertShader;
use crate::shader::phong::PhongShader;
use crate::vector::colors::{NormalizedColor, Rgba};

#[typetag::serde]
pub trait BaseShader {
    fn get_id(&self) -> String;
    fn compute(&self, collision: &RayCollision, lights: &LightEnum) -> Result<NormalizedColor, SysError>;
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "shader_type")]
pub enum ShaderEnum {
    #[serde(rename="lambert")]
    Lambert(LambertShader),
    #[serde(rename="phong")]
    Phong(PhongShader),
    #[serde(rename="face_shader")]
    FaceShader(FaceShader),
}

#[typetag::serde]
impl BaseShader for ShaderEnum {
    fn get_id(&self) -> String {
        match self {
            ShaderEnum::Lambert(shader) => shader.get_id(),
            ShaderEnum::Phong(shader) => shader.get_id(),
            ShaderEnum::FaceShader(shader) => shader.get_id(),
        }
    }

    fn compute(&self, collision: &RayCollision, lights: &LightEnum) -> Result<Rgba, SysError> {
        match self {
            ShaderEnum::Lambert(shader) => shader.compute(collision, lights),
            ShaderEnum::Phong(shader) => shader.compute(collision, lights),
            ShaderEnum::FaceShader(shader) => shader.compute(collision, lights),
        }
    }
}

impl Default for ShaderEnum {
    fn default() -> Self {
        ShaderEnum::Lambert(LambertShader::default())
    }
}

impl From<&str> for ShaderEnum {
    fn from(shader_type: &str) -> Self {
        match shader_type {
            "lambert" => ShaderEnum::Lambert(LambertShader::default()),
            "phong" => ShaderEnum::Phong(PhongShader::default()),
            "face_shader" => ShaderEnum::FaceShader(FaceShader::default()),
            _ => ShaderEnum::Lambert(LambertShader::default()),
        }
    }
}

impl From<ShaderEnum> for String {
    fn from(shader_type: ShaderEnum) -> Self {
        match shader_type {
            ShaderEnum::Lambert(s) => String::from("lambert"),
            ShaderEnum::Phong(s) => String::from("phong"),
            ShaderEnum::FaceShader(s) => String::from("flat"),
        }
    }
}


#[derive(Default, Deserialize, Serialize)]
pub struct Reflection {
    enable: bool,

    // how deep reflection should recursively go
    // default: 2
    limit: i8,

    // 0 to 1
    // 0 -> sharp reflections,
    // 1 -> fully blurred
    glossiness: f32,

    // number of samples when glossiness > 0
    samples: i8,
}