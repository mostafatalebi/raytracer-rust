#[cfg(test)]
mod test_dummy_renders {
    use std::sync::{Arc, Mutex, RwLock};
    use crate::render::renderer::Renderer;
    use crate::tests::dummy_multi_object_scene::get_multi_objects_scene;
    use crate::tests::lights_and_multi_object_scene::get_lights_and_multi_objects_scene;
    use crate::tests::mocks::{get_dummy_scene, get_simple_cube_scene, get_simple_plane_scene};
    use crate::vector::arithmetic::VectorArithmetic;
    use crate::vector::constants::WORLD_Z;
    use crate::vector::types::Vector;
    use crate::vector::vec3f::Vec3f;

    #[test]
    fn test_simple_plane() {
        let s = get_simple_plane_scene();

        let mut renderer = Renderer::new(Arc::new(RwLock::new(s)));
        let result = renderer.render();

        if result.is_err() {
            panic!("error={:?}", result.err().unwrap());
        }
    }

    #[test]
    fn test_simple_cube() {
        let s = get_simple_cube_scene();

        let mut renderer = Renderer::new(Arc::new(RwLock::new(s.clone())));
        let result = renderer.render();

        if result.is_err() {
            panic!("error={:?}", result.err().unwrap());
        }
        let cam = s.get_default_camera().unwrap();
        let expected_forward = cam.transform.local.rotate.rotate_vec3f(&WORLD_Z);
        let origin  = (Vec3f::default() - cam.transform.local.translate).normalized();
        let dot_product = VectorArithmetic::dot(&expected_forward, &origin);
    }

    #[test]
    fn test_simple_multiple_objects() {
        let s = get_multi_objects_scene();

        let mut renderer = Renderer::new(Arc::new(RwLock::new(s.clone())));
        let result = renderer.render();

        if result.is_err() {
            panic!("error={:?}", result.err().unwrap());
        }
        let cam = s.get_default_camera().unwrap();
        let expected_forward = cam.transform.local.rotate.rotate_vec3f(&WORLD_Z);
        let origin  = (Vec3f::default() - cam.transform.local.translate).normalized();
        let dot_product = VectorArithmetic::dot(&expected_forward, &origin);
    }
    #[test]
    fn test_get_lights_and_multi_objects_scene() {
        let s = get_lights_and_multi_objects_scene();

        let mut renderer = Renderer::new(Arc::new(RwLock::new(s.clone())));
        let result = renderer.render();

        if result.is_err() {
            panic!("error={:?}", result.err().unwrap());
        }
    }


    #[test]
    fn test_dummy_scene() {
        let s = get_dummy_scene();

        let mut renderer = Renderer::new(Arc::new(RwLock::new(s)));

        renderer.render();
    }
}