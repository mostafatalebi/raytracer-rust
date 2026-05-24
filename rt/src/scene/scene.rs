use serde::{Deserialize, Serialize};
use crate::camera::camera::{BaseCamera, StandardCamera};
use crate::error::error::SysError;
use crate::error::kinds::ErrorKind;
use crate::error::kinds::ErrorKind::BadSceneDescription;
use crate::index::index::IndexOfEntities;
use crate::light::light::{LightEnum};
use crate::object::geometry::{Geometry};
use crate::ray::types::RayContext;
use crate::scene::metadata::Metadata;
use crate::scene::render_settings::RenderSettings;
use crate::shader::shader::{BaseShader, ShaderEnum};

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct  Scene {
    pub version: String,
    #[serde(skip, default)]
    pub metadata: Metadata,
    pub geometries: Vec<Geometry>,
    pub lights: Vec<LightEnum>,
    pub cameras: Vec<StandardCamera>,
    pub shaders: Vec<ShaderEnum>,
    pub render_settings: RenderSettings,
    #[serde(skip, default)]
    _indices_db: IndexOfEntities,
}


impl Scene {

    pub fn load_from_file(file_addr: &str) -> Result<Scene, SysError> {
        let scene_json = std::fs::read_to_string(file_addr);

        if scene_json.is_err() {
            return Err(SysError::new(BadSceneDescription, scene_json.err().unwrap().to_string()));
        }

        match Scene::load(&scene_json.unwrap()) {
            Ok(mut s) => {
                s.apply_indexing();
                s.post_process();
                Ok(s)
            },
            Err(e) => Err(e)
        }

    }

    pub fn load(content: &str) -> Result<Scene, SysError> {
        let result = serde_json::from_str(content);

        if result.is_err() {
            Err(SysError::new(ErrorKind::BadSceneDescription, result.err().unwrap().to_string()))
        } else {
            let mut s: Scene = result.unwrap();
            s.post_process();
            Ok(s)
        }

    }

    pub fn validate_geometry(&self) -> Result<(), SysError> {
        for geometry in &self.geometries {
            if geometry.render_attributes.material_name.is_empty() {
                return Err(SysError::new_str(ErrorKind::InvalidMaterialType, "Invalid material"));
            } else {
                if !self._indices_db._shaders_by_name.contains_key(&geometry.render_attributes.material_name) {
                    return Err(SysError::new(ErrorKind::MaterialNotFound, format!("Material not found: {}", geometry.render_attributes.material_name)));
                }
            }
        }

        Ok(())
    }

    pub fn validate(&self) -> Result<(), SysError> {
        if self.version != "1.0".to_string() {
            Err(SysError::new_str(ErrorKind::InvalidVersion, "wrong version"))
        } else if self.geometries.is_empty() {
            Err(SysError::new_str(ErrorKind::NoGeometry, "the scene doesn't contain any geometry"))
        } else if self.lights.is_empty() {
            Err(SysError::new_str(ErrorKind::NoLight, "the scene doesn't contain any light"))
        } else if self.cameras.is_empty() {
            Err(SysError::new_str(ErrorKind::NoCamera, "the scene doesn't contain any camera"))
        } else {
            if let Err(e) = self.validate_geometry() {
                return Err(e)
            }
            Ok(())
        }
    }

    // creates indexed access tables for various objects
    // in a separated O(1) hash map table
    pub fn apply_indexing(&mut self) {
        for (k, obj) in self.geometries.iter().enumerate() {
            self._indices_db.insert_geometry(obj.id.clone(), k)
        }
        for (k, obj) in self.shaders.iter().enumerate() {
            self._indices_db.insert_shader(obj.get_id(), k)
        }
        for (k, obj) in self.cameras.iter().enumerate() {
            self._indices_db.insert_camera(obj.get_id(), k)
        }
    }

    fn post_process(&mut self) {
        if self.cameras.len() > 0 {
            for cam in self.cameras.iter_mut() {
                cam.set_res(self.render_settings.width, self.render_settings.height);
                cam.configure();
            }
        }
    }

    pub fn lookup_shader(&self, id: &str) -> Option<(usize, &ShaderEnum)> {
        if !self.shaders.is_empty() {
            if let Some(index) = self._indices_db.lookup_shader(id) {
                if self.shaders.len() > *index {
                    return Some((*index, self.shaders.get(*index).unwrap()))
                }
            }
        }
        None
    }
   pub fn lookup_shader_index(&self, id: &str) -> Option<usize> {
        if !self.shaders.is_empty() {
            if let Some(index) = self._indices_db.lookup_shader(id) {
                if self.shaders.len() > *index {
                    return Some(*index)
                }
            }
        }
        None
    }

    pub fn lookup_shader_by_index(&mut self, id: usize) -> Option<&mut ShaderEnum> {
        self.shaders.get_mut(id)
    }

    pub fn lookup_camera(&self, id: &str) -> Option<&StandardCamera> {
        if !self.cameras.is_empty() {
            if let Some(index) = self._indices_db.lookup_camera(id) {
                if self.shaders.len() > *index {
                    return Some(&self.cameras[*index])
                }
            }
        }
        None
    }

    // there is no defined mechanism for selecting
    // a default camera. For now, we just return the first
    // camera in the scene.
    pub fn get_default_camera(&self) -> Option<&StandardCamera> {
        if self.cameras.len() > 0 {
            return Some(&self.cameras[0])
        }
        None
    }

}


#[cfg(test)]
mod tests {
    use std::fs;
    use crate::scene::scene::Scene;

    #[test]
    fn load_scene_test() {
        let scene_json = fs::read_to_string("../resources/scene_examples/scene_basic.json");

        assert_eq!(false, scene_json.is_err(), "err={:?}", scene_json.err().unwrap());
        if !scene_json.is_err() {
            let result = Scene::load(&scene_json.unwrap());
            assert_eq!(false, result.is_err(), "err={:?}", result.err().unwrap());

            if result.is_ok() {
                let scene = result.unwrap();
                assert_eq!(scene.version, "0.1".to_string());
                assert_eq!(1, scene.geometries.len());
            }
        } else {
            println!("{e}:{:?}", scene_json.err(), e="Test Scene Failed");
        }
    }
}