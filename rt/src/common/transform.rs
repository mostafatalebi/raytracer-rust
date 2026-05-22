use serde::{Deserialize, Serialize};
use crate::matrix::column_major_trait::ColumnMajor;
use crate::matrix::m4x4::Matrix4x4;
use crate::quaternion::quaternion::Quaternion;
use crate::vector::vec3f::Vec3f;

#[derive(Clone, Deserialize, Serialize)]
pub struct Transform {
    pub local: TransformAttributes,
    pub world: TransformAttributes,
    pub pivot: Vec3f,

}

impl Transform {
    pub fn set_world(&mut self, world: TransformAttributes) {
        self.world = world;
    }

    pub fn set_world_rotate(&mut self, translate: Vec3f) {
        self.world.rotate = Quaternion::new_from_euler(&translate);
    }
    pub fn set_local_rotate(&mut self, translate: Vec3f) {
        self.local.rotate = Quaternion::new_from_euler(&translate);
    }
    pub fn set_world_scale(&mut self, translate: Vec3f) {
        self.world.scale = translate;
    }
    pub fn set_local_scale(&mut self, translate: Vec3f) {
        self.local.scale = translate;
    }
    pub fn set_local(&mut self, local: TransformAttributes) {
        self.world = local;
    }

    pub fn move_world(&mut self, x_unit: f64, y_unit: f64, z_unit: f64) {
        self.world.translate[0] += x_unit;
        self.world.translate[1] += y_unit;
        self.world.translate[2] += z_unit;
    }

    pub fn move_world_vec3f(&mut self, add: &Vec3f) {
        self.world.translate[0] += add[0];
        self.world.translate[1] += add[1];
        self.world.translate[2] += add[2];
    }

    pub fn move_local(&mut self, x_unit: f64, y_unit: f64, z_unit: f64) {
        self.local.translate[0] += x_unit;
        self.local.translate[1] += y_unit;
        self.local.translate[2] += z_unit;
    }

    pub fn move_local_vec3f(&mut self, add: &Vec3f) {
        self.local.translate[0] += add[0];
        self.local.translate[1] += add[1];
        self.local.translate[2] += add[2];
    }

    pub fn rotate_world(&mut self, x_unit: f64, y_unit: f64, z_unit: f64) {
        let q = Quaternion::new_from_euler(&Vec3f::new(x_unit, y_unit, z_unit));
        self.world.rotate *= &q
    }

    pub fn rotate_local(&mut self, x_unit: f64, y_unit: f64, z_unit: f64) {
        let q = Quaternion::new_from_euler(&Vec3f::new(x_unit, y_unit, z_unit));
        self.local.rotate = (self.local.rotate * q).normalize()
    }

    pub fn scale_world(&mut self, x_unit: f64, y_unit: f64, z_unit: f64) {
        self.world.scale[0] += x_unit;
        self.world.scale[1] += y_unit;
        self.world.scale[2] += z_unit;
    }

    pub fn scale_local(&mut self, x_unit: f64, y_unit: f64, z_unit: f64) {
        self.local.scale[0] += x_unit;
        self.local.scale[1] += y_unit;
        self.local.scale[2] += z_unit;
    }


    pub fn set_pivot(&mut self, v: &Vec3f) {
        self.pivot = v.clone();
    }




    pub fn get_m4x4(&self) -> Matrix4x4 {
        let mut r = self.local.rotate.to_m3x3();

        r.cm_mul(0, 0, self.local.scale[0]);
        r.cm_mul(0, 1, self.local.scale[0]);
        r.cm_mul(0, 2, self.local.scale[0]);

        r.cm_mul(1, 0, self.local.scale[1]);
        r.cm_mul(1, 1, self.local.scale[1]);
        r.cm_mul(1, 2, self.local.scale[1]);

        r.cm_mul(2, 0, self.local.scale[2]);
        r.cm_mul(2, 1, self.local.scale[2]);
        r.cm_mul(2, 2, self.local.scale[2]);

        Matrix4x4::from_m3x3(&r, &Vec3f::new(0.0,0.0,0.0), &self.local.translate, 1.0)
    }


    // this approach is naive. It applies the rotation to each
    // vertex; for the start, it's good, but we can optimize it
    // later using ray-orientation (instead of this vertex's one)
    fn apply_rotation(&self, rotation_q: &Quaternion, vertices: &mut Vec<Vec3f>) {
        for vertex in vertices.iter_mut() {
            let qv = Vec3f::new(rotation_q.x, rotation_q.y, rotation_q.z);
            let t = qv.cross3(&vertex) * 2.0;
            *vertex = vertex.clone() + (&t * rotation_q.w + qv.cross3(&t))
        }
    }


    fn transform_vertices(&self, m: &Matrix4x4, vertices: &mut Vec<Vec3f>) {
        for vertex in vertices.iter_mut() {
            let v = self.transform_single_vertex(m, vertex);
            *vertex = v
        }
    }

    fn transform_single_vertex(&self, m: &Matrix4x4, vertex: &Vec3f) -> Vec3f {
        Vec3f::new(
            m[0][0] * vertex[0] + m[1][0] * vertex[1] + m[2][0] * vertex[2] + m[3][0],
            m[0][1] * vertex[0] + m[1][1] * vertex[1] + m[2][1] * vertex[2] + m[3][1],
            m[0][2] * vertex[0] + m[1][2] * vertex[1] + m[2][2] * vertex[2] + m[3][2],
        )
    }

    pub fn run_transform_pipeline(&mut self, vertices: &Vec<Vec3f>) -> Vec<Vec3f> {
        let mut transformed = vertices.clone();
        self.transform_vertices(&self.get_m4x4(), &mut transformed);
        transformed
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform{
            local: TransformAttributes::default(),
            world: TransformAttributes::default(),
            pivot: Vec3f::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TransformAttributes {
    pub translate: Vec3f,
    pub rotate: Quaternion,
    pub scale: Vec3f
}

impl Default for TransformAttributes {
    fn default() -> Self {
        Self {
            translate: Vec3f::default(),
            rotate: Quaternion::default(),
            scale: Vec3f::new(1.0, 1.0, 1.0),
        }
    }
}