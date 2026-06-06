use std::ops::{Add, Mul};
use rand::{random, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use crate::camera::types::AntiAliasingMethod;
use crate::colors::types::{InputChannel, NColor3};
use crate::common::transform::Transform;
use crate::quaternion::quaternion::Quaternion;
use crate::vector::constants::{BLACK, WORLD_RIGHT, WORLD_UP, WORLD_Z};
use crate::vector::types::{Vec2i, Vector};
use crate::vector::utils::Utils;
use crate::vector::vec2f::Vec2f;
use crate::vector::vec3f::Vec3f;

#[typetag::serde]
pub trait BaseCamera{
    fn get_id(&self) -> String;
}



#[derive(Default, Deserialize, Serialize, Clone)]
pub struct StandardCamera {
    pub id:             String,
    pub transform: Transform,

    #[serde(skip)]
    pub resolution: Vec2i,

    // @todo at the time of creating image plane
    // we either must use concrete sensor sizes or
    // rely on aspect ratio. For now, we use sensor
    // sizes, but we can allow both and use one
    // defined at the time of camera creation
    pub aspect_ratio: f64,

    // the direction at which the camera is looking
    pub look_at:        Option<Vec3f>,

    #[serde(skip)]
    // this is the imaginary plan used for perspective
    // rendering
    pub image_plane_width: f64,
    #[serde(skip)]
    pub image_plane_height: f64,

    // lens focal length, default 50mm
    pub focal_length:   f64,
    #[serde(skip)]
    pub _fov: Vec2f,

    #[serde(skip)]
    pub _sensor_size:   Vec2i,

    // direction vectors
    #[serde(skip)]
    forward:       Vec3f,
    #[serde(skip)]
    up:           Vec3f,
    #[serde(skip)]
    right:       Vec3f,

    #[serde(skip)]
    background: InputChannel,

}

#[typetag::serde]
impl BaseCamera for StandardCamera {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}



impl StandardCamera {
    // new instance without applying any internal calculations
    // the instance is not usable unless other settings method
    // be called
    pub fn new(resolution: Vec2i, sensor_size: Vec2i, look_at: Option<Vec3f>, up: Vec3f, focal_length: f64, origin: Option<Vec3f>) -> Self{
        let aspect_ratio = resolution[0] as f64 /resolution[1] as f64;
        let mut c = Self{
            // @todo must grab from global ID pool
            id: "camera".to_string(),
            resolution: resolution,
            transform: Default::default(),
            aspect_ratio: aspect_ratio,
            focal_length: focal_length,
            look_at: None,
            _fov: Vec2f::default(),
            _sensor_size: sensor_size,
            forward: Default::default(),
            up: up,
            right: Default::default(),
            image_plane_width: 0.0,
            image_plane_height: 0.0,
            background: InputChannel::new_with_color(BLACK),
        };

        if let Some(o) = origin {
            c.transform.local.translate = o;
        }


        c.configure();

        c
    }

    pub fn set_res(&mut self, width: usize, height: usize) {
        self.resolution[0] = width as i64;
        self.resolution[1] = height as i64;
    }


    // configures the camera based on basic essential
    // values
    pub fn configure(&mut self) {
        self.calc_procedural_image_plane();
        self.configure_orientation();
    }

    pub fn lock_to(&mut self, look_at: Vec3f) {
        self.look_at = Some(look_at);
        self.configure_orientation()
    }

    pub fn unlock(&mut self) {
        self.look_at = None;
        self.configure_orientation()
    }

    pub fn calc_procedural_image_plane(&mut self) {
        // FOV is calculated using: 2 x arctan(sensor / 2 * focal length)
        self._fov = Utils::calc_fov(&self._sensor_size, &self.focal_length);
        self.image_plane_height = 2.0 * self.focal_length * (self._fov[1]/ 2.0).tan();
        self.image_plane_width = self.image_plane_height * self.aspect_ratio;
    }

    pub fn create_frustum(&mut self, fov_y: f64, aspect: f64, near: f64, far: f64) -> [Vec3f; 8]{
        let half_tan_of_fov_y = fov_y.to_radians() / 2.0;
        let near_height = 2.0 / near * half_tan_of_fov_y;
        let near_width = 2.0 / near_height * aspect;

        let far_height = 2.0 * far * half_tan_of_fov_y;
        let far_width = far_height * aspect;


        let mut corners: [Vec3f; 8] = [Vec3f::default(); 8];

        // Counter Clock Wise, from bottom-left
        corners[0] = Vec3f::new(-near_width/2.0, -near_height, -near);
        corners[1] = Vec3f::new(near_width/2.0, -near_height, -near);
        corners[2] = Vec3f::new(near_width/2.0, near_height, -near);
        corners[3] = Vec3f::new(-near_width/2.0, near_height, near);


        corners[4] = Vec3f::new(-far_width/2.0, -far_height, -far);
        corners[5] = Vec3f::new(far_width/2.0, -far_height, -far);
        corners[6] = Vec3f::new(far_width/2.0, far_height, -far);
        corners[7] = Vec3f::new(-far_width/2.0, far_height, -far);

        corners
    }


    pub fn configure_orientation(&mut self) {
        if let Some(look_at) = self.look_at {
            self.transform.local.rotate = Quaternion::look_at(&self.transform.local.translate, &look_at, &WORLD_UP);
        }
        self.update_basis_axis();
    }

    fn update_basis_axis(&mut self) {
        self.forward = self.transform.local.rotate.rotate_vec3f(&WORLD_Z);
        self.right = self.transform.local.rotate.rotate_vec3f(&WORLD_RIGHT);
        self.up = self.transform.local.rotate.rotate_vec3f(&WORLD_UP);
    }


    pub fn get_anti_aliased_pixel_coordinates(&self, i: i64, j: i64, anti_aliasing: u8, method: &AntiAliasingMethod) -> Vec<Vec3f> {
        let mut pixels = vec![Vec3f::default(); anti_aliasing as usize];
        for k in 0..anti_aliasing {
            pixels[k as usize] = self.get_pixel_coordinates_with_sampler(i, j, k, anti_aliasing, &method);
        }

        pixels
    }

    fn get_pixel_coordinates_with_sampler(&self, i: i64, j: i64, k: u8, total_samples_count: u8, method: &AntiAliasingMethod) -> Vec3f {
        let i_sample: f64;
        let j_sample: f64;
        match method {
            AntiAliasingMethod::Uniform => {
                let each_grid_cell = 1.0 / (total_samples_count+1) as f64;
                i_sample = i as f64 + (each_grid_cell * (k+1) as f64);
                j_sample = j as f64 + (each_grid_cell * (k+1) as f64);
            },
            AntiAliasingMethod::MonteCarlo => {
                i_sample = i as f64 + random::<f64>();
                j_sample = j as f64 + random::<f64>();
            }
        }

        let ndc = StandardCamera::get_ndc(&self.resolution, i_sample, j_sample);
        let screen_space = StandardCamera::get_screen_space(ndc[0], ndc[1]);
        let fov = Utils::calc_fov(&self._sensor_size, &self.focal_length);
        let scale = (fov[1] / 2.0).tan();
        let x = screen_space[0] * self.aspect_ratio * scale;
        let y = screen_space[1] * scale;

        self.transform.local.translate
            .add_with(&self.forward)
            .add_with(&self.right.multiply_scalar(x))
            .add_with(&self.up.multiply_scalar(y))
    }


    pub fn get_pixel_coordinates(&self, i: i64, j: i64) -> Vec3f {
        let ndc = StandardCamera::get_ndc(&self.resolution, i as f64, j as f64);
        let screen_space = StandardCamera::get_screen_space(ndc[0], ndc[1]);
        let fov = Utils::calc_fov(&self._sensor_size, &self.focal_length);
        let scale = (fov[1] / 2.0).tan();
        let x = screen_space[0] * self.aspect_ratio * scale;
        let y = screen_space[1] * scale;

        self.transform.local.translate
            .add_with(&self.forward)
            .add_with(&self.right.multiply_scalar(x))
            .add_with(&self.up.multiply_scalar(y))
    }

    // gets the NDC coordinates of a pixel. NDC stands for normalized device coordinates,
    // and instead of directly dealing with pixel indices (which is awkward), it abstracts
    // them with values from 0 to 1. Any pixel index will correspond to a coordinate
    // in the range 0 to 1.
    // This function gets used in conjunction with get_screen_space to convert pixel indices
    // to screen coordinates.
    pub fn get_ndc(res: &Vec2i, i: f64, j: f64) -> [f64; 2] {
        [(i) / res[0] as f64,
        (j) / res[1] as f64]
    }

    // converts NDC to screen space
    // NDC (normalized device coordinates) returns coordinates
    // in range 0-1, but we need to have a -1.0-1 range, hence
    // this function converts any NDC coordinate to
    // screen coordinates (like 0.5 of NDC to 0 of screen space).
    // The formula ensures that the screen space coordinate is
    // applied.
    pub fn get_screen_space(u: f64, v: f64) -> [f64; 2] {
        [2.0 * u - 1.0,
        1.0  - 2.0 *v]
    }

    pub fn get_forward(&self) -> Vec3f {
        return self.forward;
    }
    pub fn get_right(&self) -> Vec3f {
        return self.right;
    }

    pub fn get_up(&self) -> Vec3f {
        return self.up;
    }
    pub fn get_fov(&self) -> Vec2f {
        return self._fov.clone()
    }

    pub fn get_aspect_ratio(&self) -> f64 {
        return self.aspect_ratio
    }

    pub fn get_look_at(&self) -> Option<Vec3f> {
        return self.look_at
    }

    // pan means moving the camera in 2d coordinates.
    // This translates to moving the camera and the look_at's target
    // at the same time and with the same amount.
    pub fn pan(&mut self, x: f64, y: f64) {
        if let Some(ref mut look_at) = self.look_at {
            let offset = Vec3f::new(x, y, 0.0);
            self.transform.local.translate += offset;
            *look_at += offset;
        }
    }


    pub fn sample_background(&self, u: f64, v: f64) -> NColor3 {
        BLACK
    }

}


#[cfg(test)]

mod tests {
    use crate::camera::aspect_ratios::RES_FHD;
    use super::*;

    #[test]
    fn test_get_pixel_position() {
        let mut c = StandardCamera::new(Vec2i([100,100]), Vec2i([100,100]), Some(WORLD_Z),WORLD_UP,50.0, None);
        let pos = c.get_pixel_coordinates(0, 0);
        assert_eq!(pos.trunc(10000), Vec3f::new(-0.2375, 0.2375, 50.0));

        let expected_ndc = [(20.0+0.5)/1920f64, (3.0+0.5)/1080f64];
        let real_ndc = StandardCamera::get_ndc(&Vec2i(RES_FHD), 20.0, 3.0);
        assert_eq!(expected_ndc, real_ndc);

        let c = StandardCamera::new(Vec2i(RES_FHD), Vec2i([100,100]), Some(WORLD_Z),WORLD_UP,50.0, None);
        let pos = c.get_pixel_coordinates(1919, 1079);
        assert_eq!(pos.trunc(10000), Vec3f::new(0.4263, 0.2397, 50.0));
    }
}