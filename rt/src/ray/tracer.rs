use crate::buffer::buffer::Buffer;
use crate::camera::camera::StandardCamera;
use crate::common::constants::{EPS, REFLECTION_GLOSSINESS_SCATTER_FACTOR};
use crate::error::error::SysError;
use crate::error::kinds::ErrorKind;
use crate::object::geometry::{Geometry, GeometrySubType};
use crate::ray::types::{RayContext, RayType};
use crate::scene::scene::Scene;
use crate::shader::shader::BaseShader;
use crate::vector::arithmetic::VectorArithmetic;
use crate::vector::types::{Vec3i, Vector};
use crate::vector::vec3f::Vec3f;
use std::sync::{Arc, RwLock};
use rand::random;
use crate::colors::types::{Color, NColor3};
use crate::light::light::{BaseLight, LightEnum};
use crate::object::geometry::GeometryType::{Polygon, Procedural};
use crate::object::procedural::get_sphere_normal;
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
    pub fn walk_pixel_by_pixel(
        &self,
        buffer: &mut Buffer,
        cam: &StandardCamera,
        s: &Arc<RwLock<Scene>>,
    ) -> Buffer {
        let mut pixel = buffer.get_next_pixel_indices();
        let mut output_buffer = Buffer::new(buffer.x, buffer.y);
        while pixel.is_some() {
            let buffer_index = pixel.unwrap();

            let pixel_coordinate = cam.get_pixel_coordinates(buffer_index[1] as i64, buffer_index[2] as i64);
            let mut rc = RayContext::new_for_camera_ray(
                &cam.transform.local.translate,
                Some(buffer_index),
            );
            rc.camera_position = cam.transform.local.translate;
            rc.pixel_coordinate = Some(pixel_coordinate);
            rc.ray_dir = (&rc.pixel_coordinate.unwrap() - &rc.origin_coordinate).normalized();
            self.process_ray_for_objects(&mut rc, Some(&mut output_buffer), s);


            pixel = buffer.get_next_pixel_indices();
        }
        output_buffer
    }


    // it iterates over list of objects and checks
    // if a given ray intersects that object or not (
    // through another iteration of tests done on that
    // object's faces, iff the object is a polygon.
    pub fn process_ray_for_objects(
        &self,
        mut rc: &mut RayContext,
        output: Option<&mut Buffer>,
        s: &Arc<RwLock<Scene>>,
    ) -> Option<NColor3> {
        let geometries: Vec<Geometry>;
        {
            let scene = s.read().unwrap();
            geometries = scene.geometries.clone();
        }

        for ref obj in geometries.iter().enumerate() {
            rc.reset_for_next_iteration(obj.0, obj.1.render_attributes.shadows.receive);


            if let Ok(()) = self.trace_single_ray(&obj.1, &mut rc) {
                // yet nothing
            }
        }

        if rc.has_ever_intersected() {
            if let Some(normalized_color) =
                self.on_ray_target_intersection(&mut rc, s)
            {
                if let Some(output) = output {
                    output.save_pixel_color(
                        rc.buffer_index.clone().unwrap()[0],
                        Color::n_to_r(&normalized_color.to_4()),
                    );

                }
                return Some(normalized_color);
            }
        }
        None
    }

    pub fn trace_from_camera_to_scene(
        &self,
        input_buffer: &mut Buffer,
        cam: &StandardCamera,
        s: Arc<RwLock<Scene>>,
    ) -> Result<Buffer, SysError> {

        let output = self.walk_pixel_by_pixel(
            input_buffer,
            &cam,
            &s
        );

        Ok(output)
    }



    pub fn callback_object_iteration(&self, obj: &Geometry, mut ray_intersection: &mut RayContext) {
        if let Ok(()) = self.trace_single_ray(&obj, &mut ray_intersection) {
            // yet nothing
        }
    }

    /// this method is invoked when a ray intersects
    /// an object. This ray can be an ordinary camera
    /// ray, a shadow, a reflection or a refraction (not yet)
    /// ray. Hence, the origin of the call can be from
    /// a camera ray, a shadow ray, a reflection or
    /// refraction ray. Though all rays except camera
    /// ray are absolutely spawned by another ray. The root
    /// of all rays absolutely goes back to a camera ray.
    pub fn on_ray_target_intersection(
        &self,
        mut rc: &mut RayContext,
        s: &Arc<RwLock<Scene>>,
    ) -> Option<NColor3> {
        let scene = s.read().unwrap();
        if rc.intersected_object_index.is_none() {
            panic!("object id is null")
        }

        let material_name = scene.geometries[rc.intersected_object_index.unwrap()]
            .render_attributes
            .material_name
            .clone();

        let mut shader = scene.lookup_shader(&material_name);

        match shader {
            Some(ref mut ss) => {
                let mut main_point_color = NColor3::default();
                for light in &scene.lights {
                    if rc.can_continue_for_reflection() && ss.1.cast_reflection() {
                        let mut rc_reflection = rc.fork_for_reflection(RayType::ReflectionRay, &rc.intersection_coordinate,
                                                                       rc.intersected_face_normal, rc.intersected_face_vertex_normal);

                        ss.1.set_reflection_properties(&mut rc_reflection);
                        rc.increment_reflection_level();
                        match self.trace_rays_for_reflection(&mut rc_reflection, s) {
                            Err(err) => {
                                panic!("an error occurred when calculating the reflection: {:?}", err)
                            },
                            Ok(res) => {
                                if let Some(ref_color) = res {
                                    main_point_color += ss.1.get_reflection_final_color(&ref_color);
                                }
                            }
                        }
                    }
                    if rc.obj_receive_shadow && light.can_cast_shadow() {
                        let acneless_shadow_origin = rc.intersection_coordinate.add_with(&rc.intersected_face_normal.unwrap().multiply_scalar(0.001));
                        let mut rc_shadow = RayContext::new_for_secondary_ray(RayType::ShadowRay, &acneless_shadow_origin, rc.intersected_face_normal, rc.intersected_face_vertex_normal);

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
                                    main_point_color += Color::r_to_n(&BLACK)
                                }
                            }
                        }
                    } else {
                        match ss.1.compute(&rc, &light) {
                            Ok(color) => {
                                main_point_color += color
                            },
                            Err(err) => {
                                panic!("an error occurred when calculating the shader's shadow: {:?}", err)
                            }
                        }
                    }
                }
                Some(main_point_color)
            }
            None => return None,
        }
    }


    pub fn trace_rays_for_shadows(&self, light: &LightEnum, mut rc: &mut RayContext, sc: &Arc<RwLock<Scene>>) -> Result<(), SysError> {
        if light.can_cast_shadow() == false {
            rc.is_in_shadow = false;
            return Ok(());
        }
        let light_to_point_vector = light.get_displacement_vector(&rc.intersection_coordinate);
        rc.ray_dir = light_to_point_vector.normalized();
        let distance_to_light = light_to_point_vector.magnitude();
        let scene = sc.read().unwrap();
        rc.intersected = false;
        rc.is_in_shadow = false;
        rc.previous_closest_distance = distance_to_light;
        for obj in scene.geometries.iter().enumerate() {
            match self.trace_single_ray(&obj.1, &mut rc) {
                Ok(()) => {
                    if rc.intersected == true {
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



    pub fn trace_rays_for_reflection(&self, mut rc: &mut RayContext, sc: &Arc<RwLock<Scene>>) -> Result<Option<NColor3>, SysError> {
        rc.ever_intersected = false;
        rc.intersected = false;
        rc.previous_closest_distance = f64::INFINITY;
        let direct_ray_dir = rc.ray_dir;
        let num_of_emitted_rays = rc.reflection_glossiness_samples as usize;
        if num_of_emitted_rays > 0 {
            let mut final_color = NColor3::default();
            for _ in 0..num_of_emitted_rays {
                let mut random_ray = self.unit_random();
                if random_ray.dot(&direct_ray_dir) < 0.0 {
                    random_ray = random_ray.multiply_scalar(-1.0);
                }
                let rnd_ray_dir = (&(direct_ray_dir + random_ray * rc.reflection_glossiness * REFLECTION_GLOSSINESS_SCATTER_FACTOR)).normalized();
                rc.ray_dir = rnd_ray_dir;
                if let Some(color) = self.process_ray_for_objects(rc, None, sc) {
                    final_color += color;
                }
            }
            return Ok(Some(final_color.divide_by_scalar(num_of_emitted_rays as f64)));
        }
        if let Some(color) = self.process_ray_for_objects(rc, None, sc) {
            return Ok(Some(color));
        }
        Ok(None)
    }


    pub fn trace_single_ray(&self, obj: &Geometry, rc: &mut RayContext) -> Result<(), SysError> {
        if obj.render_attributes.renderable == false {
            return Ok(());
        }
        rc.intersected_geo_type = Some(obj.geometry_type.clone());
        rc.intersected_geo_subtype = Some(obj.geometry_subtype.clone());
        rc.intersected_object_centroid = Some(obj.get_center());
        if obj.geometry_type == Polygon {
            _ = self.try_over_faces(
                &obj.data.faces,
                &obj.data.vertices,
                &obj.data.face_normals,
                &obj.data.vertices,
                rc,
            )
        } else if obj.geometry_type == Procedural {
            _ = self.try_over_procedural(obj, rc);
        } else {
            return Err(SysError::new_str(ErrorKind::GeometryTypeUndefined, "cannot understand the geometry type"))
        }
        Ok(())
    }


    pub fn try_over_faces(
        &self,
        faces: &Vec<Vec3i>,
        vertices: &Vec<Vec3f>,
        face_normals: &Vec<Vec3f>,
        vertices_normals: &Vec<Vec3f>,
        rc: &mut RayContext,
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
                if  rc.is_closest_so_far(c.0) {
                    rc.update_intersection(rc.next_object_index, Some(kv.0), Some(face_normals[kv.0]), c.0, c.1);
                }
            }
        }
        Ok(())
    }

    pub fn try_over_procedural(&self, geo: &Geometry, rc: &mut RayContext) -> Result<(), SysError> {
        match geo.geometry_subtype.clone() {
            GeometrySubType::Sphere => {
                let r = geo.data.params.get("radius");
                match r {
                    Some(rr) => {

                        if let Some(r_intr) = self.use_sphere_detection(&rc.origin_coordinate, &rc.ray_dir, &geo.transform.local.translate, rr.v_f64) {
                            let normal = get_sphere_normal(&r_intr.1, &geo.transform.local.translate);
                            _ = rc.update_intersection(rc.next_object_index, None, Some(normal), r_intr.0, r_intr.1);
                        }

                        Ok(())
                    },
                    None => {
                        Err(SysError::new_str(ErrorKind::GeometryTypeUndefined, "proc. sphere doesn't have any radius"))
                    }
                }
            }
            _ => {
                Err(SysError::new_str(ErrorKind::GeometryTypeUndefined, "cannot understand the procedural geometry's sub type"))
            }
        }

    }


    pub fn shoot_ray_to_planar_triangle(
        &self,
        origin: &Vec3f,
        target: &Vec3f,
        face: &Vec<&Vec3f>,
    ) -> Option<(f64, Vec3f)> {
        if let Some(intersection) = self.use_moller_trumbore(origin, &target, face) {
            return Some(intersection);
        }
        None
    }


    /// ray dir MUST be normalized before being passed to the function
    /// returns (distance, intersection coordinate)
    pub fn use_sphere_detection(&self, origin: &Vec3f, ray_dir: &Vec3f, sphere_center: &Vec3f, sphere_radius: f64) -> Option<(f64, Vec3f)> {
        let ray_to_sphere = origin - sphere_center;
        let a = 1.0_f64; // since squaring a normalized vector results in one (hence self.dot(self) = 1)
        let b = &(2.0)*ray_to_sphere.dot(ray_dir);
        let c = ray_to_sphere.dot(&ray_to_sphere) - sphere_radius*sphere_radius;

        // this is essentially solving quadratic formula
        let discrm = (b * b) - 4.0*c;
        // in math formula, denominator is 2a, but since
        // a is 1.0 (due to it being normalized and raised to
        // the power of 2), we drop it
        let distance = (-b - discrm.sqrt()) / 2.0;

        if distance > 0.0 {
            return Some((distance, origin+&ray_dir.multiply_scalar(distance)));
        }
        None
    }

    // uses Möller–Trumbore to calculate if a ray + direction vector of the pixel
    // hits a given triangle
    // In case the ray hits something, it returns the intersection point's coordinates
    // as well as the normalized distance scalar (known as t)
    pub fn use_moller_trumbore(
        &self,
        origin: &Vec3f,
        ray_dir: &Vec3f,
        triangle: &Vec<&Vec3f>,
    ) -> Option<(f64, Vec3f)> {
        let e1 = triangle[1] - triangle[0];
        let e2 = triangle[2] - triangle[0];
        let s = origin - triangle[0];

        let p = VectorArithmetic::cross3(ray_dir, &e2);
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
        let v = f * (VectorArithmetic::dot(ray_dir, &q));
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = f * VectorArithmetic::dot(&e2, &q);

        if t > 0.0 {
            return Some((t, origin + &ray_dir.multiply_scalar(t)));
        }

        None
    }


    fn gen_rand(&self) -> f64 {
        random::<f64>()
    }

    // returns a random number from -1.0 to 1.0
    fn get_random_in_range(&self) -> f64 {
        return self.gen_rand() * 2.0 - 1.0;
    }

    fn unit_random(&self) -> Vec3f {
        loop {
            let mut v = Vec3f::default();
            for k in 0..3 {
                v[k] = self.get_random_in_range();
            }

            if v.length_squared() <= 1.0 {
                return v
            }
        }
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
    use std::sync::{Arc, RwLock};

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
        let result = tracer.use_moller_trumbore(&cam_pos, &ray_dir, &triangle);
        assert_ne!(None, result);

        let mut triangle: Vec<&Vec3f> = Vec::new();
        let vx1 = &Vec3f::new(-3000.0, 20.0, 45.0);
        let vx2 = &Vec3f::new(-2000.0, 20.0, 45.0);
        let vx3 = &Vec3f::new(-3000.0, 30.0, 45.0);
        triangle.push(vx1);
        triangle.push(vx2);
        triangle.push(vx3);
        let result = tracer.use_moller_trumbore(&cam_pos, &ray_dir, &triangle);
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
