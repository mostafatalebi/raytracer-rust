use crate::camera::camera::StandardCamera;
use crate::colors::input::CheckeredTexture;
use crate::common::helpers::create_sphere;
use crate::light::ambient_light::AmbientLight;
use crate::light::light::LightEnum;
use crate::light::point_light::PointLight;
use crate::light::types::Attenuation;
use crate::object::geometry::Geometry;
use crate::object::helpers::create_cube;
use crate::object::procedural::create_procedural_sphere;
use crate::scene::render_settings::RenderSettings;
use crate::scene::scene::Scene;
use crate::shader::lambert::LambertShader;
use crate::shader::phong::PhongShader;
use crate::shader::shader::{BaseShader, ShaderEnum};
use crate::colors::types::{Color, NColor3};
use crate::object::geometry::GeometryType::Procedural;
use crate::vector::constants::{CAST_DAY, CYAN, FAINT_BLUE_WHITE, FAINT_GREEN, GRAY, MAGENTA, MUTED_PLUM, OCEAN_BLUE, RED, SKY_BLUE, SOFT_PINK, SUN, WHITE, WORLD_UP};
use crate::vector::types::{Vec2i, Vec3i, Vector, SENSOR_SQUARE_66};
use crate::vector::vec3f::Vec3f;

pub fn get_scene_001() -> Scene {
    let mut scene = Scene::default();

    let width = 500;
    let height = 500;

    let room_width: f64 = 50.0;
    let room_length: f64 = 50.0;
    let room_height: f64 = 100.0;
    let room_thickness: f64 = 5.0;

    let mut room_front_wall = create_cube(room_width, room_height, room_thickness);
    room_front_wall.transform.move_local(0.0,0.0,-50.0);
    LambertShader::new()
        .auto_id()
        .set_diffuse(Color::r_to_n(&FAINT_GREEN))
        .assign_to(&mut room_front_wall)
        .add_to_scene(&mut scene);


    let mut room_left_wall = create_cube(room_thickness, room_height, room_width);
    room_left_wall.transform.move_local(-room_width,0.0,0.0);
    LambertShader::new()
        .auto_id()
        .set_diffuse(Color::r_to_n(&FAINT_BLUE_WHITE))
        .assign_to(&mut room_left_wall)
        .add_to_scene(&mut scene);

    let mut room_right_wall = create_cube(room_thickness, room_height, room_width);
    room_right_wall.transform.move_local(room_width,0.0,0.0);
    LambertShader::new()
        .auto_id()
        .set_diffuse(Color::r_to_n(&SOFT_PINK))
        .assign_to(&mut room_right_wall)
        .add_to_scene(&mut scene);

    let mut room_ceiling = create_cube(room_width, room_thickness, room_width);
    room_ceiling.transform.move_local(0.0, room_height,0.0);
    LambertShader::new()
        .auto_id()
        .set_diffuse(Color::r_to_n(&SOFT_PINK))
        .assign_to(&mut room_ceiling)
        .add_to_scene(&mut scene);

    let mut room_floor = create_cube(room_width, room_thickness, room_width);
    room_floor.transform.move_local(0.0, 0.0,0.0);

    PhongShader::new()
        .auto_id()
        .set_diffuse_color(Color::r_to_n(&CYAN))
        .set_specularity(0.5, Color::r_to_n(&WHITE), 1.0)
        .set_reflection(0.5, 0.3)
        .assign_to(&mut room_floor)
        .add_to_scene(&mut scene);

    scene.geometries.push(room_front_wall);
    scene.geometries.push(room_left_wall);
    scene.geometries.push(room_right_wall);
    scene.geometries.push(room_ceiling);
    scene.geometries.push(room_floor);

    let radius: f64 = 10.0;
    let mut sphere_1 = create_procedural_sphere(Vec3f::new(30.0, 15.0, -35.0), radius);
    PhongShader::new()
        .auto_id()
         .set_diffuse_texture(Box::new(CheckeredTexture::new(24.0)))
        //.set_diffuse_color(Color::r_to_n(&NColor3::new(20.0,20.0,20.0)))
        .set_specularity(0.5, Color::r_to_n(&WHITE), 1.0)
        .set_reflection(0.3, 0.0)
        .assign_to(&mut sphere_1)
        .add_to_scene(&mut scene);
    scene.geometries.push(sphere_1);

    let mut sphere_2 = create_procedural_sphere(Vec3f::new(0.0, 15.0, -35.0), radius);
    PhongShader::new()
        .auto_id()
        .set_diffuse_color(Color::r_to_n(&SKY_BLUE))
        .set_specularity(0.5, Color::r_to_n(&WHITE), 1.0)
        .set_reflection(0.5, 0.0)
        .assign_to(&mut sphere_2)
        .add_to_scene(&mut scene);
    scene.geometries.push(sphere_2);


    let mut sphere_3 = create_procedural_sphere(Vec3f::new(-30.0, 15.0, -35.0), radius);
    PhongShader::new()
        .auto_id()
        .set_diffuse_color(Color::r_to_n(&MUTED_PLUM))
        .set_specularity(0.5, Color::r_to_n(&WHITE), 1.0)
        .set_reflection(0.1, 0.0)
        .assign_to(&mut sphere_3)
        .add_to_scene(&mut scene);
    scene.geometries.push(sphere_3);

    let radius: f64 = 10.0;
    let mut sphere_4 = create_procedural_sphere(Vec3f::new(30.0, 15.0, -5.0), radius);
    PhongShader::new()
        .auto_id()
        .set_diffuse_color(Color::r_to_n(&RED))
        .set_specularity(0.5, Color::r_to_n(&WHITE), 1.0)
        .set_reflection(0.5, 0.0)
        .assign_to(&mut sphere_4)
        .add_to_scene(&mut scene);
    scene.geometries.push(sphere_4);

    let mut sphere_5 = create_procedural_sphere(Vec3f::new(0.0, 15.0, -5.0), radius);
    PhongShader::new()
        .auto_id()
        .set_diffuse_color(Color::r_to_n(&NColor3::new(20.0,20.0,20.0)))
        .set_specularity(0.5, Color::r_to_n(&WHITE), 1.0)
        .set_reflection(0.7, 0.8)
        .assign_to(&mut sphere_5)
        .add_to_scene(&mut scene);
    scene.geometries.push(sphere_5);

    let mut sphere_6 = create_procedural_sphere(Vec3f::new(-30.0, 15.0, -5.0), radius);
    PhongShader::new()
        .auto_id()
        .set_diffuse_color(Color::r_to_n(&FAINT_GREEN))
        .set_specularity(0.5, Color::r_to_n(&WHITE), 1.0)
        .set_reflection(0.5, 0.0)
        .assign_to(&mut sphere_6)
        .add_to_scene(&mut scene);
    scene.geometries.push(sphere_6);


    let mut i = 1;
    for mut geo in scene.geometries.iter_mut() {
        geo.calc_all_normals();
        geo.apply_transformations();
        if geo.geometry_type == Procedural {
            if i != 5 {
                geo.render_attributes.renderable = true;
            }
            i += 1;
        }
    }

    let mut point_light = PointLight::new("point_light_1", 0.8, Color::r_to_n(&FAINT_BLUE_WHITE), Attenuation::Linear);
    point_light.transform.move_local(0.0, 25.0, 20.0);

    let ambient_light = AmbientLight::new("ambient_light_1", 0.35, Color::r_to_n(&WHITE));

    scene.lights.push(LightEnum::PointLight(point_light));
    // scene.lights.push(LightEnum::PointLight(point_light2));
    scene.lights.push(LightEnum::AmbientLight(ambient_light));

    let mut cam = StandardCamera::new(
        Vec2i::new(width, height),
        SENSOR_SQUARE_66,
        None,
        WORLD_UP,
        50.0,
        Some(Vec3f::new(0.0, 40.0, 80.0)),
    );

    cam.transform.move_local(0.0, 0.0, -30.0);
    cam.transform.rotate_local(-10.0, 0.0, 0.0);

    cam.configure();
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
    scene.cameras.push(cam);

    scene.render_settings = RenderSettings::default();
    scene.render_settings.file_name = "scene_room_001{#}".to_string();
    scene.render_settings.width = width as usize;
    scene.render_settings.height = height as usize;

    scene.apply_indexing();

    scene
}