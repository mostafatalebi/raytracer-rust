use crate::camera::camera::StandardCamera;
use crate::light::ambient_light::AmbientLight;
use crate::light::light::LightEnum;
use crate::light::point_light::PointLight;
use crate::light::types::Attenuation;
use crate::scene::render_settings::RenderSettings;
use crate::scene::scene::Scene;
use crate::shader::lambert::LambertShader;
use crate::colors::types::{Color};
use crate::vector::constants::{FAINT_BLUE_WHITE, SOFT_PINK, WHITE, WORLD_UP};
use crate::vector::types::{Vec2i, SENSOR_SQUARE_66};
use crate::vector::vec3f::Vec3f;
use std::thread::available_parallelism;
use crate::camera::types::AntiAliasingMethod::{MonteCarlo};
use crate::light::area_light::AreaLight;
use crate::scene::environment::Environment;
use crate::scene::obj_importer::ObjImporter;
use crate::shader::phong::PhongShader;
use crate::shader::shader::{BaseShader};

pub fn get_scene_teapot_obj() -> Scene {
    let mut scene = Scene::default();

    let width = 600;
    let height = 600;



    let mut lambert = LambertShader::new();
    lambert.auto_id()
        .set_diffuse(Color::r_to_n(&SOFT_PINK))
        .add_to_scene(&mut scene);

    let obj_file = ObjImporter::parse("../resources/obj_files/table_mug_cup.obj", true);

    if obj_file.is_err() {
        panic!("{:?}", obj_file.err());
    }


    let mut geometries = obj_file.unwrap();
    let mut i = 1;
    for mut geo in geometries.iter_mut() {
        // geo.prepare_geometry();
        geo.apply_transformations();
        geo.enable_smooth();
    }

    let floor = LambertShader::new()
        .auto_id()
        .set_diffuse(Color::r_to_n(&Vec3f::new(200.0,200.0,200.0)))
        .add_to_scene(&mut scene).get();

    let kettle = PhongShader::new()
        .auto_id()
        .set_diffuse_color(Color::r_to_n(&Vec3f::new(40.0, 40.0, 40.0)))
        .set_specularity(0.5, Color::r_to_n(&WHITE), 1.0)
        .set_reflection(0.6, 0.6)
        .add_to_scene(&mut scene).get();

    let table_top = PhongShader::new()
        .auto_id()
        .set_diffuse_color(Color::r_to_n(&Vec3f::new(122.0, 88.2, 33.2)))
        .set_specularity(0.5, Color::r_to_n(&WHITE), 1.0)
        .set_reflection(0.2, 0.8)
        .add_to_scene(&mut scene).get();


    let cup_white = PhongShader::new()
        .auto_id()
        .set_diffuse_color(Color::r_to_n(&Vec3f::new(245.0, 245.2, 245.2)))
        .set_specularity(0.6, Color::r_to_n(&WHITE), 1.0)
        .set_reflection(0.2, 0.2)
        .add_to_scene(&mut scene).get();

    let cup_light_orange = PhongShader::new()
        .auto_id()
        .set_diffuse_color(Color::r_to_n(&Vec3f::new(237.0, 151.0, 85.0)))
        .set_specularity(0.6, Color::r_to_n(&Vec3f::new(242.0, 216.0, 196.0)), 1.0)
        .set_reflection(0.2, 0.2)
        .add_to_scene(&mut scene).get();

    let mug_light_green = PhongShader::new()
        .auto_id()
        .set_diffuse_color(Color::r_to_n(&Vec3f::new(141.0, 181.0, 109.0)))
        .set_specularity(0.6, Color::r_to_n(&Vec3f::new(237.0, 252.0, 225.0)), 1.0)
        .set_reflection(0.2, 0.2)
        .add_to_scene(&mut scene).get();
    let mug_light_blue = PhongShader::new()
        .auto_id()
        .set_diffuse_color(Color::r_to_n(&Vec3f::new(133.0, 197.0, 237.0)))
        .set_specularity(0.6, Color::r_to_n(&Vec3f::new(214.0, 239.0, 255.0)), 1.0)
        .set_reflection(0.2, 0.2)
        .add_to_scene(&mut scene).get();

    scene.geometries = geometries;
    scene.apply_indexing();
    scene.assign_shader_to(&"floor", &floor.get_id());
    scene.assign_shader_to(&"kettle", &kettle.get_id());
    scene.assign_shader_to(&"table_top", &table_top.get_id());
    scene.assign_shader_to(&"cup_001", &cup_white.get_id());
    scene.assign_shader_to(&"cup_002", &cup_white.get_id());
    scene.assign_shader_to(&"cup_003", &cup_light_orange.get_id());
    scene.assign_shader_to(&"mug_001", &mug_light_green.get_id());
    scene.assign_shader_to(&"mug_002", &mug_light_blue.get_id());

    let mut area_light = AreaLight::new().set_id("area_light_1".to_string()).set_dimensions(30.0, 30.0)
        .set_intensity(30.0).set_shadow_samples(12).set_attenuation(Attenuation::Linear).get();
    area_light.transform.move_params(5.0, 10.0, 10.0);
    area_light.transform.rotate_by(-30.0, 0.0, 10.0);
    area_light.apply_transformation();
    area_light.flip();
    let mut point_light = PointLight::new("point_light_1", 2.5, Color::r_to_n(&FAINT_BLUE_WHITE), Attenuation::Flat);
    point_light.transform.translate = Vec3f::new(5.0, 12.5, 12.5);

    let ambient_light = AmbientLight::new("ambient_light_1", 0.0, Color::r_to_n(&WHITE));

    // scene.lights.push(LightEnum::PointLight(point_light));
    scene.lights.push(LightEnum::AmbientLight(ambient_light));
    if area_light.is_visible() {
        let (geom, shape_shader) = area_light.get_visibility_geometry();
        scene.geometries.push(geom);
        scene.shaders.push(shape_shader.get_shader());
    }
    scene.lights.push(LightEnum::AreaLight(area_light));

    let mut cam = StandardCamera::new(
        Vec2i::new(width, height),
        SENSOR_SQUARE_66,
        Some(Vec3f::new(0.0,0.0,0.0)),
        WORLD_UP,
        30.0,
        Some(Vec3f::new(-3.0, 1.5, 3.5)),
    );
    cam.transform.rotate_by(-5.0, -10.0, -4.0);

    cam.configure();

    scene.cameras.push(cam);

    scene.render_settings = RenderSettings::default();
    scene.render_settings.file_name = "teapot_obj{#}".to_string();
    scene.render_settings.width = width as usize;
    scene.render_settings.height = height as usize;
    scene.render_settings.multi_threading.enabled = true;
    if let Ok(num_of_threads) = available_parallelism() {
        scene.render_settings.multi_threading.count = usize::max(1, num_of_threads.get() - 1);
    }
    scene.render_settings.anti_aliasing.sample = 1;
    scene.render_settings.anti_aliasing.method = MonteCarlo;

    let mut env = Environment::new_with_image("../resources/images/pexels-curtis-adams-1694007-10164897.jpg".to_string());
    env.flip_v();
    env.set_repeat(4, 2);
    env.rotate_y(-40.0);
    scene.render_settings.environment = Some(env);
    scene.render_settings.disable_reflections = true;

    scene.apply_indexing();
    scene
}