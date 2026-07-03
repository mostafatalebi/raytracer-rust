use crate::light::deserializers::deserialize_area_light;
use serde::{Deserialize, Serialize};
use crate::colors::types::NColor3;
use crate::common::id::Id;
use crate::common::transform::Transform;
use crate::light::ambient_light::AmbientLight;
use crate::light::area_light::AreaLight;
use crate::light::directional_light::DirectionalLight;
use crate::light::point_light::PointLight;
use crate::ray::ray_context::RayContext;
use crate::vector::vec3f::Vec3f;

#[typetag::serde]
pub trait BaseLight {
    fn get_type(&self) -> i8;

    fn get_attenuated_intensity(&self, dist: &Vec3f) -> f64;

    fn compute_light(&self, rc: &RayContext, dir: &Vec3f) -> Option<NColor3>;

    fn get_displacement_vector(&self, to: Option<&Vec3f>, from: &Vec3f) -> Vec3f;

    fn get_transform(&self) -> Transform;

    fn can_cast_shadow(&self) -> bool;

    fn get_samples_count(&self) -> usize;
    fn get_samples(&self) -> Vec<Vec3f>;
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

    #[serde(rename = "area", deserialize_with = "deserialize_area_light")]
    AreaLight(AreaLight),
}

#[typetag::serde]
impl BaseLight for LightEnum {

    fn get_type(&self) -> i8 {
        match self {
            LightEnum::PointLight(light) => light.get_type(),
            LightEnum::DirectionalLight(light) => light.get_type(),
            LightEnum::AmbientLight(light) => light.get_type(),
            LightEnum::AreaLight(light) => light.get_type()
        }
    }

    fn compute_light(&self, rc: &RayContext, dir: &Vec3f) -> Option<NColor3> {
        match self {
            LightEnum::PointLight(light) => light.compute_light(rc, dir),
            LightEnum::DirectionalLight(light) => light.compute_light(rc, dir),
            LightEnum::AmbientLight(light) => light.compute_light(rc, dir),
            LightEnum::AreaLight(light) => light.compute_light(rc, dir)
        }
    }
    fn get_transform(&self) -> Transform {
        match self {
            LightEnum::PointLight(light) => light.get_transform(),
            LightEnum::DirectionalLight(light) => light.get_transform(),
            LightEnum::AmbientLight(light) => light.get_transform(),
            LightEnum::AreaLight(light) => light.get_transform()
        }
    }

    fn get_displacement_vector(&self, to: Option<&Vec3f>, from: &Vec3f) -> Vec3f {
        match self {
            LightEnum::PointLight(light) => light.get_displacement_vector(to, from),
            LightEnum::DirectionalLight(light) => light.get_displacement_vector(to, from),
            LightEnum::AmbientLight(light) => light.get_displacement_vector(to, from),
            LightEnum::AreaLight(light) => light.get_displacement_vector(to, from)
        }
    }

    fn get_attenuated_intensity(&self, dist: &Vec3f) -> f64 {
        match self {
            LightEnum::PointLight(light) => light.get_attenuated_intensity(dist),
            LightEnum::DirectionalLight(light) => light.get_attenuated_intensity(dist),
            LightEnum::AmbientLight(light) => light.get_attenuated_intensity(dist),
            LightEnum::AreaLight(light) => light.get_attenuated_intensity(dist)
        }
    }

    fn can_cast_shadow(&self) -> bool {
        match self {
            LightEnum::PointLight(light) => light.can_cast_shadow(),
            LightEnum::DirectionalLight(light) => light.can_cast_shadow(),
            LightEnum::AmbientLight(light) => light.can_cast_shadow(),
            LightEnum::AreaLight(light) => light.can_cast_shadow()
        }
    }
    fn get_samples_count(&self) -> usize {
        match self {
            LightEnum::PointLight(light) => light.get_samples_count(),
            LightEnum::DirectionalLight(light) => light.get_samples_count(),
            LightEnum::AmbientLight(light) => light.get_samples_count(),
            LightEnum::AreaLight(light) => light.get_samples_count()
        }
    }fn get_samples(&self) -> Vec<Vec3f> {
        match self {
            LightEnum::PointLight(light) => light.get_samples(),
            LightEnum::DirectionalLight(light) => light.get_samples(),
            LightEnum::AmbientLight(light) => light.get_samples(),
            LightEnum::AreaLight(light) => light.get_samples()
        }
    }}

impl Id for LightEnum {
    fn get_id(&self) -> String {
        match self {
            LightEnum::PointLight(light) => light.get_id(),
            LightEnum::DirectionalLight(light) => light.get_id(),
            LightEnum::AmbientLight(light) => light.get_id(),
            LightEnum::AreaLight(light) => light.get_id()
        }
    }

}



#[derive(Default, Deserialize, Serialize, Clone)]
pub struct Shadow {
    pub enable: bool,

    // Not for the first version
    // controls how [non] grainy the
    // final shadow is rendered. More samples
    // means longer render time but cleaner [feathered] edges
    pub samples: i8,

    // Not for the first version
    // controls the size of the light source
    // and affects shadow softness. Larger light sources
    // create softer shadows
    pub radius: i32,
}

impl Shadow {
    pub fn default() -> Shadow {
        Shadow{enable: true, samples: 1, radius: 1}
    }
}