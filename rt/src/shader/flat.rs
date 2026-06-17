use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::colors::types::{Color, NColor3};
use crate::common::types::NormalizedF;
use crate::error::error::SysError;
use crate::light::light::{BaseLight, LightEnum};
use crate::geometry::geometry::Geometry;
use crate::ray::ray_context::RayContext;
use crate::scene::scene::Scene;
use crate::vector::constants::{BLACK, GRAY, WHITE};
use crate::vector::types::Vector;
use crate::shader::shader::{BaseShader, ShaderEnum};
#[derive(Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct FlatShader {
    id: String,
    diffuse: NColor3,
}

#[typetag::serde]
impl BaseShader for FlatShader {
    fn get_id(&self) -> String {
        self.id.clone()
    }


    fn compute(&self, rc: &RayContext, light: &LightEnum) -> Result<NColor3, SysError> {
        return Ok(self.diffuse);
    }

    fn cast_reflection(&self) -> bool {
        false
    }

    fn set_reflection_properties(&self, rc: &mut RayContext) {

    }

    fn get_reflection_final_color(&self, ref_color: &NColor3) -> NColor3 {
        ref_color.clone()
    }
}

impl FlatShader {

    pub fn get_shader(&self) -> ShaderEnum {
        ShaderEnum::Flat(self.clone())
    }

    pub fn new() -> Self {
        FlatShader::default()
    }


    pub fn default() -> FlatShader {
        FlatShader{
            id: "flat_001".to_string(),
            diffuse: Color::r_to_n(&WHITE),
        }
    }


    pub fn set_diffuse(&mut self, diffuse_color: NColor3) -> &mut Self {
        self.diffuse = diffuse_color;
        self
    }
    pub fn get_diffuse(&self) -> NColor3 {
        self.diffuse
    }

    pub fn set_id(&mut self, id: &str) -> &mut Self {
        self.id = String::from(id);
        self
    }
    pub fn auto_id(&mut self) -> &mut Self {
        self.id = format!("flat::{}", Uuid::new_v4().to_string());
        self
    }

    pub fn assign_to(&mut self, geo: &mut Geometry) -> &mut Self {
        geo.assign_shader(&self.get_id());
        self
    }

    pub fn add_to_scene(&self, sc: &mut Scene) {
        sc.shaders.push(self.get_shader());
    }

    pub fn get(&mut self) -> Self {
        self.clone()
    }

}