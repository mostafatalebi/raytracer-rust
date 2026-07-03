use rand::{random, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use crate::bounding_box::aabb::AABB;
use crate::colors::types::{Color, NColor3};
use crate::common::id::Id;
use crate::common::transform::Transform;
use crate::geometry::geometry::{Geometry, GeometrySubType, GeometryType};
use crate::geometry::helpers::create_cube;
use crate::light::area_light::AreaLightShape::Rectangle;
use crate::light::types::{Attenuation, POINT_LIGHT};
use crate::light::light::{BaseLight, Shadow};
use crate::ray::ray_context::RayContext;
use crate::shader::flat::FlatShader;
use crate::shader::shader::BaseShader;
use crate::vector::arithmetic::VectorArithmetic;
use crate::vector::constants::{WHITE, WORLD_RIGHT, WORLD_UP};
use crate::vector::types::Vector;
use crate::vector::vec3f::Vec3f;


#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub enum AreaLightShape {
    #[default]
    #[serde(rename = "rectangle")]
    Rectangle,
    #[serde(rename = "disk")]
    Disk
}

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct AreaLight {
    pub id: String,
    pub transform: Transform,
    pub intensity: f64,
    pub color: NColor3,

    // seen in reflections
    pub shape_color: NColor3,

    pub attenuation: Attenuation,
    #[serde(default)]
    pub shadow_attributes: Shadow,


    pub shape: AreaLightShape,
    // either radius or width&height are used, based on shape
    pub width: f64,
    pub height: f64,
    pub radius: f64,

    pub shadow_samples: usize,
    pub flip_normals: bool,

    #[serde(rename="visible")]
    visible: bool,

    #[serde(skip)]
    pub centroid: Vec3f,
    #[serde(skip)]
    pub bounding_box: AABB,
    // these are generated automatically
    #[serde(skip)]
    _u_dir: Vec3f,
    #[serde(skip)]
    _v_dir: Vec3f,
    #[serde(skip)]
    _u_vec: Vec3f,
    #[serde(skip)]
    _v_vec: Vec3f,
    #[serde(skip)]
    _normal: Vec3f,
}

#[typetag::serde]
impl BaseLight for AreaLight {
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

    fn compute_light(&self, rc: &RayContext, dir: &Vec3f) -> Option<NColor3> {
        self.compute_light_for_a_single_sample(rc, dir)
    }

    fn get_displacement_vector(&self, to: Option<&Vec3f>, from: &Vec3f) -> Vec3f {
        if to.is_none() {
            return (&self.transform.translate - from)
        }
        return to.unwrap() - from
    }


    fn get_transform(&self) -> Transform {
        self.transform.clone()
    }

    fn can_cast_shadow(&self) -> bool {
        self.shadow_attributes.enable
    }

    fn get_samples_count(&self) -> usize {
        self.shadow_samples
    }

    fn get_samples(&self) -> Vec<Vec3f> {
        self.get_sampled_coordinates()
    }
}


impl AreaLight {
    pub fn new() -> Self {
        Self {
            id: String::from(""),
            transform: Transform::default(),
            centroid: Vec3f::default(),
            bounding_box: AABB::default(),
            intensity: 1.0,
            color: Color::r_to_n(&WHITE),
            shape_color: Color::r_to_n(&WHITE),
            attenuation: Attenuation::Linear,
            shadow_attributes: Shadow::default(),
            width: 1.0,
            height: 1.0,
            shape: Rectangle,
            radius: 0.0,
            shadow_samples: 1,
            visible: true,
            flip_normals: false,
            _u_dir: Default::default(),
            _v_dir: Default::default(),
            _u_vec: Default::default(),
            _v_vec: Default::default(),
            _normal: Default::default(),
        }
    }

    pub fn set_id(&mut self, id: String) -> &mut Self {
        self.id = id;
        self
    }
    pub fn set_radius(&mut self, r: f64) {
        self.radius = r;
    }

    pub fn set_shape(&mut self, shape: AreaLightShape) -> &mut Self {
        self.shape = shape;
        self
    }

    pub fn set_dimensions(&mut self, w: f64, h: f64) -> &mut Self {
        self.width = w;
        self.height = h;

        self
    }


    pub fn set_intensity(&mut self, intensity: f64) -> &mut Self {
        self.intensity = intensity;
        self
    }

    pub fn set_color(&mut self, color: &NColor3) -> &mut Self {
        self.color = *color;
        self
    }

    pub fn set_shape_color(&mut self, color: &NColor3) -> &mut Self {
        self.color = *color;
        self
    }
    pub fn set_shape_visibility(&mut self, v: bool) -> &mut Self {
        self.visible = v;
        self
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn get_visibility_geometry(&self) -> (Geometry, FlatShader) {
        let mut geom = create_cube(self.width, self.height, 0.01);
        geom.geometry_subtype = GeometrySubType::AreaLightShape;
        geom.transform = self.transform.clone();
        geom.render_attributes.shadows.receive = false;
        let flat_shader = FlatShader::default().auto_id().set_diffuse(self.shape_color).get();
        geom.assign_shader(&flat_shader.get_id());
        geom.prepare_geometry();
        geom.calc_all_normals();
        (geom, flat_shader)
    }

    pub fn set_attenuation(&mut self, attenuation: Attenuation) -> &mut Self {
        self.attenuation = attenuation;
        self
    }
    pub fn set_shadow_samples(&mut self, s: usize) -> &mut Self {
        self.shadow_samples = s;
        self
    }

    pub fn get(&self) -> AreaLight {

        self.clone()
    }


    fn compute_light_for_a_single_sample(&self, rc: &RayContext, dir: &Vec3f) -> Option<NColor3> {
        let attenuated_intensity = self.get_attenuated_intensity(dir);
        let normal = rc.get_proper_normal();
        let dir_n = dir.normalized();
        let dot = f64::max(0.0, VectorArithmetic::dot(&normal, &dir_n));
        let light_dot = f64::max(0.0, self._normal.dot(&dir_n.multiply_scalar(-1.0)));
        let color = self.color.multiply_scalar(attenuated_intensity * dot * light_dot);

        Some(color)
    }

    pub fn apply_transformation(&mut self) {
        let local_u = WORLD_RIGHT;
        let local_v = WORLD_UP;

        let result = self.transform.get_u_v_dir(self.width, self.height, &local_u, &local_v);
        let normal = result.2.cross3(&result.3).normalized();
        self._u_dir = result.0;
        self._v_dir = result.1;
        self._u_vec = result.2;
        self._v_vec = result.3;
        self._normal = normal;
    }

    pub fn get_sampled_coordinates(&self) -> Vec<Vec3f>{

        let mut sampled: Vec<Vec3f> = vec![Vec3f::default(); self.shadow_samples];

        for i in 0..self.shadow_samples {
            let u_rand = thread_rng().gen_range(-0.5..0.5);
            let v_rand = thread_rng().gen_range(-0.5..0.5);
            let p = self.transform.translate + (self._u_vec * u_rand) + (self._v_vec * v_rand);
            sampled[i] = p
        }

        sampled
    }

    /// flips the direction of the area light
    pub fn flip(&mut self) {
        self.flip_normals = !self.flip_normals;
        self._normal = self._normal * (-1.0);
    }
}


impl Id for AreaLight {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}
