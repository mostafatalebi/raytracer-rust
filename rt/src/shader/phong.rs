use serde::{Deserialize, Serialize};
use crate::common::types::NormalizedF;
use crate::error::error::SysError;
use crate::light::light::{BaseLight, LightEnum};
use crate::ray::types::RayContext;
use crate::vector::vec3f::Vec3f;
use crate::shader::shader::BaseShader;
use crate::vector::arithmetic::VectorArithmetic;
use crate::vector::colors::{Color, NColor3};
use crate::vector::constants::BLACK;
use crate::vector::types::Vector;

#[derive(Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct PhongShader {
    shader_type: String,
    id: String,
    diffuse: NColor3,
    opacity: NormalizedF,
    specularity_factor: NormalizedF,
    specularity_color: NColor3,
    specularity_opacity: NormalizedF,
}

#[typetag::serde]
impl BaseShader for PhongShader {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn compute(&self, rc: &RayContext, light: &LightEnum) -> Result<NColor3, SysError> {
        let mut light_color = NColor3::default();
        let dir = light.get_displacement_vector(&rc.intersection_coordinate);
        let res = light.compute_light(rc, &dir);
        match res {
            Some(color) => {
                light_color = light_color.add_with(&color)
            },
            None => {
                light_color = light_color.add_with(&Color::r_to_n(&BLACK))
            }
        }

        let mut final_color = &light_color * &self.diffuse * self.opacity;
        final_color = Color::n_clamp(&final_color);
        let spec_calculated = &self.compute_specularity(&dir, rc);
        return Ok(&final_color * &light_color.add_with(spec_calculated));
    }
}

impl PhongShader {
    pub fn new(id: &str, diffuse: NColor3, opacity: NormalizedF, spec_factor: NormalizedF, spec_color: NColor3, spec_opacity: NormalizedF) -> Self {
        Self {
            shader_type: "".to_string(),
            id: id.to_string(),
            diffuse,
            opacity: opacity,
            specularity_factor: spec_factor,
            specularity_color: spec_color,
            specularity_opacity: spec_opacity,
        }
    }

    fn compute_specularity(&self, light_dir: &Vec3f, rc: &RayContext) -> NColor3 {
        let halfway = (light_dir + &(&rc.camera_position - &rc.intersection_coordinate).normalized()).normalized();
        let spec = self.specularity_color * f64::max(0.0, VectorArithmetic::dot(&rc.intersected_face_normal.unwrap(), &halfway)).powi(self.get_true_spec_factor(self.specularity_factor));
        spec * self.specularity_color
    }

    // converts the spec_factor value (which is limited from 0 to 1) to a proper
    // integer value (which is at most 257)
    fn get_true_spec_factor(&self, unit_factor: NormalizedF) -> i32 {
        ((1.0 - unit_factor) * 256.0 + 1.0) as i32
    }
}