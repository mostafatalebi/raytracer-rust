use serde::{Deserialize, Serialize};
use typetag::__private22::inventory::collect;
use uuid::Uuid;
use crate::colors::types::{Color, NColor3};
use crate::common::types::NormalizedF;
use crate::error::error::SysError;
use crate::light::light::{BaseLight, LightEnum};
use crate::object::geometry::Geometry;
use crate::ray::types::RayContext;
use crate::scene::scene::Scene;
use crate::shader::lambert::LambertShader;
use crate::vector::constants::{BLACK, GRAY};
use crate::vector::types::Vector;
use crate::shader::shader::{BaseShader, ShaderEnum};
use crate::vector::vec3f::Vec3f;

#[derive(Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct WireframeShader {
    id: String,
    // RGB + Alpha
    diffuse: NColor3,
    opacity: NormalizedF,
    edge_width: NormalizedF,
    edge_color: NColor3,
    distance_threshold: f64,
}

#[typetag::serde]
impl BaseShader for WireframeShader {
    fn get_id(&self) -> String {
        self.id.clone()
    }


    fn compute(&self, collision: &RayContext, light: &LightEnum) -> Result<NColor3, SysError> {
        if collision.intersection_to_nearest_edge_distance as f64 <= self.distance_threshold {
            self.compute_lambert(&self.edge_color, collision, light)
        } else {
            self.compute_lambert(&self.diffuse, collision, light)
        }
    }




    fn cast_reflection(&self) -> bool {
        false
    }

    fn set_reflection_properties(&self, rc: &mut RayContext) {

    }

    fn get_reflection_final_color(&self, ref_color: &NColor3) -> NColor3 {
        ref_color.clone()
    }
}

impl WireframeShader {

    pub fn get_shader(&self) -> ShaderEnum {
        ShaderEnum::Wireframe(self.clone())
    }

    pub fn new() -> Self {
        WireframeShader::default()
    }


    pub fn default() -> WireframeShader {
        WireframeShader{
            id: "lambert_001".to_string(),
            diffuse: Color::r_to_n(&Vec3f::new(210.0, 210.0, 210.0)),
            opacity: 1.0,
            edge_width: 0.1,
            edge_color: Color::r_to_n(&BLACK),
            distance_threshold: 1.0,
        }
    }


    pub fn set_diffuse(&mut self, diffuse_color: NColor3) -> &mut Self {
        self.diffuse = diffuse_color;
        self
    }

    pub fn set_edge(&mut self, width: NormalizedF, color: NColor3) -> &mut Self {
        self.edge_width = width;
        self.edge_color = color;
        self
    }

    pub fn set_id(&mut self, id: &str) -> &mut Self {
        self.id = String::from(id);
        self
    }
    pub fn auto_id(&mut self) -> &mut Self {
        self.id = format!("lambert::{}", Uuid::new_v4().to_string());
        self
    }

    pub fn assign_to(&mut self, geo: &mut Geometry) -> &mut Self {
        geo.assign_shader(&self.get_id());
        self
    }
    pub fn add_to_scene(&self, sc: &mut Scene) {
        sc.shaders.push(self.get_shader());
    }

    fn compute_lambert(&self, diffuse: &NColor3, collision: &RayContext, light: &LightEnum) -> Result<NColor3, SysError> {
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

        let mut final_color: NColor3 = &light_color * &(diffuse * self.opacity);
        final_color = Color::n_clamp(&final_color);
        return Ok(final_color);
    }

}