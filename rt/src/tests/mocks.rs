use crate::camera::camera::StandardCamera;
use crate::object::geometry::Geometry;
use crate::scene::render_settings::RenderSettings;
use crate::scene::scene::Scene;
use crate::shader::face_shader::FaceShader;
use crate::shader::shader::ShaderEnum;
use crate::vector::constants::WORLD_UP;
use crate::vector::types::{Vec2i, Vec3i, Vector, SENSOR_SIZE_35, SENSOR_SQUARE_66};
use crate::vector::vec3f::Vec3f;

pub fn get_simple_plane_scene() -> Scene {
    let mut s = Scene::default();

    let width = 200;
    let height = 200;

    let mut plane = Geometry::default();
    plane.id = "plane_01".to_string();

    plane.data.vertices.push(Vec3f::new(-5.0, 0.0, -5.0)); // 0
    plane.data.vertices.push(Vec3f::new(5.0, 0.0, -5.0)); // 1
    plane.data.vertices.push(Vec3f::new(5.0, 0.0, 5.0)); // 2
    plane.data.vertices.push(Vec3f::new(-5.0, 0.0, 5.0)); // 3

    plane.data.vertices.push(Vec3f::new(0.0, 0.0, 0.0));   // 4 (center)

    plane.data.faces.push(Vec3i::new(0, 1, 4));
    plane.data.faces.push(Vec3i::new(1, 2, 4));
    plane.data.faces.push(Vec3i::new(2, 3, 4));
    plane.data.faces.push(Vec3i::new(3, 0, 4));

    plane.assign_shader("face_shader_001");


    let mut cam = StandardCamera::new(
        Vec2i::new(width, height),
        SENSOR_SQUARE_66,
        Vec3f::new(0.0, 0.0, 5.0),
        WORLD_UP,
        50.0,
        Some(Vec3f::new(0.0, 0.0, -10.0)),
    );

    let face_shader = FaceShader::new("face_shader_001", 1.0);
    s.shaders.push(ShaderEnum::FaceShader(face_shader));
    plane.transform.rotate_local(0.0, 0.0, 0.0);
    // plane.apply_transformations();
    cam.lock_to(plane.clone().transform.local.translate);
    cam.transform.move_world(0.0,0.0,-5.0);
    cam.transform.set_world_rotate(Vec3f::new(0.0,0.0, 0.0));
    cam.configure();

    s.geometries.push(plane);
    s.cameras.push(cam);

    s.render_settings = RenderSettings::default();
    s.render_settings.file_name = "plane_simple_render_{#}".to_string();
    s.render_settings.width = width as usize;
    s.render_settings.height = height as usize;

    s.apply_indexing();
    s
}



pub fn get_simple_cube_scene() -> Scene {
    let mut s = Scene::default();

    let width = 200;
    let height = 200;

    let mut cube = Geometry::default();
    cube.id = "cube_01".to_string();

    // small square plane centered in front of camera
    cube.data.vertices.push(Vec3f::new(-5.0, 5.0, 5.0));
    cube.data.vertices.push(Vec3f::new(5.0, 5.0, 5.0));
    cube.data.vertices.push(Vec3f::new(5.0, -5.0, 5.0));
    cube.data.vertices.push(Vec3f::new(-5.0, -5.0, 5.0));
    cube.data.vertices.push(Vec3f::new(-5.0, 5.0, -5.0));
    cube.data.vertices.push(Vec3f::new(5.0, 5.0, -5.0));
    cube.data.vertices.push(Vec3f::new(5.0, -5.0, -5.0));
    cube.data.vertices.push(Vec3f::new(-5.0, -5.0, -5.0));

    cube.data.faces.push(Vec3i::new(0, 1, 2));
    cube.data.faces.push(Vec3i::new(0, 2, 3));
    cube.data.faces.push(Vec3i::new(5, 4, 7));
    cube.data.faces.push(Vec3i::new(5, 7, 6));
    cube.data.faces.push(Vec3i::new(4, 0, 3));
    cube.data.faces.push(Vec3i::new(4, 3, 7));
    cube.data.faces.push(Vec3i::new(1, 5, 6));
    cube.data.faces.push(Vec3i::new(1, 6, 2));
    cube.data.faces.push(Vec3i::new(4, 5, 1));
    cube.data.faces.push(Vec3i::new(4, 1, 0));
    cube.data.faces.push(Vec3i::new(3, 2, 6));
    cube.data.faces.push(Vec3i::new(3, 6, 7));


    cube.data.face_normals.push(Vec3f::new(0.0, 0.0, 1.0));
    cube.data.face_normals.push(Vec3f::new(0.0, 0.0, 1.0));
    cube.data.face_normals.push(Vec3f::new(0.0, 0.0, -1.0));
    cube.data.face_normals.push(Vec3f::new(0.0, 0.0, -1.0));
    cube.data.face_normals.push(Vec3f::new(-1.0, 0.0, 0.0));
    cube.data.face_normals.push(Vec3f::new(-1.0, 0.0, 0.0));
    cube.data.face_normals.push(Vec3f::new(1.0, 0.0, 0.0));
    cube.data.face_normals.push(Vec3f::new(1.0, 0.0, 0.0));
    cube.data.face_normals.push(Vec3f::new(0.0, 1.0, 0.0));
    cube.data.face_normals.push(Vec3f::new(0.0, 1.0, 0.0));
    cube.data.face_normals.push(Vec3f::new(0.0, -1.0, 0.0));
    cube.data.face_normals.push(Vec3f::new(0.0, -1.0, 0.0));
    cube.assign_shader("face_shader_001");


    let mut cam = StandardCamera::new(
        Vec2i::new(width, height),
        SENSOR_SQUARE_66,
        Vec3f::new(0.0, 0.0, -5.0),
        WORLD_UP,
        50.0,
        Some(Vec3f::new(0.0, 0.0, 40.0)),
    );

    let face_shader = FaceShader::new("face_shader_001", 1.0);
    s.shaders.push(ShaderEnum::FaceShader(face_shader));
    cam.lock_to(cube.clone().transform.local.translate);
    cam.transform.move_local(0.0,10.0,0.0);
    // cam.pan(0.0, -5.0);
    cam.configure();
    s.geometries.push(cube.clone());
    println!("forward: {:?}", cam.get_forward());
    println!("forward: {:?}", cam.get_forward());
    println!("right:   {:?}", cam.get_right());
    println!("up:      {:?}", cam.get_up());
    println!("fov:      {:?}", cam.get_fov());
    println!("aspect_ratio:      {:?}", cam.get_aspect_ratio());
    let center_pixel = cam.clone().get_pixel_coordinates(100, 100);
    println!("center pixel_coord: {:?}", center_pixel);
    let pixel_coord = cam.get_pixel_coordinates(100, 100);
    let ray_dir = (&pixel_coord - &cam.transform.local.translate).normalized();
    println!("ray_dir center: {:?}", ray_dir);
    s.cameras.push(cam);

    s.render_settings = RenderSettings::default();
    s.render_settings.file_name = "cube_simple_render_{#}".to_string();
    s.render_settings.width = width as usize;
    s.render_settings.height = height as usize;

    s.apply_indexing();

    s
}
pub fn get_dummy_scene() -> Scene {
    let mut s = Scene::default();
    let width = 36*10;
    let height = 24*10;
    let mut plane = Geometry::default();
    plane.id = "cube_01".to_string();
    plane.data.vertices.push(Vec3f::new(-1.0, 1.0, 0.0));
    plane.data.vertices.push(Vec3f::new(1.0, 1.0, 0.0));
    plane.data.vertices.push(Vec3f::new(1.0, -1.0,  0.0));
    plane.data.vertices.push(Vec3f::new(-1.0, -1.0,  0.0));
    plane.data.faces.push(Vec3i::new(0, 1, 3));
    plane.data.faces.push(Vec3i::new(1, 2, 3));
    plane.data.face_normals.push(Vec3f::new(0.0, 1.0, 0.0));
    plane.data.face_normals.push(Vec3f::new(0.0, 1.0, 0.0));
    plane.data.face_normals.push(Vec3f::new(0.0, 1.0, 0.0));
    plane.data.face_normals.push(Vec3f::new(0.0, 1.0, 0.0));
    let mut cam = StandardCamera::new(Vec2i::new(width, height),
                                      SENSOR_SIZE_35, Vec3f::new(0.0, 0.0, 0.0),
                                      WORLD_UP,10.0, Some(Vec3f::new(20.0,0.0,6.0)));
    // plane.attributes.move_world(0.0, -50.0, 0.0);
    s.geometries.push(plane);
    s.cameras.push(cam);
    s.render_settings = RenderSettings::default();
    s.render_settings.file_name = "dummy_render_{#}".to_string();
    s.render_settings.width = width as usize;
    s.render_settings.height = height as usize;
    s
}