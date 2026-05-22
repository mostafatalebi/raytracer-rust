use std::any::Any;
use std::collections::HashMap;
use crate::buffer::types::BufferIndex;
use crate::common::constants::EPS;
use crate::common::params::{Params, Value};
use crate::shader::shader::ShaderEnum;
use crate::vector::vec3f::Vec3f;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum CollisionTestType {
    #[default]
    CameraRay,
    ShadowRay,
}

/// this is an important type that holds
/// information about the point at which a ray
/// intersects with a surface. It keeps the data
/// associated with the collided object, shader,
/// custom param etc.
/// @todo shaders must be able to customize registering
/// data to this type via a modular interface (like a hook
/// in the tracer)
///
/// Any collision can be tested via an object of this kind. When
/// a collision happens, further data such as collided=true,
/// coordinate, distance etc. can also be set.
///
/// memory_buffer is also a simple storage for shaders to interacet
/// with, in case their computation involves some data look up
#[derive(Clone, Debug)]
pub struct RayCollision {
    pub ray_dir: Vec3f,
    // where the ray starts its journey (camera, another object, etc.)
    pub origin_coordinate: Vec3f,
    // used only for camera rays (not shadow rays)
    pub pixel_coordinate: Option<Vec3f>,
    pub buffer_index: Option<BufferIndex>,
    pub collided_object_index: Option<usize>,
    pub collided_face_index:   Option<usize>,
    pub collided_face_normal:         Option<Vec3f>,
    pub collided_face_vertex_normal:  Option<Vec3f>,
    pub collided: bool,
    pub collision_coordinate: Vec3f,
    pub ever_collided: bool,
    pub next_object_index: Option<usize>,
    pub shader_index: usize,
    pub collision_distance: f64,
    pub extra_params: Params,
    pub memory_buffer: Params,
    pub previous_closest_distance: f64,
    pub obj_receive_shadow: bool,
    pub test_type: CollisionTestType,
    pub is_in_shadow: bool,
}

impl RayCollision {

    pub fn new_for_camera_ray(origin: &Vec3f, buffer_index: Option<BufferIndex>) -> RayCollision {
        let mut rc = RayCollision::default();

        rc.origin_coordinate = origin.clone();
        rc.test_type = CollisionTestType::CameraRay;
        rc.buffer_index = buffer_index;
        rc.shader_index = usize::MAX;
        rc.collided_object_index = None;
        rc.collided_face_index = None;

        rc
    }
    pub fn new_for_shadow_ray(origin: &Vec3f) -> RayCollision {
        let mut rc = RayCollision::default();
        rc.origin_coordinate = origin.clone();
        rc.test_type = CollisionTestType::ShadowRay;
        rc.buffer_index = None;
        rc.shader_index = usize::MAX;
        rc.collided_object_index = None;
        rc.collided_face_index = None;
        rc.previous_closest_distance = f64::MAX;
        rc
    }

    pub fn reset_for_next_iteration(&mut self, obj_index: usize, receive_shadow: bool) {
        self.collided = false;
        self.next_object_index = Some(obj_index);
        self.obj_receive_shadow = receive_shadow;
    }

    pub fn save_to_memory(&mut self, key: String, value: Value) {
        self.memory_buffer.set(key, value);
    }

    pub fn get_from_memory(&mut self, key: String, value: Value) -> Option<&Value> {
        self.memory_buffer.get(key)
    }

    pub fn copy_from(&mut self, other: &RayCollision) {
        self.collided_object_index = other.collided_object_index;
        self.collided_face_index = other.collided_face_index;
        self.shader_index = other.shader_index;
        self.buffer_index = other.buffer_index.clone();
        self.collision_distance = other.collision_distance;
        self.collision_coordinate = other.collision_coordinate.clone();
        self.extra_params = other.extra_params.clone();
        self.memory_buffer = other.memory_buffer.clone();
    }

    pub fn has_collided(&self) -> bool {
        self.collided
    }

    pub fn has_ever_collided(&self) -> bool {
        self.ever_collided
    }


    pub fn reset_for_a_new_test(&mut self) {
        self.collided = false;
        self.collision_distance = 0.0;
        self.collision_coordinate = Vec3f::default();
    }
}



impl PartialEq for RayCollision {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Default for RayCollision {
    fn default() -> Self {
        let mut rc = RayCollision {
            ray_dir: Default::default(),
            origin_coordinate: Default::default(),
            pixel_coordinate: None,
            buffer_index: None,
            collided_object_index: None,
            collided_face_index: None,
            collided_face_normal: None,
            collided_face_vertex_normal: None,
            next_object_index: None,
            shader_index: 0,
            collision_distance: 0.0,
            collision_coordinate: Default::default(),
            extra_params: Default::default(),
            memory_buffer: Default::default(),
            collided: false,
            ever_collided: false,
            previous_closest_distance: f64::INFINITY,
            obj_receive_shadow: false,
            test_type: CollisionTestType::default(),
            is_in_shadow: false,
        };

        rc
    }
}