use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::colors::types::{Color, InputChannel, NColor3, ProceduralTexture};
use crate::common::constants::MAX_REFLECTION_SAMPLES;
use crate::common::params::{Params, Value};
use crate::common::types::NormalizedF;
use crate::error::error::SysError;
use crate::light::light::{BaseLight, LightEnum};
use crate::geometry::geometry::Geometry;
use crate::ray::ray_context::RayContext;
use crate::scene::scene::Scene;
use crate::vector::vec3f::Vec3f;
use crate::shader::shader::{BaseShader, ShaderEnum};
use crate::vector::arithmetic::VectorArithmetic;
use crate::vector::constants::BLACK;
use crate::vector::types::Vector;

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct PhongShader {
    shader_type: String,
    id: String,
    #[serde(skip_serializing, skip_deserializing)]
    diffuse: InputChannel,
    opacity: NormalizedF,
    specularity_factor: NormalizedF,
    specularity_color: NColor3,
    specularity_opacity: NormalizedF,
    reflection: NormalizedF,
    reflection_glossiness: NormalizedF,
    reflection_max_samples: u16,
}





#[typetag::serde]
impl BaseShader for PhongShader {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn compute(&self, rc: &RayContext, light: &LightEnum) -> Result<NColor3, SysError> {
        let mut light_color = NColor3::default();
        let mut params = Params::default();
        params.set("origin".to_string(), Value::from_vec3f(rc.intersection_coordinate));
        params.set("obj_type".to_string(), Value::from_str(format!("{}", rc.intersected_geo_type.clone().unwrap())));
        params.set("obj_subtype".to_string(), Value::from_str(format!("{}", rc.intersected_geo_subtype.clone().unwrap())));
        params.set("obj_center".to_string(), Value::from_vec3f(rc.intersected_object_centroid.clone().unwrap()));
        let dir = light.get_displacement_vector(None, &rc.intersection_coordinate);
        let dir_n = dir.normalized();
        let res = light.compute_light(rc, &dir);
        match res {
            Some(color) => {
                light_color = light_color.add_with(&color)
            },
            None => {
                light_color = light_color.add_with(&Color::r_to_n(&BLACK))
            }
        }

        let mut final_color = &light_color * &self.diffuse.get_true_color(Some(&params)) * self.opacity;
        final_color = Color::n_clamp(&final_color);
        let spec_calculated = &self.compute_specularity(&dir_n, rc);
        return Ok(&final_color * &light_color.add_with(spec_calculated));
    }

    fn cast_reflection(&self) -> bool {
        self.reflection > 0.0
    }

    fn set_reflection_properties(&self, rc: &mut RayContext) {
        let normal = rc.get_proper_normal();
        rc.ray_dir = rc.ray_dir - 2.0*rc.ray_dir.dot(&normal) * &normal;
        rc.reflection_glossiness_samples = self.get_reflection_samples();
        rc.reflection_glossiness = self.reflection_glossiness
    }

    fn get_reflection_final_color(&self, ref_color: &NColor3) -> NColor3 {
        self.reflection * ref_color
    }
}

impl PhongShader {
    fn default() -> Self {
        Self {
            opacity: 1.0,
            reflection_max_samples: MAX_REFLECTION_SAMPLES,
            ..Default::default()
        }
    }

    pub fn get_shader(&self) -> ShaderEnum {
        ShaderEnum::Phong(self.clone())
    }

    pub fn new() -> Self {
        PhongShader::default()
    }

    pub fn new_with_params(id: &str, diffuse: NColor3, opacity: NormalizedF, spec_factor: NormalizedF,
                           spec_color: NColor3, spec_opacity: NormalizedF, reflection: NormalizedF, reflection_glossiness: NormalizedF) -> Self {
        Self {
            shader_type: "".to_string(),
            id: id.to_string(),
            diffuse: InputChannel::new_with_color(diffuse),
            opacity: opacity,
            specularity_factor: spec_factor,
            specularity_color: spec_color,
            specularity_opacity: spec_opacity,
            reflection: reflection,
            reflection_glossiness,
            reflection_max_samples: MAX_REFLECTION_SAMPLES,
        }
    }

    fn compute_specularity(&self, light_dir: &Vec3f, rc: &RayContext) -> NColor3 {
        let normal = rc.get_proper_normal();
        let halfway = (light_dir + &(&rc.camera_position - &rc.intersection_coordinate).normalized()).normalized();
        let spec = self.specularity_color * f64::max(0.0, VectorArithmetic::dot(&normal, &halfway)).powi(self.get_true_spec_factor(self.specularity_factor));
        spec * self.specularity_color
    }

    // converts the spec_factor value (which is limited from 0 to 1) to a proper
    // integer value (which is at most 257)
    fn get_true_spec_factor(&self, unit_factor: NormalizedF) -> i32 {
        ((1.0 - unit_factor) * 256.0 + 1.0) as i32
    }

    fn get_reflection_samples(&self) -> i8 {
        let true_ref = self.reflection_glossiness.clamp(0.0, 1.0);
        if true_ref == 0.0 {
            return 1;
        }
        (true_ref*self.reflection_max_samples as f64) as i8
    }

    pub fn set_specularity(&mut self, factor: NormalizedF, color: NColor3, opacity: NormalizedF) -> &mut Self {
        self.specularity_factor = factor;
        self.specularity_color = color;
        self.specularity_opacity = opacity;

        self
    }
    pub fn set_reflectivity(&mut self, reflectivity: NormalizedF) -> &mut Self {
        self.reflection = reflectivity;

        self
    }
    pub fn set_reflection(&mut self, reflection: NormalizedF, glossiness: NormalizedF) -> &mut Self {
        self.reflection = reflection;
        self.reflection_glossiness = glossiness;

        self
    }

    pub fn set_diffuse_color(&mut self, diffuse_color: NColor3) -> &mut Self {
        self.diffuse.set_color(Some(diffuse_color));
        self
    }
    pub fn set_diffuse_texture(&mut self, texture: Box<dyn ProceduralTexture>) -> &mut Self {
        self.diffuse.set_texture(texture);
        self
    }

    pub fn auto_id(&mut self) -> &mut Self {
        self.id = format!("phong::{}", Uuid::new_v4().to_string());
        self
    }

    pub fn assign_to(&mut self, geo: &mut Geometry) -> &mut Self {
        geo.assign_shader(&self.get_id());
        self
    }
    pub fn add_to_scene(&mut self, sc: &mut Scene) -> &mut Self {
        sc.shaders.push(self.get_shader());
        self
    }

    pub fn get(&self) -> Self {
        self.clone()
    }
}