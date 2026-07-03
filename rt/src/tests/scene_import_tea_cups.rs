use std::fs;
use crate::scene::render_settings::RenderSettings;
use crate::scene::scene::Scene;
use std::thread::available_parallelism;
use crate::camera::types::AntiAliasingMethod::{MonteCarlo};
use crate::scene::environment::Environment;

pub fn get_imported_scene_tea_table() -> Scene {
    let scene_json = fs::read_to_string("../resources/scene_examples/scene_tea_and_cups.json");
    let mut scene: Scene = Scene::default();
    assert_eq!(false, scene_json.is_err(), "err={:?}", scene_json.err().unwrap());
    if !scene_json.is_err() {
        let result = Scene::load(&scene_json.unwrap());
        assert_eq!(false, result.is_err(), "err={:?}", result.err().unwrap());

        if result.is_ok() {
            scene = result.unwrap();
            assert_eq!(scene.version, "0.1".to_string());
            assert_eq!(true, scene.geometries.len() > 0);
        }
    } else {
        panic!("scene loading failed: {}", scene_json.unwrap_err());
    }

    scene.apply_indexing();
    scene
}