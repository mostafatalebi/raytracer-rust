use crate::camera::camera::StandardCamera;
use crate::colors::procedural::CheckeredTexture;
use crate::light::ambient_light::AmbientLight;
use crate::light::light::LightEnum;
use crate::light::point_light::PointLight;
use crate::light::types::Attenuation;
use crate::geometry::helpers::create_cube;
use crate::geometry::procedural::create_procedural_sphere;
use crate::scene::render_settings::RenderSettings;
use crate::scene::scene::Scene;
use crate::shader::lambert::LambertShader;
use crate::shader::phong::PhongShader;
use crate::colors::types::{Color, NColor3};
use crate::geometry::geometry::GeometryType::Procedural;
use crate::vector::constants::{CYAN, FAINT_BLUE_WHITE, FAINT_GREEN, MUTED_PLUM, RED, SKY_BLUE, SOFT_PINK, WHITE, WORLD_UP};
use crate::vector::types::{Vec2i, SENSOR_SQUARE_66};
use crate::vector::vec3f::Vec3f;
use std::thread::available_parallelism;
use crate::camera::types::AntiAliasingMethod::{MonteCarlo, Uniform};

pub fn get_scene_room_and_sphere() -> Scene {
    let mut scene = Scene::default();

    let width = 1500;
    let height = 1500;

    let room_width: f64 = 50.0;
    let room_length: f64 = 50.0;
    let room_height: f64 = 100.0;
    let room_thickness: f64 = 5.0;

    let mut room_front_wall = create_cube(room_width, room_height, room_thickness);
    room_front_wall.transform.move_params(0.0, 0.0, -50.0);
    LambertShader::new()
        .auto_id()
        .set_diffuse(Color::r_to_n(&FAINT_GREEN))
        .assign_to(&mut room_front_wall)
        .add_to_scene(&mut scene);


    let mut room_left_wall = create_cube(room_thickness, room_height, room_width);
    room_left_wall.transform.move_params(-room_width, 0.0, 0.0);
    LambertShader::new()
        .auto_id()
        .set_diffuse(Color::r_to_n(&FAINT_BLUE_WHITE))
        .assign_to(&mut room_left_wall)
        .add_to_scene(&mut scene);

    let mut room_right_wall = create_cube(room_thickness, room_height, room_width);
    room_right_wall.transform.move_params(room_width, 0.0, 0.0);
    LambertShader::new()
        .auto_id()
        .set_diffuse(Color::r_to_n(&SOFT_PINK))
        .assign_to(&mut room_right_wall)
        .add_to_scene(&mut scene);

    let mut room_ceiling = create_cube(room_width, room_thickness, room_width);
    room_ceiling.transform.move_params(0.0, room_height, 0.0);
    LambertShader::new()
        .auto_id()
        .set_diffuse(Color::r_to_n(&SOFT_PINK))
        .assign_to(&mut room_ceiling)
        .add_to_scene(&mut scene);

    let mut room_floor = create_cube(room_width, room_thickness, room_width);
    room_floor.transform.move_params(0.0, 0.0, 0.0);

    PhongShader::new()
        .auto_id()
        .set_diffuse_color(Color::r_to_n(&WHITE))
        .set_specularity(0.5, Color::r_to_n(&WHITE), 1.0)
        .set_reflection(0.6, 0.5)
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
        .set_diffuse_color(Color::r_to_n(&Vec3f::new(26.0, 158.0, 214.0)))
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
        .set_diffuse_color(Color::r_to_n(&NColor3::new(60.0,60.0,60.0)))
        .set_specularity(0.5, Color::r_to_n(&WHITE), 1.0)
        .set_reflection(0.99, 0.5)
        .assign_to(&mut sphere_5)
        .add_to_scene(&mut scene);
    scene.geometries.push(sphere_5);

    let mut sphere_6 = create_procedural_sphere(Vec3f::new(-30.0, 15.0, -5.0), radius);
    PhongShader::new()
        .auto_id()
        .set_diffuse_color(Color::r_to_n(&NColor3::new(52.0, 140.0, 235.0)))
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

    let mut point_light = PointLight::new("point_light_1", 0.6, Color::r_to_n(&FAINT_BLUE_WHITE), Attenuation::Flat);
    point_light.transform.move_params(0.0, 25.0, 20.0);

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

    cam.transform.move_params(0.0, 0.0, -30.0);
    cam.transform.rotate_by(-10.0, 0.0, 0.0);

    cam.configure();
    scene.cameras.push(cam);

    scene.render_settings = RenderSettings::default();
    scene.render_settings.file_name = "scene_room_001{#}".to_string();
    scene.render_settings.width = width as usize;
    scene.render_settings.height = height as usize;
    scene.render_settings.multi_threading.enabled = true;
    if let Ok(num_of_threads) = available_parallelism() {
        scene.render_settings.multi_threading.count = usize::max(1, num_of_threads.get() - 1);
        // scene.render_settings.mt_num_of_threads = 8;
    }
    scene.render_settings.anti_aliasing.sample = 8;
    scene.render_settings.anti_aliasing.method = MonteCarlo;
    scene.apply_indexing();

    scene
}