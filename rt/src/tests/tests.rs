#[cfg(test)]
mod test_dummy_renders {
    use std::sync::{Arc, Mutex, RwLock};
    use crate::render::renderer::Renderer;
    use crate::render::types::RenderRegion;
    use crate::tests::dummy_multi_object_scene::get_multi_objects_scene;
    use crate::tests::lights_and_multi_object_scene::get_lights_and_multi_objects_scene;
    use crate::tests::mocks::{get_dummy_scene, get_simple_cube_scene, get_simple_plane_scene};
    use crate::tests::room_and_spheres::get_scene_room_and_sphere;
    use crate::tests::scene_import_tea_cups::get_imported_scene_tea_table;
    use crate::tests::teapot_from_obj::get_scene_teapot_obj;
    use crate::vector::arithmetic::VectorArithmetic;
    use crate::vector::constants::WORLD_Z;
    use crate::vector::types::Vector;
    use crate::vector::vec3f::Vec3f;

    #[test]
    fn test_simple_plane() {
        let s = get_simple_plane_scene();

        let mut renderer = Renderer::new(Arc::new(RwLock::new(s)));
        let result = renderer.render(None);

        if result.is_err() {
            panic!("error={:?}", result.err().unwrap());
        }
    }

    #[test]
    fn test_simple_cube() {
        let s = get_simple_cube_scene();

        let mut renderer = Renderer::new(Arc::new(RwLock::new(s.clone())));
        let result = renderer.render(None);

        if result.is_err() {
            panic!("error={:?}", result.err().unwrap());
        }
        let cam = s.get_default_camera().unwrap();
        let expected_forward = cam.transform.rotate.rotate_vec3f(&WORLD_Z);
        let origin  = (Vec3f::default() - cam.transform.translate).normalized();
        let dot_product = VectorArithmetic::dot(&expected_forward, &origin);
    }

    #[test]
    fn test_simple_multiple_objects() {
        let s = get_multi_objects_scene();

        let mut renderer = Renderer::new(Arc::new(RwLock::new(s.clone())));
        let result = renderer.render(None);

        if result.is_err() {
            panic!("error={:?}", result.err().unwrap());
        }
        let cam = s.get_default_camera().unwrap();
        let expected_forward = cam.transform.rotate.rotate_vec3f(&WORLD_Z);
        let origin  = (Vec3f::default() - cam.transform.translate).normalized();
        let dot_product = VectorArithmetic::dot(&expected_forward, &origin);
    }
    #[test]
    fn test_get_lights_and_multi_objects_scene() {
        let s = get_lights_and_multi_objects_scene();

        let mut renderer = Renderer::new(Arc::new(RwLock::new(s.clone())));
        let result = renderer.render(None);

        if result.is_err() {
            panic!("error={:?}", result.err().unwrap());
        }
    }


    #[test]
    fn test_dummy_scene() {
        let s = get_dummy_scene();

        let mut renderer = Renderer::new(Arc::new(RwLock::new(s)));

        _ = renderer.render(None);
    }

    #[test]
    fn test_scene_room_and_sphere() {
        let s = get_scene_room_and_sphere();

        let mut renderer = Renderer::new(Arc::new(RwLock::new(s)));

        _ = renderer.render(None);
    }
    #[test]
    fn test_scene_teapot_obj() {
        let s = get_scene_teapot_obj();

        let mut renderer = Renderer::new(Arc::new(RwLock::new(s)));

        _ = renderer.render(None);
    }

    #[test]
    fn test_scene_teapot_obj_regional() {
        let s = get_scene_teapot_obj();

        let mut renderer = Renderer::new(Arc::new(RwLock::new(s)));

        _ = renderer.render(Some(RenderRegion::new(300, 300, 300, 300)));
    }
    #[test]
    fn test_json_get_imported_scene_tea_table_regional() {
        let s = get_imported_scene_tea_table();

        let mut renderer = Renderer::new(Arc::new(RwLock::new(s)));

        _ = renderer.render(Some(RenderRegion::new(300, 300, 300, 300)));
    }
}