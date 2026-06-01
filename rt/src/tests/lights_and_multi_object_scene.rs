use crate::camera::camera::StandardCamera;
use crate::common::helpers::create_sphere;
use crate::light::ambient_light::AmbientLight;
use crate::light::light::LightEnum;
use crate::light::point_light::PointLight;
use crate::light::types::Attenuation;
use crate::object::geometry::Geometry;
use crate::object::procedural::create_procedural_sphere;
use crate::scene::render_settings::RenderSettings;
use crate::scene::scene::Scene;
use crate::shader::lambert::LambertShader;
use crate::shader::phong::PhongShader;
use crate::shader::shader::{BaseShader, ShaderEnum};
use crate::colors::types::Color;
use crate::vector::constants::{CAST_DAY, CYAN, FAINT_BLUE_WHITE, FAINT_GREEN, GRAY, MAGENTA, SKY_BLUE, SUN, WHITE, WORLD_UP};
use crate::vector::types::{Vec2i, Vec3i, Vector, SENSOR_SQUARE_66};
use crate::vector::vec3f::Vec3f;

pub fn get_lights_and_multi_objects_scene() -> Scene {
    let mut s = Scene::default();

    let width = 1500;
    let height = 1500;

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
    cube.data.faces.push(Vec3i::new(4, 1, 5));
    cube.data.faces.push(Vec3i::new(4, 0, 1));
    cube.data.faces.push(Vec3i::new(3, 2, 6));
    cube.data.faces.push(Vec3i::new(3, 6, 7));
    cube.calc_all_normals();


    let lambert = ShaderEnum::Lambert(LambertShader::new_from_params("lambert_01", Color::r_to_n(&GRAY), 1.0));
    let lambert_green = ShaderEnum::Lambert(LambertShader::new_from_params("lambert_green", Color::r_to_n(&FAINT_GREEN), 1.0));
    let spec_color = Color::r_to_n(&WHITE);
    let phong = ShaderEnum::Phong(PhongShader::new_with_params("phong_01", Color::r_to_n(&MAGENTA), 1.0, 0.5, spec_color, 1.0,
                                                               0.3, 0.6));


    let mut sphere = create_procedural_sphere(Vec3f::new(0.0,0.0,0.0), 5.0);


    sphere.transform.move_local(-15.0, 0.0, 0.0);
    sphere.apply_transformations();
    sphere.assign_shader(&phong.get_id());
    s.geometries.push(sphere);

    cube.assign_shader(&lambert.get_id());
    let mut plane = Geometry::default();
    plane.id = "plane_01".to_string();
    plane.data.vertices.push(Vec3f::new(-25.0, 0.0, -15.0));
    plane.data.vertices.push(Vec3f::new(-25.0, 0.0, 15.0));
    plane.data.vertices.push(Vec3f::new(25.0, 0.0, 15.0));
    plane.data.vertices.push(Vec3f::new(25.0, 0.0, -15.0));

    plane.data.faces.push(Vec3i::new(0, 1, 3));
    plane.data.faces.push(Vec3i::new(1, 2, 3));

    plane.calc_all_normals();

    plane.assign_shader(&lambert_green.get_id());
    plane.transform.move_local(0.0,-5.0,-18.0);
    plane.transform.scale_local(2.0,1.0,1.0);
    plane.apply_transformations();


    let mut point_light = PointLight::new("point_light_1", 0.8, Color::r_to_n(&FAINT_BLUE_WHITE), Attenuation::Linear);
    point_light.transform.move_local(5.0, 30.0, 20.0);

    let mut point_light2 = PointLight::new("point_light_2", 5.0, Color::r_to_n(&SUN),Attenuation::Quadratic);

    point_light2.transform.move_local(-5.0, 20.0, 0.0);

    let ambient_light = AmbientLight::new("ambient_light_1", 0.35, Color::r_to_n(&WHITE));

    s.lights.push(LightEnum::PointLight(point_light));
    s.lights.push(LightEnum::PointLight(point_light2));
    s.lights.push(LightEnum::AmbientLight(ambient_light));

    let mut cam = StandardCamera::new(
        Vec2i::new(width, height),
        SENSOR_SQUARE_66,
        Some(Vec3f::new(0.0, 0.0, -5.0)),
        WORLD_UP,
        50.0,
        Some(Vec3f::new(0.0, 0.0, 40.0)),
    );

    s.shaders.push(lambert);
    s.shaders.push(phong);
    s.shaders.push(lambert_green);
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
    s.render_settings.file_name = "scene_001_lambert_phong{#}".to_string();
    s.render_settings.width = width as usize;
    s.render_settings.height = height as usize;

    s.apply_indexing();

    s
}