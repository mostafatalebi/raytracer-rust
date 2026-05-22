use serde::{Deserialize, Serialize};
use crate::error::error::SysError;
use crate::error::kinds::ErrorKind;
use crate::light::light::{BaseLight, LightEnum};
use crate::ray::types::RayCollision;
use crate::vector::constants::{BLACK, GRAY};
use crate::vector::types::Vector;
use crate::shader::shader::BaseShader;
use crate::vector::colors::{NormalizedColor, Rgba};
#[derive(Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct LambertShader {
    id: String,

    // RGB + Alpha
    diffuse: NormalizedColor,
}

#[typetag::serde]
impl BaseShader for LambertShader {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn compute(&self, collision: &RayCollision, light: &LightEnum) -> Result<NormalizedColor, SysError> {
        let mut light_color = NormalizedColor::default();
        if let Some(face_normal) = collision.collided_face_normal {
            let dir = light.get_displacement_vector(&collision.collision_coordinate);
            let res = light.compute_light(collision, &dir);
            match res {
                Some(color) => {
                    light_color = light_color.add_with(&color)
                },
                None => {
                    light_color = light_color.add_with(&BLACK.to_normalized_color())
                }
            }

            let mut final_color = &light_color * &self.diffuse;
            final_color.clamp();
            return Ok(final_color);
        }


        Err(SysError::new_str(ErrorKind::FaceIdNotFound, "surface normal not found"))
    }
}

impl LambertShader {
    pub fn default() -> LambertShader {
        LambertShader{
            id: "lambert_001".to_string(),
            diffuse: GRAY.to_normalized_color(),
        }
    }

    pub fn new(id: &str, diffuse: NormalizedColor) -> LambertShader {
        LambertShader{
            id: String::from(id),
            diffuse,
        }
    }
}