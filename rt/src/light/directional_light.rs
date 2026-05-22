use serde::{Deserialize, Serialize};
use crate::common::transform::Transform;
use crate::light::types::{Attenuation, DIRECTIONAL_LIGHT};
use crate::light::light::{BaseLight, Shadow};
use crate::ray::types::RayCollision;
use crate::vector::colors::Rgba;
use crate::vector::types::Vector;
use crate::vector::vec3f::Vec3f;
use crate::vector::vec4f::Vec4f;

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct DirectionalLight {
    id: String,
    transform: Transform,
    intensity: f64,
    color: Vec4f,
    shadow_attributes: Shadow,
    attenuation_type: Attenuation,
}

#[typetag::serde]
impl BaseLight for DirectionalLight {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_type(&self) -> i8 {
        DIRECTIONAL_LIGHT
    }

    fn get_attenuated_intensity(&self, dist: &Vec3f) -> f64 {
        match self.attenuation_type {
            Attenuation::Flat => self.intensity,
            Attenuation::Linear => {
                return self.intensity / dist.magnitude()
            },
            Attenuation::Quadratic => {
                return self.intensity * dist.length_squared()
            },
            Attenuation::Cube => {
                return self.intensity * dist.magnitude().powi(3)
            },
            _ => {
                self.intensity
            }
        }

    }

    fn compute_light(&self, rc: &RayCollision, dir: &Vec3f) -> Option<Rgba> {
        None
    }

    fn get_displacement_vector(&self, from: &Vec3f) -> Vec3f {
        return Vec3f::new(1.0,1.0,1.0);
    }

    fn get_transform(&self) -> Transform {
        self.transform.clone()
    }

    fn supports_shadow(&self) -> bool {
        true
    }
}
