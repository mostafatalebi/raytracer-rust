use serde::{Deserialize, Serialize};
use crate::colors::types::NColor3;
use crate::common::id::Id;
use crate::common::transform::Transform;
use crate::light::types::{Attenuation, AMBIENT_LIGHT};
use crate::light::light::BaseLight;
use crate::light::types::Attenuation::Flat;
use crate::ray::ray_context::RayContext;
use crate::vector::types::Vector;
use crate::vector::vec3f::Vec3f;

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct AmbientLight {
    id: String,
    intensity: f64,
    color: NColor3,
    dir: Vec3f,
    attenuation_type: Attenuation,
    // since ambient returns the same
    // light intensity for all rays
    _cached: NColor3,
}



#[typetag::serde]
impl BaseLight for AmbientLight {

    fn get_type(&self) -> i8 {
        AMBIENT_LIGHT
    }

    fn get_attenuated_intensity(&self, dist: &Vec3f) -> f64 {
        match self.attenuation_type {
            Flat => self.intensity,
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

    fn compute_light(&self, rc: &RayContext, dir: &Vec3f) -> Option<NColor3> {
        Some(self._cached)
    }

    fn get_displacement_vector(&self, to: Option<&Vec3f>, from: &Vec3f) -> Vec3f {
        return Vec3f::new(1.0,1.0,1.0);
    }

    fn get_transform(&self) -> Transform {
        Transform::default()
    }

    fn can_cast_shadow(&self) -> bool {
        false
    }

    fn get_samples_count(&self) -> usize {
        0
    }

    fn get_samples(&self) -> Vec<Vec3f> {
        return vec![];
    }
}

impl AmbientLight {
    pub fn new(id: &str, intensity: f64, color: NColor3) -> Self {
        let mut a  = Self {
            id: String::from(id),
            intensity, color,
            attenuation_type: Attenuation::Flat,
            dir: Vec3f::new(1.0,1.0,1.0),
            _cached: color.multiply_scalar(intensity)
        };


        a
    }


}


impl Id for AmbientLight {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}
