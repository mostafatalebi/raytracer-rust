use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct IndexOfEntities {
    pub _shaders_by_name:    HashMap<String, usize>,
    pub _geometries_by_name: HashMap<String, usize>,
    pub _cameras_by_name:    HashMap<String, usize>,
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

    pub fn lookup_shader(&self, name: &str) -> Option<&usize> {
        self._shaders_by_name.get(name)
    }

    pub fn lookup_geometry(&self, name: &str) -> Option<&usize> {
        self._geometries_by_name.get(name)
    }

    pub fn lookup_camera(&self, name: &str) -> Option<&usize> {
        self._cameras_by_name.get(name)
    }
}