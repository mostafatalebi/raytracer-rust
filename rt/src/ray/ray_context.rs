use crate::buffer::types::BufferIndex;
use crate::colors::types::NColor3;
use crate::common::constants::{EPS, MAX_REFLECTION_SAMPLES};
use crate::common::params::{Params};
use crate::common::types::NormalizedF;
use crate::vector::vec3f::Vec3f;
use crate::geometry::geometry::{GeometrySubType, GeometryType};
use crate::vector::constants::BLACK;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum RayType {
    #[default]
    CameraRay,
    ShadowRay,
    ReflectionRay,
    RefractionRay,
}

/// this is an important type that holds
/// information about the point at which a ray
/// intersects with a surface. It keeps the data
/// associated with the collided geometry, shader,
/// custom param etc.
/// @todo shaders must be able to customize registering
/// data to this type via a modular interface (like a hook
/// in the tracer)
///
/// Any collision can be tested via an geometry of this kind. When
/// a collision happens, further data such as collided=true,
/// coordinate, distance etc. can also be set.
///
/// memory_buffer is also a simple storage for shaders to interacet
/// with, in case their computation involves some data look up
#[derive(Clone, Debug)]
pub struct RayContext {
    pub camera_position: Vec3f,
    pub ray_dir: Vec3f,
    pub inv_ray_dir: Vec3f,
    // where the ray starts its journey (camera, another geometry, etc.)
    pub origin_coordinate: Vec3f,
    // used only for camera rays (not shadow rays)
    pub pixel_coordinate: Option<Vec3f>,
    pub buffer_index: Option<BufferIndex>,
    pub intersected_object_index: Option<usize>,
    pub intersected_object_centroid: Option<Vec3f>,
    pub intersected_geo_type: Option<GeometryType>,
    pub intersected_geo_subtype: Option<GeometrySubType>,
    pub intersected_face_index: Option<usize>,
    pub intersected_face_normal: Option<Vec3f>,
    pub intersected_vertex_normal: Option<Vec3f>,
    pub intersection_to_nearest_edge_distance: u64,
    pub intersected: bool,
    pub intersection_coordinate: Vec3f,
    pub intersection_coordinate_barycentric_u: f64,
    pub intersection_coordinate_barycentric_v: f64,
    pub ever_intersected: bool,
    pub next_object_index: Option<usize>,
    pub shader_index: usize,
    pub intersection_distance: f64,
    pub extra_params: Option<Params>,
    pub memory_buffer: Option<Params>,
    pub previous_closest_distance: f64,
    pub obj_cast_reflection: bool,
    pub reflection_max_depth: u8,
    pub reflection_max_sample: u16,
    pub reflection_current_level: u8,
    pub reflection_glossiness_samples: i8,
    pub reflection_glossiness: f64,
    pub ray_type: RayType,
    pub is_in_shadow: bool,

    pub do_smooth: bool,

    pub area_light_samples_count: usize,
    pub area_light_sampled_points: Option<Vec<Vec3f>>,
    pub area_light_sampled_shadow_multiplier: NormalizedF,
}

impl RayContext {
    pub fn new_for_camera_ray(origin: &Vec3f, buffer_index: Option<BufferIndex>) -> RayContext {
        let mut rc = RayContext::default();

        rc.origin_coordinate = origin.clone();
        rc.ray_type = RayType::CameraRay;
        rc.buffer_index = buffer_index;
        rc.shader_index = usize::MAX;
        rc.intersected_object_index = None;
        rc.intersected_face_index = None;

        rc
    }
    pub fn new_for_secondary_ray(ray_type: RayType, origin: &Vec3f, f_n: Option<Vec3f>, v_n: Option<Vec3f>) -> RayContext {
        let mut rc = RayContext::default();
        rc.origin_coordinate = origin.clone();
        rc.ray_type = RayType::ShadowRay;
        rc.buffer_index = None;
        rc.shader_index = usize::MAX;
        rc.intersected_object_index = None;
        rc.intersected_face_index = None;
        rc.intersected_face_normal = f_n;
        rc.intersected_vertex_normal = v_n;
        rc.previous_closest_distance = f64::MAX;
        rc.ray_type = ray_type;
        rc
    }

    pub fn fork_for_reflection(&self, origin: &Vec3f, f_n: Option<Vec3f>, v_n: Option<Vec3f>) -> RayContext {
        let mut rc = self.clone();
        rc.ray_type = RayType::ReflectionRay;
        rc.origin_coordinate = origin.clone();
        rc.buffer_index = None;
        rc.shader_index = usize::MAX;
        rc.intersected_face_normal = f_n;
        rc.intersected_vertex_normal = v_n;
        rc.previous_closest_distance = f64::MAX;

        rc.reflection_max_depth = 2;
        rc
    }

    pub fn reset_for_next_iteration(&mut self, ray_dir: Vec3f, pixel_coordinate: Option<Vec3f>) {
        self.ray_dir = ray_dir;
        self.pixel_coordinate = pixel_coordinate;
        self.intersected_object_index = None;
        self.intersected_object_centroid = None;
        self.intersected_geo_type = None;
        self.intersected_geo_subtype = None;
        self.intersected_face_index = None;
        self.intersected_face_normal = None;
        self.intersected_vertex_normal = None;

        self.intersected = false;
        self.intersection_coordinate = Vec3f::default();
        self.ever_intersected = false;
        self.next_object_index = None;
        self.shader_index = 0;
        self.intersection_distance = f64::MAX;

        self.previous_closest_distance = f64::MAX;
        self.reflection_current_level = 0;
        self.is_in_shadow = false;

        self.do_smooth = false;
    }

    pub fn refresh_for_new_object_test(&mut self, obj_index: usize, receive_shadow: bool) {
        self.intersected = false;
        self.next_object_index = Some(obj_index);
    }



    pub fn has_ever_intersected(&self) -> bool {
        self.ever_intersected
    }

    pub fn is_closest_so_far(&self, curr_dist: f64) -> bool {
        curr_dist > EPS && curr_dist < self.previous_closest_distance
    }

    pub fn is_camera_ray(&self) -> bool {
        self.ray_type == RayType::CameraRay
    }

    pub fn is_shadow_ray(&self) -> bool {
        self.ray_type == RayType::CameraRay
    }
    pub fn is_reflection_ray(&self) -> bool {
        self.ray_type == RayType::ReflectionRay
    }

    pub fn is_refraction_ray(&self) -> bool {
        self.ray_type == RayType::RefractionRay
    }

    pub fn update_intersection(
        &mut self,
        obj_index: Option<usize>,
        face_index: Option<usize>,
        face_normal: Option<Vec3f>,
        vertex_normal: Option<Vec3f>,
        dist: f64,
        inters_crd: Vec3f,
        do_smooth: bool,
    ) -> &mut Self {
        self.previous_closest_distance = dist;
        self.intersection_coordinate = inters_crd;
        self.intersection_distance = dist;
        self.intersected_object_index = obj_index;
        self.intersected_face_index = face_index;
        self.intersected_face_normal = face_normal;
        self.intersected_vertex_normal = vertex_normal;
        self.intersected = true;
        self.do_smooth = do_smooth;
        if !self.ever_intersected {
            self.ever_intersected = true;
        }

        self

    }

    pub fn can_continue_for_reflection(&self) -> bool {
        self.obj_cast_reflection && self.reflection_max_depth > 0 && self.reflection_current_level < self.reflection_max_depth-1
    }

    pub fn increment_reflection_level(&mut self) {
        self.reflection_current_level += 1;
    }

    pub fn calc_inv_ray_dir(&mut self) {
        self.inv_ray_dir[0] = 1.0 / self.ray_dir[0];
        self.inv_ray_dir[1] = 1.0 / self.ray_dir[1];
        self.inv_ray_dir[2] = 1.0 / self.ray_dir[2];
    }

    // based on a number of settings, which are
    // geo type (poly or proc) and smooth setting,
    // it returns either face normals (smooth=false
    // and/or geo type=polygon). For geo_type=procedural,
    // it returns the default face_normal which is also
    // used for procedural normals as well
    pub fn get_proper_normal(&self) -> Vec3f {
        let mut normal = self.intersected_face_normal.unwrap();
        if self.do_smooth && self.intersected_geo_type.clone().unwrap() == GeometryType::Polygon {
        }
        normal = self.intersected_vertex_normal.unwrap();

        normal
    }
}

impl PartialEq for RayContext {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Default for RayContext {
    fn default() -> Self {
        RayContext {
            camera_position: Default::default(),
            ray_dir: Default::default(),
            inv_ray_dir: Default::default(),
            origin_coordinate: Default::default(),
            pixel_coordinate: None,
            buffer_index: None,
            intersected_object_index: None,
            intersected_object_centroid: None,
            intersected_geo_type: None,
            intersected_geo_subtype: None,
            intersected_face_index: None,
            intersected_face_normal: None,
            intersected_vertex_normal: None,
            intersection_to_nearest_edge_distance: 0,
            next_object_index: None,
            shader_index: 0,
            intersection_distance: 0.0,
            intersection_coordinate: Default::default(),
            intersection_coordinate_barycentric_u: 0.0,
            intersection_coordinate_barycentric_v: 0.0,
            extra_params: None,
            memory_buffer: None,
            intersected: false,
            ever_intersected: false,
            previous_closest_distance: f64::INFINITY,
            obj_cast_reflection: true,
            reflection_max_depth: 2,
            reflection_max_sample: MAX_REFLECTION_SAMPLES,
            reflection_current_level: 0,
            reflection_glossiness_samples: 0,
            reflection_glossiness: 0.0,
            ray_type: RayType::default(),
            is_in_shadow: false,
            do_smooth: false,
            area_light_samples_count: 0,
            area_light_sampled_points: None,
            area_light_sampled_shadow_multiplier: 0.0,
        }
    }
}
