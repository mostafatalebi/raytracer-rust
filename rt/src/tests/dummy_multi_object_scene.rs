use crate::camera::camera::StandardCamera;
use crate::geometry::geometry::Geometry;
use crate::scene::render_settings::RenderSettings;
use crate::scene::scene::Scene;
use crate::shader::face_shader::FaceShader;
use crate::shader::shader::{BaseShader, ShaderEnum};
use crate::vector::constants::WORLD_UP;
use crate::vector::types::{Vec2i, Vec3i, Vector, SENSOR_SQUARE_66};
use crate::vector::vec3f::Vec3f;

pub fn get_multi_objects_scene() -> Scene {
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
    cube.assign_shader("face_shader_cube_1");

    let mut plane = Geometry::default();
    plane.id = "plane_01".to_string();
    plane.data.vertices.push(Vec3f::new(-25.0, 0.0, -15.0));
    plane.data.vertices.push(Vec3f::new(-25.0, 0.0, 15.0));
    plane.data.vertices.push(Vec3f::new(25.0, 0.0, 15.0));
    plane.data.vertices.push(Vec3f::new(25.0, 0.0, -15.0));

    plane.data.faces.push(Vec3i::new(0, 1, 3));
    plane.data.faces.push(Vec3i::new(1, 2, 3));

    plane.data.face_normals.push(Vec3f::new(0.0, 0.0, 1.0));
    plane.data.face_normals.push(Vec3f::new(0.0, 0.0, 1.0));
    plane.assign_shader("face_shader_plane_1");
    plane.transform.move_local(0.0,-10.0,-18.0);
    plane.apply_transformations();
    let mut cam = StandardCamera::new(
        Vec2i::new(width, height),
        SENSOR_SQUARE_66,
        Some(Vec3f::new(0.0, 0.0, -5.0)),
        WORLD_UP,
        50.0,
        Some(Vec3f::new(0.0, 0.0, 40.0)),
    );

    let face_shader_cube = FaceShader::new("face_shader_cube_1", 1.0);
    let face_shader_plane = FaceShader::new("face_shader_plane_1", 1.0);
    cube.assign_shader(&face_shader_cube.get_id());
    plane.assign_shader(&face_shader_plane.get_id());
    s.shaders.push(ShaderEnum::FaceShader(face_shader_cube));
    s.shaders.push(ShaderEnum::FaceShader(face_shader_plane));
    cam.lock_to(cube.clone().transform.local.translate);
    cam.transform.move_local(0.0,10.0,0.0);
    // cam.pan(0.0, -5.0);
    cam.configure();
    s.geometries.push(cube.clone());
    s.geometries.push(plane.clone());
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
    s.render_settings.file_name = "multi_objects_scene_render_{#}".to_string();
    s.render_settings.width = width as usize;
    s.render_settings.height = height as usize;

    s.apply_indexing();

    s
}