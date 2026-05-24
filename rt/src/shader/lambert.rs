use serde::{Deserialize, Serialize};
use crate::common::types::NormalizedF;
use crate::error::error::SysError;
use crate::light::light::{BaseLight, LightEnum};
use crate::ray::types::RayContext;
use crate::vector::constants::{BLACK, GRAY};
use crate::vector::types::Vector;
use crate::shader::shader::BaseShader;
use crate::vector::colors::{Color, NColor3};

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
        let dir = light.get_displacement_vector(&collision.intersection_coordinate);
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
}

impl LambertShader {
    pub fn default() -> LambertShader {
        LambertShader{
            id: "lambert_001".to_string(),
            diffuse: Color::r_to_n(&GRAY),
            opacity: 1.0,
        }
    }

    pub fn new(id: &str, diffuse: NColor3, opacity: NormalizedF) -> LambertShader {
        LambertShader{
            id: String::from(id),
            diffuse, opacity
        }
    }

}