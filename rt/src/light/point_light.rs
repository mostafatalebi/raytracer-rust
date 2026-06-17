use serde::{Deserialize, Serialize};
use crate::colors::types::NColor3;
use crate::common::id::Id;
use crate::common::transform::Transform;
use crate::light::area_light::AreaLight;
use crate::light::types::{Attenuation, POINT_LIGHT};
use crate::light::light::{BaseLight, Shadow};
use crate::ray::ray_context::RayContext;
use crate::vector::arithmetic::VectorArithmetic;
use crate::vector::types::Vector;
use crate::vector::vec3f::Vec3f;

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct PointLight {
    pub id: String,
    pub transform: Transform,
    pub intensity: f64,
    pub color: NColor3,
    pub attenuation: Attenuation,

    #[serde(default)]
    pub shadow_attributes: Shadow
}

#[typetag::serde]
impl BaseLight for PointLight {
    fn get_type(&self) -> i8 {
        POINT_LIGHT
    }
    fn get_attenuated_intensity(&self, dist: &Vec3f) -> f64 {
        match self.attenuation {
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

    // dir should not be normalized. It must be normalized after getting attenuation
    fn compute_light(&self, rc: &RayContext, dir: &Vec3f) -> Option<NColor3> {
        let attenuated_intensity = self.get_attenuated_intensity(dir);
        let normal = rc.get_proper_normal();
        let dir_n = dir.normalized();
        let dot = f64::max(0.0, VectorArithmetic::dot(&normal, &dir_n));

        let color = self.color.multiply_scalar(attenuated_intensity * dot);

        Some(color)
    }


    fn get_displacement_vector(&self, to: Option<&Vec3f>, from: &Vec3f) -> Vec3f {
        (&self.transform.local.translate - from)
    }

    fn get_transform(&self) -> Transform {
        self.transform.clone()
    }

    fn can_cast_shadow(&self) -> bool {
        self.shadow_attributes.enable
    }

    fn get_samples_count(&self) -> usize {
        0
    }

    fn get_samples(&self) -> Vec<Vec3f> {
       return vec![];
    }

}


impl PointLight {
    pub fn new(id: &str, intensity: f64, color: NColor3, attenuation: Attenuation) -> Self {
        Self {
            id: String::from(id),
            transform: Transform::default(),
            intensity,
            color,
            attenuation: attenuation,
            shadow_attributes: Shadow::default(),
        }
    }

    pub fn set_id(&mut self, id: String) -> &mut Self {
        self.id = id;
        self
    }

    pub fn set_attenuation(&mut self, attenuation: Attenuation) -> &mut Self {
        self.attenuation = attenuation;
        self
    }

    pub fn get(&self) -> PointLight {
        self.clone()
    }
}


impl Id for PointLight {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

