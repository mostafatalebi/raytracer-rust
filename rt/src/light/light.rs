use serde::{Deserialize, Serialize};
use crate::common::transform::Transform;
use crate::light::ambient_light::AmbientLight;
use crate::light::directional_light::DirectionalLight;
use crate::light::point_light::PointLight;
use crate::ray::types::RayCollision;
use crate::vector::colors::Rgba;
use crate::vector::vec3f::Vec3f;

#[typetag::serde]
pub trait BaseLight {
    fn get_id(&self) -> String;

    fn get_type(&self) -> i8;

    fn get_attenuated_intensity(&self, dist: &Vec3f) -> f64;

    fn compute_light(&self, rc: &RayCollision, dir: &Vec3f) -> Option<Rgba>;

    fn get_displacement_vector(&self, from: &Vec3f) -> Vec3f;

    fn get_transform(&self) -> Transform;

    fn supports_shadow(&self) -> bool;
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "light_type")]
pub enum LightEnum {
    #[serde(rename = "point")]
    PointLight(PointLight),

    #[serde(rename = "directional")]
    DirectionalLight(DirectionalLight),

    #[serde(rename = "ambient")]
    AmbientLight(AmbientLight),
}

#[typetag::serde]
impl BaseLight for LightEnum {
    fn get_id(&self) -> String {
        match self {
            LightEnum::PointLight(light) => light.get_id(),
            LightEnum::DirectionalLight(light) => light.get_id(),
            LightEnum::AmbientLight(light) => light.get_id()
        }
    }
    fn get_type(&self) -> i8 {
        match self {
            LightEnum::PointLight(light) => light.get_type(),
            LightEnum::DirectionalLight(light) => light.get_type(),
            LightEnum::AmbientLight(light) => light.get_type()
        }
    }

    fn compute_light(&self, rc: &RayCollision, dir: &Vec3f) -> Option<Rgba> {
        match self {
            LightEnum::PointLight(light) => light.compute_light(rc, dir),
            LightEnum::DirectionalLight(light) => light.compute_light(rc, dir),
            LightEnum::AmbientLight(light) => light.compute_light(rc, dir)
        }
    }
    fn get_transform(&self) -> Transform {
        match self {
            LightEnum::PointLight(light) => light.get_transform(),
            LightEnum::DirectionalLight(light) => light.get_transform(),
            LightEnum::AmbientLight(light) => light.get_transform(),
        }
    }

    fn get_displacement_vector(&self, from: &Vec3f) -> Vec3f {
        match self {
            LightEnum::PointLight(light) => light.get_displacement_vector(from),
            LightEnum::DirectionalLight(light) => light.get_displacement_vector(from),
            LightEnum::AmbientLight(light) => light.get_displacement_vector(from)
        }
    }

    fn get_attenuated_intensity(&self, dist: &Vec3f) -> f64 {
        match self {
            LightEnum::PointLight(light) => light.get_attenuated_intensity(dist),
            LightEnum::DirectionalLight(light) => light.get_attenuated_intensity(dist),
            LightEnum::AmbientLight(light) => light.get_attenuated_intensity(dist)
        }
    }

    fn supports_shadow(&self) -> bool {
        match self {
            LightEnum::PointLight(light) => light.supports_shadow(),
            LightEnum::DirectionalLight(light) => light.supports_shadow(),
            LightEnum::AmbientLight(light) => light.supports_shadow(),
        }
    }
}


#[derive(Default, Deserialize, Serialize, Clone)]
pub struct Shadow {
    enable: bool,

    // Not for the first version
    // controls how [non] grainy the
    // final shadow is rendered. More samples
    // means longer render time but cleaner [feathered] edges
    samples: i8,

    // Not for the first version
    // controls the size of the light source
    // and affects shadow softness. Larger light sources
    // create softer shadows
    radius: i32,
}

impl Shadow {
    pub fn default() -> Shadow {
        Shadow{enable: true, samples: 1, radius: 1}
    }
}