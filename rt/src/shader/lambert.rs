use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::colors::types::{Color, NColor3};
use crate::common::types::NormalizedF;
use crate::error::error::SysError;
use crate::light::light::{BaseLight, LightEnum};
use crate::geometry::geometry::Geometry;
use crate::ray::ray_context::RayContext;
use crate::scene::scene::Scene;
use crate::vector::constants::{BLACK, GRAY};
use crate::vector::types::Vector;
use crate::shader::shader::{BaseShader, ShaderEnum};
#[derive(Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct LambertShader {
    id: String,

    // RGB + Alpha
    diffuse: NColor3,
    opacity: NormalizedF,


}

#[typetag::serde]
impl BaseShader for LambertShader {
    fn get_id(&self) -> String {
        self.id.clone()
    }


    fn compute(&self, collision: &RayContext, light: &LightEnum) -> Result<NColor3, SysError> {
        let mut light_color = NColor3::default();
        let dir = light.get_displacement_vector(None, &collision.intersection_coordinate);
        let res = light.compute_light(collision, &dir);
        match res {
            Some(color) => {
                light_color = light_color.add_with(&color)
            },
            None => {
                light_color = light_color.add_with(&Color::r_to_n(&BLACK))
            }
        }

        let mut final_color: NColor3 = &light_color * &(&self.diffuse * self.opacity);
        final_color = Color::n_clamp(&final_color);
        return Ok(final_color);
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

impl LambertShader {

    pub fn get_shader(&self) -> ShaderEnum {
        ShaderEnum::Lambert(self.clone())
    }

    pub fn new() -> Self {
        LambertShader::default()
    }


    pub fn default() -> LambertShader {
        LambertShader{
            id: "lambert_001".to_string(),
            diffuse: Color::r_to_n(&GRAY),
            opacity: 1.0,
        }
    }

    pub fn new_from_params(id: &str, diffuse: NColor3, opacity: NormalizedF) -> LambertShader {
        LambertShader{
            id: String::from(id),
            diffuse, opacity
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
        self.id = format!("lambert::{}", Uuid::new_v4().to_string());
        self
    }

    pub fn assign_to(&mut self, geo: &mut Geometry) -> &mut Self {
        geo.assign_shader(&self.get_id());
        self
    }
    pub fn add_to_scene(&mut self, sc: &mut Scene) -> &mut Self {
        sc.shaders.push(self.get_shader());
        self
    }


    pub fn get(&mut self) -> Self {
        self.clone()
    }
}