use serde::{Deserialize, Serialize};
use crate::colors::types::NColor3;
use crate::error::error::SysError;
use crate::light::light::LightEnum;
use crate::ray::types::RayContext;
use crate::shader::face_shader::FaceShader;
use crate::shader::lambert::LambertShader;
use crate::shader::phong::PhongShader;
#[typetag::serde]
pub trait BaseShader {
    fn get_id(&self) -> String;
    fn compute(&self, collision: &RayContext, lights: &LightEnum) -> Result<NColor3, SysError>;

    fn cast_reflection(&self) -> bool;
    fn set_reflection_properties(&self, rc: &mut RayContext);
    fn get_reflection_final_color(&self, ref_color: &NColor3) -> NColor3;
}

#[derive(Deserialize, Serialize, Clone)]
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

    fn compute(&self, collision: &RayContext, lights: &LightEnum) -> Result<NColor3, SysError> {
        match self {
            ShaderEnum::Lambert(shader) => shader.compute(collision, lights),
            ShaderEnum::Phong(shader) => shader.compute(collision, lights),
            ShaderEnum::FaceShader(shader) => shader.compute(collision, lights),
        }
    }

    fn cast_reflection(&self) -> bool {
        match self {
            ShaderEnum::Lambert(shader) => shader.cast_reflection(),
            ShaderEnum::Phong(shader) => shader.cast_reflection(),
            ShaderEnum::FaceShader(shader) => shader.cast_reflection(),
        }
    }

    fn set_reflection_properties(&self, rc: &mut RayContext) {
        match self {
            ShaderEnum::Lambert(shader) => shader.set_reflection_properties(rc),
            ShaderEnum::Phong(shader) => shader.set_reflection_properties(rc),
            ShaderEnum::FaceShader(shader) => shader.set_reflection_properties(rc),
        }
    }

    fn get_reflection_final_color(&self, ref_color: &NColor3) -> NColor3 {
        match self {
            ShaderEnum::Lambert(shader) => shader.get_reflection_final_color(ref_color),
            ShaderEnum::Phong(shader) => shader.get_reflection_final_color(ref_color),
            ShaderEnum::FaceShader(shader) => shader.get_reflection_final_color(ref_color),
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