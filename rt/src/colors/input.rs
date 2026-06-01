use std::any::Any;
use std::f32::consts::PI;
use crate::colors::types::{InputChannel, NColor3, ProceduralTexture};
use crate::common::params::Params;
use crate::object::geometry::GeometrySubType::Sphere;
use crate::object::geometry::GeometryType::Procedural;
use crate::vector::constants::{BLACK, WHITE};
use crate::vector::types::Vector;
use crate::vector::vec3f::Vec3f;

#[derive(Clone)]
pub struct CheckeredTexture {
    scale: f64,
    first_color: NColor3,
    second_color: NColor3,
}

impl Default for CheckeredTexture {
    fn default() -> Self {
        Self {
            scale: 1.0,
            first_color: WHITE,
            second_color: BLACK,
        }
    }
}

impl CheckeredTexture {
    pub fn new(scale: f64) -> CheckeredTexture {
        CheckeredTexture { scale: scale, first_color: BLACK, second_color: WHITE }
    }

    fn compute(&self, v: &Vec3f) -> NColor3 {
        let dir = v.normalized();

        let x = (dir[0] * self.scale).floor();
        let y = (dir[1] * self.scale).floor();
        let z = (dir[2] * self.scale).floor();

        if (x+y+z) % 2.0 == 0.0 {
            self.first_color
        } else {
            self.second_color
        }
    }

    fn compute_for_sphere(&self, v: &Vec3f) -> NColor3 {
        let dir = v.normalized();

        let x = (dir[0] * self.scale).floor();
        let y = (dir[1] * self.scale).floor();
        let z = (dir[2] * self.scale).floor();

        if (x+y+z) % 2.0 == 0.0 {
            self.first_color
        } else {
            self.second_color
        }

    }
}



impl ProceduralTexture for CheckeredTexture {
    fn get_texture(&self, params: Option<&Params>) -> NColor3 {
        if let Some(params) = params {
            let origin = params.get("origin");
            let obj_type = params.get("obj_type");
            let obj_subtype = params.get("obj_subtype");
            let center = params.get("obj_center");

            if let (Some(origin), Some(obj_type), Some(subtype)) = (origin, obj_type, obj_subtype) {
                if obj_type.v_str == Procedural && subtype.v_str == Sphere {
                    if let Some(sphere_center) = center {
                        let origin = origin.v_vec3f.unwrap() - sphere_center.v_vec3f.unwrap();
                        return self.compute(&origin);
                    }
                }
            } else if let Some(origin) = origin {
                return self.compute(&origin.v_vec3f.unwrap());
            }
        }

        self.first_color
    }

    fn equals(&self, other: &dyn ProceduralTexture) -> bool {
        if self.get_texture(None) == other.get_texture(None) {
            return true;
        }
        false
    }
}