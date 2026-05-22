use serde::{Deserialize, Serialize};
use crate::common::transform::Transform;
use crate::light::types::{Attenuation, POINT_LIGHT};
use crate::light::light::{BaseLight, Shadow};
use crate::light::types::Attenuation::Flat;
use crate::ray::types::RayCollision;
use crate::vector::arithmetic::VectorArithmetic;
use crate::vector::colors::{NormalizedColor, Rgba};
use crate::vector::types::Vector;
use crate::vector::vec3f::Vec3f;

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct PointLight {
    pub id: String,
    pub transform: Transform,
    pub intensity: f64,
    pub color: NormalizedColor,

    pub attenuation_type: Attenuation,

    #[serde(default)]
    pub shadow_attributes: Shadow
}

#[typetag::serde]
impl BaseLight for PointLight {
    fn get_id(&self) -> String {
        self.id.clone()
    }
    fn get_type(&self) -> i8 {
        POINT_LIGHT
    }
    fn get_attenuated_intensity(&self, dist: &Vec3f) -> f64 {
        match self.attenuation_type {
            Attenuation::Flat => self.intensity,
            Attenuation::Linear => {
                self.intensity / dist.magnitude()
            },
            Attenuation::Quadratic => {
                self.intensity / dist.length_squared()
            },
            Attenuation::Cube => {
                self.intensity / dist.magnitude().powi(3)
            },
            _ => {
                self.intensity
            }
        }
    }

    fn compute_light(&self, rc: &RayCollision, dir: &Vec3f) -> Option<Rgba> {
        let attenuated_intensity = self.get_attenuated_intensity(dir);
        let dot = f64::max(0.0, VectorArithmetic::dot(&rc.collided_face_normal.unwrap(), dir));
        let mut color = self.color.multiply_scalar(attenuated_intensity * dot);
        color[3] = 1.0;
        
        Some(color)
    }


    fn get_displacement_vector(&self, from: &Vec3f) -> Vec3f {
        (&self.transform.local.translate - from)
    }

    fn get_transform(&self) -> Transform {
        self.transform.clone()
    }

    fn supports_shadow(&self) -> bool {
        true
    }
}


impl PointLight {
    pub fn new(id: &str, intensity: f64, color: Rgba, atten: Attenuation) -> Self {
        Self {
            id: String::from(id),
            transform: Transform::default(),
            intensity,
            color,
            attenuation_type: atten,
            shadow_attributes: Default::default(),
        }
    }
}