use std::collections::HashMap;
use crate::common::primitive::PrimitiveType;

#[derive(Default, Clone)]
pub struct IndexOfEntities {
    _shaders_by_name:    HashMap<String, usize>,
    _geometries_by_name: HashMap<String, usize>,
    _cameras_by_name:    HashMap<String, usize>,
    _lights_by_name:    HashMap<String, usize>,
    _global_index: HashMap<usize, PrimitiveType>
}

impl IndexOfEntities {
    pub fn insert_shader(&mut self, name: String, index: usize) {
        self._shaders_by_name.insert(name, index);
    }

    pub fn insert_geometry(&mut self, name: String, index: usize) {
        self._geometries_by_name.insert(name, index);
    }

    pub fn insert_camera(&mut self, name: String, index: usize) {
        self._cameras_by_name.insert(name, index);
    }
    pub fn insert_light(&mut self, name: String, index: usize) {
        self._lights_by_name.insert(name, index);
    }

    pub fn add_global_index(&mut self, index: usize, primitive_type: PrimitiveType) {
        self._global_index.insert(index, primitive_type);
    }

    pub fn lookup_shader(&self, name: &str) -> Option<&usize> {
        self._shaders_by_name.get(name)
    }
    pub fn has_shader(&self, name: &str) -> bool {
        self._shaders_by_name.contains_key(name)
    }

    pub fn lookup_geometry(&self, name: &str) -> Option<&usize> {
        self._geometries_by_name.get(name)
    }

    pub fn lookup_camera(&self, name: &str) -> Option<&usize> {
        self._cameras_by_name.get(name)
    }

    pub fn iter_primitives(&self) -> &HashMap<usize, PrimitiveType> {
        &self._global_index
    }
}


