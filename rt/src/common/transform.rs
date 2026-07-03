use serde::{de, Deserialize, Deserializer, Serialize};
use crate::matrix::column_major_trait::ColumnMajor;
use crate::matrix::m4x4::Matrix4x4;
use crate::quaternion::quaternion::Quaternion;
use crate::vector::vec3f::Vec3f;



#[derive(Clone, Deserialize, Serialize)]
pub struct Transform {
    pub translate: Vec3f,
    #[serde(deserialize_with = "deserialize_rotate")]
    pub rotate: Quaternion,
    pub scale: Vec3f
}


impl Transform {
    pub fn move_params(&mut self, x_unit: f64, y_unit: f64, z_unit: f64) {
        self.translate[0] += x_unit;
        self.translate[1] += y_unit;
        self.translate[2] += z_unit;
    }

    pub fn move_vec3f(&mut self, add: &Vec3f) {
        self.translate[0] += add[0];
        self.translate[1] += add[1];
        self.translate[2] += add[2];
    }


    pub fn rotate(&mut self, r: Vec3f) {
        let q = Quaternion::new_from_euler(&r);
        self.rotate = (self.rotate * q).normalize()
    }


    pub fn rotate_by(&mut self, x_unit: f64, y_unit: f64, z_unit: f64) {
        let q = Quaternion::new_from_euler(&Vec3f::new(x_unit, y_unit, z_unit));
        self.rotate = (self.rotate * q).normalize()
    }

    pub fn get_u_v_dir(&mut self, w: f64, h: f64, u: &Vec3f, v: &Vec3f) -> (Vec3f, Vec3f, Vec3f, Vec3f) {
        let u_dir = self.rotate.rotate_vec3f(u);
        let v_dir = self.rotate.rotate_vec3f(v);

        let u_vec = u_dir * w;
        let v_vec = v_dir * h;

        (u_dir, v_dir, u_vec, v_vec)
    }


    pub fn scale(&mut self, x_unit: f64, y_unit: f64, z_unit: f64) {
        self.scale[0] += x_unit;
        self.scale[1] += y_unit;
        self.scale[2] += z_unit;
    }




    pub fn get_m4x4(&self) -> Matrix4x4 {
        let mut r = self.rotate.to_m3x3();

        r.cm_mul(0, 0, self.scale[0]);
        r.cm_mul(0, 1, self.scale[0]);
        r.cm_mul(0, 2, self.scale[0]);

        r.cm_mul(1, 0, self.scale[1]);
        r.cm_mul(1, 1, self.scale[1]);
        r.cm_mul(1, 2, self.scale[1]);

        r.cm_mul(2, 0, self.scale[2]);
        r.cm_mul(2, 1, self.scale[2]);
        r.cm_mul(2, 2, self.scale[2]);

        Matrix4x4::from_m3x3(&r, &Vec3f::new(0.0,0.0,0.0), &self.translate, 1.0)
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
        Self {
            translate: Vec3f::default(),
            rotate: Quaternion::default(),
            scale: Vec3f::new(1.0, 1.0, 1.0),
        }
    }
}


pub fn deserialize_rotate<'de, D>(deserializer: D) -> Result<Quaternion, D::Error>
where
    D: Deserializer<'de>,
{
    let res = Vec3f::deserialize(deserializer);

    if res.is_err() {
        return Err(serde::de::Error::custom("rotate deserialize failed"));
    }

    Ok(Quaternion::new_from_euler(&res.unwrap()))
}