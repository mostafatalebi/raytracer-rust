use crate::buffer::buffer::Buffer;
use crate::camera::camera::StandardCamera;
use crate::common::constants::{EPS, REFLECTION_GLOSSINESS_SCATTER_FACTOR};
use crate::error::error::SysError;
use crate::error::kinds::ErrorKind;
use crate::geometry::geometry::{Geometry, GeometrySubType};
use crate::ray::ray_context::{RayContext, RayType};
use crate::scene::scene::Scene;
use crate::shader::shader::BaseShader;
use crate::vector::arithmetic::VectorArithmetic;
use crate::vector::types::Vector;
use crate::vector::vec3f::Vec3f;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::atomic::Ordering::SeqCst;
use rand::random;
use crate::colors::types::{Color, NColor3};
use crate::light::light::{BaseLight, LightEnum};
use crate::geometry::geometry::GeometryType::{Polygon, Procedural};
use crate::geometry::procedural::get_sphere_normal;
use crate::vector::constants::BLACK;
use rayon::prelude::*;
use crate::bounding_box::aabb::AABB;
use crate::bounding_box::bvh::BvhNode;
use crate::buffer::types::BufferIndex;
use crate::camera::types::AntiAliasingMethod;
use crate::common::enums::TraversalReturn::{Continue, DontContinue};
use crate::common::types::NormalizedF;
use crate::common::volume::Centroid;
use crate::geometry::geometry::GeometrySubType::AreaLightShape;
use crate::ray::ray_context::RayType::ShadowRay;
use crate::render::pass::RenderPass;
use crate::render::types::RenderRegion;
use crate::scene::environment::Environment;
use crate::scene::render_settings::AntiAliasingSetting;
use crate::vector::utils::Utils;
use crate::vector::vec3i::Vec3i;

#[derive(Clone)]
pub struct Tracer<'a> {
    max_depth: i8,
    max_traversal_distance: i64,
    max_reflection_traversal: i8,
    max_sample_ray: i8,
    num_of_threads: usize,
    anti_aliasing: AntiAliasingSetting,

    geometries: Option<&'a Vec<Geometry>>,
    pub bhv_tree: Option<&'a BvhNode>,
    pub total_rays_to_process: Arc<AtomicU64>,
    pub rays_processed_sofar: Arc<AtomicU64>,

    env_bg: Option<Environment>,
}

impl<'a> Default for Tracer<'a> {
    fn default() -> Self {
        Tracer {
            max_depth: 0,
            max_traversal_distance: 100,
            max_reflection_traversal: 1,
            max_sample_ray: 1,
            num_of_threads: 1,
            anti_aliasing: AntiAliasingSetting::default(),
            geometries: None,
            bhv_tree: None,
            total_rays_to_process: Arc::new(AtomicU64::new(0)),
            rays_processed_sofar: Arc::new(AtomicU64::new(0)),
            env_bg: None,
        }
    }
}
impl<'a> Tracer<'a> {

    pub fn set_num_of_threads(&mut self, num_of_threads: usize) {
        if num_of_threads < 1 {
            panic!("num_of_threads must be greater than 0");
        }
        self.num_of_threads = num_of_threads;
    }

    pub fn set_environment(&mut self, env: Environment) {
        self.env_bg = Some(env.clone())
    }

    pub fn set_anti_aliasing(&mut self, aa: &AntiAliasingSetting) {
        if aa.sample < 1 {
            panic!("anti_aliasing must be greater than 0");
        }
        self.anti_aliasing = aa.clone()
    }


    pub fn set_geometries(&mut self, geometries: &'a Vec<Geometry>) {
        self.geometries = Some(geometries);
    }
    pub fn set_bvh_tree(&mut self, bvh_node: &'a BvhNode) {
        self.bhv_tree = Some(bvh_node);
    }



    pub fn create_workload(&mut self, buffer: &mut Buffer) -> Vec<BufferIndex> {
        let mut workload = vec![BufferIndex::default(); buffer.get_size()];
        let mut pixel = buffer.get_next_pixel_indices();
        let mut i = 0usize;

        while pixel.is_some() {
            workload[i] = pixel.clone().unwrap();

            pixel = buffer.get_next_pixel_indices();
            i += 1
        }
        self.total_rays_to_process.store(buffer.get_size() as u64 * self.anti_aliasing.sample as u64, SeqCst);
        workload
    }


    pub fn walk_pixel_by_pixel(
        &mut self,
        buffer: &mut Buffer,
        cam: &StandardCamera,
        s: &Arc<RwLock<Scene>>,
        render_region: Option<RenderRegion>,
    ) -> Buffer {
        let mut workload = self.create_workload(buffer);
        let output_buffer = Arc::new(Mutex::new(Buffer::new(buffer.x, buffer.y)));

        let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(self.num_of_threads)
            .start_handler(|thread_index| {
                //println!("thread #{} started", thread_index);
            })
        .build();

        let buffer_cloned = output_buffer.clone();
        thread_pool.unwrap().install(|| {
            workload.par_iter_mut().for_each(|tile| {
                if let Some(ref rr) = render_region {
                    if rr.is_in_region(tile[1], tile[2]) == false {
                        return;
                    }
                }
                let pixel_coordinate = cam.get_anti_aliased_pixel_coordinates(tile[1] as i64, tile[2] as i64, self.anti_aliasing.sample, &self.anti_aliasing.method);
                let mut rc = RayContext::new_for_camera_ray(
                    &cam.transform.translate,
                    Some(tile.clone()),
                );
                rc.camera_position = cam.transform.translate;
                let mut pixel_color = NColor3::default();
                let mut hit_count: usize = 0;
                for pixel_c in pixel_coordinate {
                    let ray_dir = (&pixel_c - &rc.origin_coordinate).normalized();
                    rc.reset_for_next_iteration(ray_dir, Some(pixel_c));
                    if let Some(color_pass) = self.process_ray_for_objects(&mut rc, s) {
                        pixel_color += color_pass.composite();
                        hit_count += 1;
                    }
                    self.rays_processed_sofar.fetch_add(1, Ordering::SeqCst);

                }

                if hit_count > 0 {
                    let mut output = buffer_cloned.lock().unwrap();
                    let normalized_color_avg: NColor3 = Color::avg(&pixel_color, hit_count as u64);
                    output.save_pixel_color(
                        tile[0],
                        Color::n_to_r(&normalized_color_avg.to_4()),
                    );
                }
            });
        });

        output_buffer.clone().lock().unwrap().clone()
    }


    // it iterates over list of objects and checks
    // if a given ray intersects that geometry or not (
    // through another iteration of tests done on that
    // geometry's faces, iff the geometry is a polygon.
    pub fn process_ray_for_objects(
        &self,
        mut rc: &mut RayContext,
        s: &Arc<RwLock<Scene>>,
    ) -> Option<RenderPass> {
        rc.calc_inv_ray_dir();
        self.bhv_tree.unwrap().inorder_traversal(rc, &mut |_rc, is_leaf, bounding_volume, objects_list| {
            if self.test_against_bb(bounding_volume, _rc) == false {
                // return early
                return Continue;
            }

            if is_leaf {
                for object_index in objects_list {
                    let geom = self.geometries.unwrap()[*object_index].clone();
                    if geom.geometry_subtype == AreaLightShape && _rc.is_camera_ray() {
                        // continue;
                    }
                    _rc.refresh_for_new_object_test(*object_index, geom.render_attributes.shadows.receive);

                    if let Ok(()) = self.trace_single_ray(&geom, _rc) {
                        // yet nothing
                    }
                }
            }

            Continue
        });


        if rc.has_ever_intersected() {
            return self.on_ray_target_intersection(&mut rc, s);
        } else {
            return Some(self.get_bg_color(&rc.ray_dir.normalized()));
        }
        None
    }

    pub fn trace_from_camera_to_scene(
        &mut self,
        input_buffer: &mut Buffer,
        cam: &StandardCamera,
        s: Arc<RwLock<Scene>>,
        rr: Option<RenderRegion>
    ) -> Result<Buffer, SysError> {

        let output = self.walk_pixel_by_pixel(
            input_buffer,
            &cam,
            &s,
            rr
        );

        Ok(output)
    }


    /// this method is invoked when a ray intersects
    /// an geometry. The ray can be an ordinary camera
    /// ray (main ray), a shadow, a reflection or a refraction (not yet)
    /// ray. Hence, the origin of the call can be from
    /// a camera ray, a shadow ray, a reflection or
    /// refraction ray. Though all rays except camera
    /// ray are absolutely spawned by another ray. The root
    /// of all rays absolutely goes back to a camera ray.
    pub fn on_ray_target_intersection(
        &self,
        mut rc: &mut RayContext,
        s: &Arc<RwLock<Scene>>,
    ) -> Option<RenderPass> {
        if rc.intersected_object_index.is_none() {
            panic!("geometry id is null")
        }
        let scene = s.read().unwrap();

        let material_name = self.geometries.unwrap()[rc.intersected_object_index.unwrap()]
            .render_attributes
            .material_name
            .clone();
        let mut shader = scene.lookup_shader(&material_name);

        match shader {
            Some(ref mut ss) => {
                let mut color_pass = RenderPass::default();
                let shader_ptr = ss.1;
                let mut rc_reflection = rc.fork_for_reflection(&rc.intersection_coordinate,
                                                               rc.intersected_face_normal, rc.intersected_vertex_normal);

                if !scene.render_settings.disable_reflections && rc_reflection.can_continue_for_reflection() && shader_ptr.cast_reflection() {
                    shader_ptr.set_reflection_properties(&mut rc_reflection);
                    rc_reflection.increment_reflection_level();
                    match self.trace_rays_for_reflection(&mut rc_reflection, s) {
                        Err(err) => {
                            panic!("an error occurred when calculating the reflection: {:?}", err)
                        },
                        Ok(res) => {
                            if let Some(ref_color) = res {
                                color_pass.set_reflection(shader_ptr.get_reflection_final_color(&ref_color));
                            }
                        }
                    }
                }

                for light in &scene.lights {
                    let mut shadow_multiplier = 1.0;
                    // shadow pass
                    if !scene.render_settings.disable_shadows && light.can_cast_shadow() {
                        let normal = rc.get_proper_normal();
                        let acneless_shadow_origin = rc.intersection_coordinate.add_with(&normal.multiply_scalar(0.001));
                        let mut rc_shadow = RayContext::new_for_secondary_ray(RayType::ShadowRay,
                                                                              &acneless_shadow_origin, rc.intersected_face_normal,
                                                                              rc.intersected_vertex_normal);

                        match self.trace_rays_for_shadows(&light, &mut rc_shadow, s) {
                            Err(err) => {
                                panic!("an error occurred when calculating the shadow: {:?}", err)
                            },
                            Ok(multiplier) => {
                                shadow_multiplier = multiplier;
                                color_pass.add_shadow(multiplier);
                            }
                        }
                    }


                    // diffuse pass
                    match shader_ptr.compute(&rc, &light) {
                        Ok(color) => {
                            color_pass.add_diffuse(&color, shadow_multiplier);
                        },
                        Err(err) => {
                            panic!("an error occurred when calculating the shader's color: {:?}", err)
                        }
                    }
                }
                Some(color_pass)
            }
            None => return None,
        }
    }


    pub fn trace_rays_for_shadows(&self, light: &LightEnum, mut rc: &mut RayContext, sc: &Arc<RwLock<Scene>>) -> Result<NormalizedF, SysError> {
        match light {
            LightEnum::AreaLight(a) => {
                if a.shadow_samples > 1 {
                    return self.trace_rays_for_area_shadows(light, &mut rc, sc);
                }
            },
            _ => {}
        }
        return self.trace_rays_for_shadows_for_normal_light(light, &mut rc, sc);
    }

    pub fn trace_rays_for_shadows_for_normal_light(&self, light: &LightEnum, mut rc: &mut RayContext, sc: &Arc<RwLock<Scene>>) -> Result<NormalizedF, SysError> {

        if light.can_cast_shadow() == false {
            rc.is_in_shadow = false;
            return Ok(1.0);
        }
        let light_to_point_vector = light.get_displacement_vector(None, &rc.intersection_coordinate);
        rc.ray_dir = light_to_point_vector.normalized();
        let distance_to_light = light_to_point_vector.magnitude();
        rc.intersected = false;
        rc.is_in_shadow = false;
        rc.previous_closest_distance = distance_to_light;
        self.bhv_tree.unwrap().inorder_traversal(rc, &mut |_rc, is_leaf, bounding_volume, objects_list| {
            if self.test_against_bb(bounding_volume, _rc) == false {
                return Continue;
            }

            if is_leaf {
                for object_index in objects_list {
                    let geom = self.geometries.unwrap()[*object_index].clone();
                    if !geom.render_attributes.shadows.receive {
                        continue;
                    }
                    match self.trace_single_ray(&geom, _rc) {
                        Ok(()) => {
                            if _rc.intersected == true {
                                _rc.is_in_shadow = true;
                                return DontContinue;
                            }
                        }
                        Err(err) => {
                            return DontContinue
                        }
                    }
                }
            }

            Continue
        });


        if rc.is_in_shadow {
            return Ok(0.0);
        }
        Ok(1.0)
    }

    pub fn trace_rays_for_area_shadows(&self, light: &LightEnum, mut rc: &mut RayContext, sc: &Arc<RwLock<Scene>>) -> Result<NormalizedF, SysError> {
        if light.can_cast_shadow() == false {
            rc.is_in_shadow = false;
            return Ok(1.0);
        }
        rc.area_light_sampled_points = Some(light.get_samples());
        if rc.area_light_sampled_points.is_none() {
            return self.trace_rays_for_shadows_for_normal_light(light, &mut rc, sc);
        }

        rc.area_light_samples_count = light.get_samples_count();
        let mut multiplier = 0.0;
        for (i, point) in rc.area_light_sampled_points.clone().unwrap().iter().enumerate() {
            let light_to_point_vector = light.get_displacement_vector(Some(&point), &rc.intersection_coordinate);
            rc.ray_dir = light_to_point_vector.normalized();
            let distance_to_light = light_to_point_vector.magnitude();
            rc.intersected = false;
            rc.is_in_shadow = false;
            rc.previous_closest_distance = distance_to_light;

            self.bhv_tree.unwrap().inorder_traversal(rc, &mut |_rc, is_leaf, bounding_volume, objects_list| {
                if self.test_against_bb(bounding_volume, _rc) == false {
                    return Continue;
                }

                if is_leaf {
                    for object_index in objects_list {
                        let geom = self.geometries.unwrap()[*object_index].clone();
                        if !geom.render_attributes.shadows.receive {
                            continue;
                        }
                        match self.trace_single_ray(&geom, _rc) {
                            Ok(()) => {
                                if _rc.intersected == true {
                                    _rc.is_in_shadow = true;
                                    return DontContinue;
                                }
                            }
                            Err(err) => {
                                return panic!("{}", err.to_string());
                            }
                        }
                    }
                }

                Continue
            });

            if rc.is_in_shadow {
                multiplier += 0.0;
            } else {
                multiplier += 1.0;
            }
        }
        let multiplier = multiplier / (rc.area_light_samples_count as f64);
        Ok(multiplier)
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
                if let Some(color_pass) = self.process_ray_for_objects(rc, sc) {
                    final_color += color_pass.composite();
                }
            }
            return Ok(Some(final_color.divide_by_scalar(num_of_emitted_rays as f64)));
        }
        if let Some(color) = self.process_ray_for_objects(rc, sc) {
            return Ok(Some(color.composite()));
        }
        Ok(None)
    }


    pub fn test_against_bb(&self, bb: &AABB, rc: &RayContext) -> bool {
        let mut t_min = f64::NEG_INFINITY;
        let mut t_max = f64::INFINITY;

        for i in 0..3 {
            let mut t1 = (bb.min[i] - rc.origin_coordinate[i]) * rc.inv_ray_dir[i];
            let mut t2 = (bb.max[i] - rc.origin_coordinate[i]) * rc.inv_ray_dir[i];


            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }

            t_min = t_min.max(t1);
            t_max = t_max.min(t2);

            if t_max < t_min {
                return false;
            }
        }

        if t_max < 0.0 {
            return false;
        }
        true
    }

    pub fn trace_single_ray(&self, obj: &Geometry, rc: &mut RayContext) -> Result<(), SysError> {
        if obj.render_attributes.renderable == false {
            return Ok(());
        }
        rc.intersected_geo_type = Some(obj.geometry_type.clone());
        rc.intersected_geo_subtype = Some(obj.geometry_subtype.clone());
        rc.intersected_object_centroid = Some(obj.get_centroid());
        rc.do_smooth = obj.render_attributes.smooth.enable;
        if obj.geometry_type == Polygon {
            obj.bvh_tree.as_ref().unwrap().inorder_traversal(rc, &mut |_rc, is_leaf, bounding_volume, triangles_list| {
                if self.test_against_bb(bounding_volume, _rc) == false {
                    // return early
                    return Continue;
                }
                if is_leaf {
                    for triangle_index in triangles_list {
                        let f_index = *triangle_index;
                        let face_coordinates = obj.data._faces_computed[f_index];
                        let mut vertex_normals: [Vec3f; 3] = [Vec3f::default(); 3];
                        if let Some(c) = self.use_moller_trumbore(_rc, &face_coordinates)
                        {
                            if _rc.is_closest_so_far(c.0) {
                                vertex_normals[0] = obj.data.vertex_normals[obj.data.face_to_v_normals[f_index][0] as usize];
                                vertex_normals[1] = obj.data.vertex_normals[obj.data.face_to_v_normals[f_index][1] as usize];
                                vertex_normals[2] = obj.data.vertex_normals[obj.data.face_to_v_normals[f_index][2] as usize];
                                let v_normal = Utils::calc_vertices_normal(_rc.intersection_coordinate_barycentric_u,
                                                                           _rc.intersection_coordinate_barycentric_v, &vertex_normals);
                                _rc.update_intersection(_rc.next_object_index, Some(f_index), Some(obj.data.face_normals[f_index]),
                                                       Some(v_normal),
                                                       c.0, c.1, _rc.do_smooth);
                            }

                            if _rc.ray_type == ShadowRay {
                                return DontContinue
                            }
                        }
                    }
                }

                Continue
            });
        } else if obj.geometry_type == Procedural {
            _ = self.try_over_procedural(obj, rc);
        } else {
            return Err(SysError::new_str(ErrorKind::GeometryTypeUndefined, "cannot understand the geometry type"))
        }
        Ok(())
    }


    pub fn try_over_procedural(&self, geo: &Geometry, rc: &mut RayContext) -> Result<(), SysError> {
        match geo.geometry_subtype.clone() {
            GeometrySubType::Sphere => {
                let r = geo.data.params.get("radius");
                match r {
                    Some(rr) => {

                        if let Some(r_intr) = self.use_sphere_detection(&rc.origin_coordinate, &rc.ray_dir, &geo.transform.translate, rr.v_f64) {
                            let normal = get_sphere_normal(&r_intr.1, &geo.transform.translate);
                            _ = rc.update_intersection(rc.next_object_index, None, Some(normal), Some(normal), r_intr.0, r_intr.1, false);
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
        rc: &mut RayContext,
        triangle: &[Vec3f; 3],
    ) -> Option<(f64, Vec3f)> {
        let e1 = triangle[1] - triangle[0];
        let e2 = triangle[2] - triangle[0];
        let s = rc.origin_coordinate - triangle[0];

        let p = rc.ray_dir.cross3(&e2);
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
        let q = s.cross3(&e1);
        let v = f * (VectorArithmetic::dot(&rc.ray_dir, &q));
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = f * VectorArithmetic::dot(&e2, &q);

        if t > 0.0 {
            rc.intersection_coordinate_barycentric_u = u;
            rc.intersection_coordinate_barycentric_v = v;
            return Some((t, &rc.origin_coordinate + &rc.ray_dir.multiply_scalar(t)));
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


    fn get_bg_color(&self, ray_dir_n: &Vec3f) -> RenderPass {
        match self.env_bg.as_ref() {
            None => {
                RenderPass::new_from_color(Color::r_to_n(&BLACK))
            }
            Some(env) => {
                RenderPass::new_from_color(env.get_dome_color(ray_dir_n))
            }
        }
    }

}

#[cfg(test)]

mod test {
    use crate::render::renderer::Renderer;
    use crate::scene::scene::Scene;
    use std::sync::{Arc, RwLock};

    #[test]
    fn test_ray_emitter() {
        let s = Scene::load_from_file("../resources/scene_examples/scene_tea_and_cups.json");
        assert_eq!(false, s.is_err(), "err={:?}", s.err().unwrap());

        if s.is_ok() {
            let s = s.unwrap();
            let mut renderer = Renderer::new(Arc::new(RwLock::new(s)));
            _ = renderer.render(None);
        }
    }
}
