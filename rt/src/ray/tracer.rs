use crate::buffer::buffer::Buffer;
use crate::buffer::types::BufferIndex;
use crate::camera::camera::StandardCamera;
use crate::common::constants::EPS;
use crate::error::error::SysError;
use crate::error::kinds::ErrorKind;
use crate::object::geometry::Geometry;
use crate::ray::types::RayCollision;
use crate::scene::scene::Scene;
use crate::shader::shader::BaseShader;
use crate::vector::arithmetic::VectorArithmetic;
use crate::vector::colors::{NormalizedColor, Rgba};
use crate::vector::types::{Vec3i, Vector};
use crate::vector::vec3f::Vec3f;
use std::sync::{Arc, Mutex, RwLock};
use crate::light::light::{BaseLight, LightEnum};
use crate::ray::types::CollisionTestType::{CameraRay, ShadowRay};
use crate::vector::constants::BLACK;

pub struct Tracer {
    max_depth: i8,
    max_traversal_distance: i64,
    max_reflection_traversal: i8,
    max_sample_ray: i8,
    parallelism: i8,
}

impl Default for Tracer {
    fn default() -> Self {
        Tracer {
            max_depth: 0,
            max_traversal_distance: 100,
            max_reflection_traversal: 1,
            max_sample_ray: 1,
            parallelism: 1,
        }
    }
}
impl Tracer {
    pub fn walk_pixel_by_pixel<F: Fn(BufferIndex, &Vec3f, &mut Buffer, &Arc<RwLock<Scene>>)>(
        &self,
        buffer: &mut Buffer,
        cam: &StandardCamera,
        s: &Arc<RwLock<Scene>>,
        f: F,
    ) -> Buffer {
        let mut pixel = buffer.get_next_pixel_indices();
        let mut output_buffer = Buffer::new(buffer.x, buffer.y);
        while pixel.is_some() {
            let buffer_index = pixel.unwrap();

            let pixel_coordinate =
                cam.get_pixel_coordinates(buffer_index[1] as i64, buffer_index[2] as i64);

            f(buffer_index, &pixel_coordinate, &mut output_buffer, s);
            pixel = buffer.get_next_pixel_indices();
        }
        output_buffer
    }

    pub fn walk_scene_objects<
        Shoot: Fn(&Geometry, &mut RayCollision),
        OnClosestCollision: Fn(&mut RayCollision, &mut Buffer, &Arc<RwLock<Scene>>),
    >(
        &self,
        mut rc: &mut RayCollision,
        output: &mut Buffer,
        s: &Arc<RwLock<Scene>>,
        object_ray_collision_test: Shoot,
        on_closest_collision: OnClosestCollision,
    ) {
        let geometries: Vec<Geometry>;
        {
            let scene = s.read().unwrap();
            geometries = scene.geometries.clone();
        }

        for ref obj in geometries.iter().enumerate() {
            rc.reset_for_next_iteration(obj.0, obj.1.render_attributes.shadows.receive);
            object_ray_collision_test(obj.1, &mut rc)
        }

        if rc.has_ever_collided() {
            on_closest_collision(rc, output, s);
        }
    }

    pub fn walk_scene_lights<
        Shoot: Fn(&Geometry, &mut RayCollision),
        OnNoCollision: Fn(&RayCollision, &Arc<RwLock<Scene>>),
    >(
        &self,
        light: &LightEnum,
        mut rc: &mut RayCollision,
        s: &Arc<RwLock<Scene>>,
        object_ray_collision_test: Shoot,
        on_no_collision: OnNoCollision,
    ) {
        let scene = s.read().unwrap();

        let point_to_light_vector = light.get_displacement_vector(&rc.collision_coordinate);
        rc.ray_dir = point_to_light_vector.normalized();

        for ref obj in scene.geometries.iter().enumerate() {
            rc.reset_for_next_iteration(obj.0, obj.1.render_attributes.shadows.receive);
            object_ray_collision_test(obj.1, &mut rc)
        }

        if rc.has_ever_collided() == false {
            on_no_collision(rc, s);
        }
    }

    pub fn callback_object_iteration(&self, obj: &Geometry, mut ray_collision: &mut RayCollision) {
        if let Ok(()) = self.trace_single_ray(&obj, &mut ray_collision) {
            // yet nothing
        }
    }

    pub fn callback_on_ray_object_collision(
        &self,
        mut rc: &mut RayCollision,
        s: &Arc<RwLock<Scene>>,
    ) -> Option<Rgba> {
        let scene = s.read().unwrap();
        if rc.collided_object_index.is_none() {
            panic!("object id is null")
        }

        let material_name = scene.geometries[rc.collided_object_index.unwrap()]
            .render_attributes
            .material_name
            .clone();

        let mut shader = scene.lookup_shader(&material_name);

        match shader {
            Some(ref mut ss) => {
                let mut main_point_color = NormalizedColor::default();
                for light in &scene.lights {
                    if rc.obj_receive_shadow {
                        let acneless_shadow_origin = rc.collision_coordinate.add_with(&rc.collided_face_normal.unwrap().multiply_scalar(0.001));
                        let mut rc_shadow = RayCollision::new_for_shadow_ray(&acneless_shadow_origin);

                        match self.trace_rays_for_shadows(&light, &mut rc_shadow, s) {
                            Err(err) => {
                                panic!("an error occurred when calculating the shadow: {:?}", err)
                            },
                            Ok(()) => {
                                if rc_shadow.is_in_shadow == false {
                                    match ss.1.compute(&rc, &light) {
                                        Ok(color) => {
                                            main_point_color += color
                                        },
                                        Err(err) => {
                                            panic!("an error occurred when calculating the shader's shadow: {:?}", err)
                                        }
                                    }
                                } else {
                                    main_point_color += BLACK.to_normalized_color()
                                }
                            }
                        }
                    }
                }
                Some(main_point_color)
            }
            None => return None,
        }
    }


    pub fn trace_rays_for_shadows(&self, light: &LightEnum, mut rc: &mut RayCollision, sc: &Arc<RwLock<Scene>>) -> Result<(), SysError> {
        if light.supports_shadow() == false {
            rc.is_in_shadow = false;
            return Ok(());
        }
        let light_to_point_vector = light.get_displacement_vector(&rc.collision_coordinate);
        rc.ray_dir = light_to_point_vector.normalized();
        let distance_to_light = light_to_point_vector.magnitude();
        let scene = sc.read().unwrap();
        rc.collided = false;
        rc.is_in_shadow = false;
        rc.previous_closest_distance = distance_to_light;
        for obj in scene.geometries.iter().enumerate() {
            match self.trace_single_ray(&obj.1, &mut rc) {
                Ok(()) => {
                    if rc.collided == true {
                        rc.is_in_shadow = true;
                        return Ok(());
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(())
    }

    // starts from the scene and loads the camera, lights and objects
    // Iterates through every single pixel of the camera and shoots ray,
    // through each individual object to further render them (or not)
    pub fn trace_from_camera_to_scene(
        &self,
        input_buffer: &mut Buffer,
        cam: &StandardCamera,
        s: Arc<RwLock<Scene>>,
    ) -> Result<Buffer, SysError> {

        let output = self.walk_pixel_by_pixel(
            input_buffer,
            &cam,
            &s,
            |pixel_index: BufferIndex, pixel_coordinate: &Vec3f,
             mut output_buffer: &mut Buffer,
             s: &Arc<RwLock<Scene>>| {
                let mut rc = RayCollision::new_for_camera_ray(
                    &cam.transform.local.translate,
                    Some(pixel_index),
                );
                rc.pixel_coordinate = Some(*pixel_coordinate);
                rc.ray_dir = (&rc.pixel_coordinate.unwrap() - &rc.origin_coordinate).normalized();
                self.walk_scene_objects(
                    &mut rc,
                    &mut output_buffer,
                    &s,
                    |obj: &Geometry, mut ray_collision: &mut RayCollision| {
                        self.callback_object_iteration(obj, &mut ray_collision);
                    },
                    |mut ray_collision: &mut RayCollision, output: &mut Buffer, sc: &Arc<RwLock<Scene>>| {
                        if let Some(normalized_color) =
                            self.callback_on_ray_object_collision(&mut ray_collision, sc)
                        {
                            output.save_pixel_color(
                                ray_collision.buffer_index.clone().unwrap()[0],
                                normalized_color.to_rgba(),
                            );
                        }
                    },
                );
            },
        );

        Ok(output)
    }

    pub fn trace_single_ray(&self, obj: &Geometry, rc: &mut RayCollision) -> Result<(), SysError> {
        self.try_ray_intersect(
            &obj.data.faces,
            &obj.data.vertices,
            &obj.data.face_normals,
            &obj.data.vertices,
            rc,
        )
    }

    pub fn try_ray_intersect(
        &self,
        faces: &Vec<Vec3i>,
        vertices: &Vec<Vec3f>,
        face_normals: &Vec<Vec3f>,
        vertices_normals: &Vec<Vec3f>,
        rc: &mut RayCollision,
    ) -> Result<(), SysError> {
        for kv in faces.iter().enumerate() {
            let mut face_coordinates: Vec<&Vec3f> = Vec::new();
            for index in 0..3 {
                if let Some(vx) = vertices.get(kv.1[index] as usize) {
                    face_coordinates.push(vx)
                } else {
                    return Err(SysError::new(
                        ErrorKind::BadFaceStructure,
                        "[face's] vertex is not found".to_string(),
                    ));
                }
            }
            if let Some(c) =
                self.shoot_ray_to_planar_triangle(&rc.origin_coordinate, &rc.ray_dir, &face_coordinates)
            {
                if rc.test_type == CameraRay  && ( c.0 > EPS && c.0 < rc.previous_closest_distance) {
                    rc.previous_closest_distance = c.0;
                    rc.collision_distance = c.0;
                    rc.collision_coordinate = c.1;
                    rc.collided_face_index = Some(kv.0);
                    rc.collided_object_index = rc.next_object_index;
                    rc.collided_face_normal = Some(face_normals[kv.0]);
                    rc.collided_face_vertex_normal = Some(vertices_normals[kv.1[0] as usize]);
                    rc.collided = true;
                    if !rc.ever_collided {
                        rc.ever_collided = true;
                    }
                } else if rc.test_type == ShadowRay && (c.0 < rc.previous_closest_distance) {
                    rc.collision_distance = c.0;
                    rc.collision_coordinate = c.1;
                    rc.collided_face_index = Some(kv.0);
                    rc.collided_object_index = rc.next_object_index;
                    rc.collided_face_normal = Some(face_normals[kv.0]);
                    rc.collided_face_vertex_normal = Some(vertices_normals[kv.1[0] as usize]);
                    rc.collided = true;
                    return Ok(())

                    // @todo for area light, we need to continue sampling also for soft shadows
                }
            }
        }
        Ok(())
    }

    pub fn shoot_ray_to_planar_triangle(
        &self,
        origin: &Vec3f,
        target: &Vec3f,
        face: &Vec<&Vec3f>,
    ) -> Option<(f64, Vec3f)> {
        if let Some(intersection) = self.solve_equation(origin, &target, face) {
            return Some(intersection);
        }
        None
    }

    // uses Möller–Trumbore to calculate if a ray + direction vector of the pixel
    // hits a given triangle
    // In case the ray hits something, it returns the intersection point's coordinates
    // as well as the normalized distance scalar (known as t)
    pub fn solve_equation(
        &self,
        origin: &Vec3f,
        d: &Vec3f,
        triangle: &Vec<&Vec3f>,
    ) -> Option<(f64, Vec3f)> {
        let e1 = triangle[1] - triangle[0];
        let e2 = triangle[2] - triangle[0];
        let s = origin - triangle[0];

        let p = VectorArithmetic::cross3(d, &e2);
        let a = VectorArithmetic::dot(&e1, &p);
        if a.abs() < EPS {
            return None;
        }
        // compute barycentric coordinates
        let f = 1f64 / a;
        let u = f * (VectorArithmetic::dot(&s, &p));
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let q = VectorArithmetic::cross3(&s, &e1);
        let v = f * (VectorArithmetic::dot(d, &q));
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = f * VectorArithmetic::dot(&e2, &q);

        if t > 0.0 {
            return Some((t, origin + &d.multiply_scalar(t)));
        }

        None
    }
}

#[cfg(test)]

mod test {
    use crate::camera::camera::StandardCamera;
    use crate::ray::tracer::Tracer;
    use crate::render::renderer::Renderer;
    use crate::scene::scene::Scene;
    use crate::vector::types::{Vec2i, Vector};
    use crate::vector::vec3f::Vec3f;
    use std::sync::{Arc, Mutex, RwLock};

    #[test]
    fn test_single_ray_plane_intersection() {
        let focal_length = 50.0;
        let image_plane_width = 100.0;
        let image_plane_height = 100.0;
        let ndc = StandardCamera::get_ndc(&Vec2i::new(2, 2), 0, 0);
        assert_eq!(0.25, ndc[0]);
        assert_eq!(0.25, ndc[1]);
        let screen_space = StandardCamera::get_screen_space(ndc[0], ndc[1]);
        assert_eq!(-0.5, screen_space[0]);
        assert_eq!(0.5, screen_space[1]);
        let x = screen_space[0] * (image_plane_width / 2.0);
        let y = screen_space[1] * (image_plane_height / 2.0);
        assert_eq!(-25.0, x);
        assert_eq!(25.0, y);
        let cam_pos = Vec3f::new(0.0, 0.0, -5.0);
        let right = Vec3f::new(1.0, 0.0, 0.0);
        let up = Vec3f::new(0.0, 1.0, 0.0);
        let forward = Vec3f::new(0.0, 0.0, 1.0);
        let mut ray_dir = forward
            .multiply_scalar(focal_length)
            .add_with(&right.multiply_scalar(x))
            .add_with(&up.multiply_scalar(y));
        assert_eq!(Vec3f::new(-25.0, 25.0, focal_length), ray_dir);
        ray_dir = ray_dir.normalized();
        let tracer = Tracer::default();
        let mut triangle: Vec<&Vec3f> = Vec::new();
        let vx1 = &Vec3f::new(-30.0, 20.0, 45.0);
        let vx2 = &Vec3f::new(-20.0, 20.0, 45.0);
        let vx3 = &Vec3f::new(-30.0, 30.0, 45.0);
        triangle.push(vx1);
        triangle.push(vx2);
        triangle.push(vx3);
        let result = tracer.solve_equation(&cam_pos, &ray_dir, &triangle);
        assert_ne!(None, result);

        let mut triangle: Vec<&Vec3f> = Vec::new();
        let vx1 = &Vec3f::new(-3000.0, 20.0, 45.0);
        let vx2 = &Vec3f::new(-2000.0, 20.0, 45.0);
        let vx3 = &Vec3f::new(-3000.0, 30.0, 45.0);
        triangle.push(vx1);
        triangle.push(vx2);
        triangle.push(vx3);
        let result = tracer.solve_equation(&cam_pos, &ray_dir, &triangle);
        assert_eq!(None, result);
    }

    #[test]
    fn test_ray_emitter() {
        let s = Scene::load_from_file("../resources/scene_examples/scene_basic.json");
        assert_eq!(false, s.is_err(), "err={:?}", s.err().unwrap());

        if s.is_ok() {
            let s = s.unwrap();
            let mut renderer = Renderer::new(Arc::new(RwLock::new(s)));
            renderer.render();
        }
    }
}
