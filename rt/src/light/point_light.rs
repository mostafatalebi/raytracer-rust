use serde::{Deserialize, Serialize};
use crate::common::transform::Transform;
use crate::common::types::NormalizedF;
use crate::light::types::{Attenuation, POINT_LIGHT};
use crate::light::light::{BaseLight, Shadow};
use crate::ray::types::RayContext;
use crate::vector::arithmetic::VectorArithmetic;
use crate::vector::colors::{NColor3};
use crate::vector::types::Vector;
use crate::vector::vec3f::Vec3f;

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct PointLight {
    pub id: String,
    pub transform: Transform,
    pub intensity: f64,
    pub color: NColor3,

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

    fn compute_light(&self, rc: &RayContext, dir: &Vec3f) -> Option<NColor3> {
        let attenuated_intensity = self.get_attenuated_intensity(dir);
        let dot = f64::max(0.0, VectorArithmetic::dot(&rc.intersected_face_normal.unwrap(), dir));
        let color = self.color.multiply_scalar(attenuated_intensity * dot);

        Some(color)
    }


    fn get_displacement_vector(&self, from: &Vec3f) -> Vec3f {
        (&self.transform.local.translate - from)
    }

    fn get_transform(&self) -> Transform {
        self.transform.clone()
    }

    fn can_cast_shadow(&self) -> bool {
        self.shadow_attributes.enable
    }
}


impl PointLight {
    pub fn new(id: &str, intensity: f64, color: NColor3, attenuation: Attenuation) -> Self {
        Self {
            id: String::from(id),
            transform: Transform::default(),
            intensity,
            color,
            attenuation_type: attenuation,
            shadow_attributes: Shadow::default(),
        }
    }
}