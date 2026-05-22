use serde::{Deserialize, Serialize};
use crate::common::transform::Transform;
use crate::vector::types::{Vec3i, Vector};
use crate::scene::render_attributes::RenderAttributes;
use crate::vector::vec3f::Vec3f;

#[derive(Deserialize, Serialize, Default, Clone)]
pub enum GeometryType {
    #[serde(rename = "polygon")]
    #[default]
    Polygon,
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct GeometryData {
    pub vertices: Vec<Vec3f>,

    // faces must absolutely be triangles; if they are quadratic,
    // it throws an error
    pub faces:    Vec<Vec3i>,

    // the sum of the values of each normal
    // entry must be 1 (it's a unit vector of size 3)
    pub face_normals:  Vec<Vec3f>,

    // vertex normals computed on the fly
    pub vertex_normals:  Vec<Vec3f>,
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Geometry {
    pub geometry_type: GeometryType,
    pub id: String,
    pub transform: Transform,
    pub render_attributes: RenderAttributes,
    pub data: GeometryData
}

impl Geometry {

    // when doing move, rotate and scale, a call to this function
    // is needed to apply the actual transformation to the geometry,
    // otherwise most of the transformation values will be ignored
    // in the final render
    pub fn apply_transformations(&mut self) {
        self.data.vertices = self.transform.run_transform_pipeline(&self.data.vertices);
    }

    pub fn assign_shader(&mut self, material_name: &str) {
        self.render_attributes.material_name = material_name.to_string();
    }

    pub fn calc_all_normals(&mut self) {
        self.calc_face_normals();
        self.compute_vertices_normals();
    }
    pub fn calc_face_normals(&mut self) {
        self.data.face_normals = vec![Vec3f::default(); self.data.faces.len()];
        for face in self.data.faces.iter().enumerate() {
            self.data.face_normals[face.0] = self.calc_single_face_normal(&face.1);
        }
    }

    fn calc_single_face_normal(&self, v: &Vec3i) -> Vec3f {
        let vx0 = self.data.vertices[v[0] as usize];
        let vx1 = self.data.vertices[v[1] as usize];
        let vx2 = self.data.vertices[v[2] as usize];

        let edge1 = vx1 - vx0;
        let edge2 = vx2 - vx0;
        (&edge1).cross3(&edge2).normalized()
    }

    /// This fn must only be called when face normals
    /// are ready. Face normals either are computed using calc_face_normals()
    /// or imported externally or inserted manually.
    pub fn compute_vertices_normals(&mut self) {
        let mut v_normals = vec![Vec3f::default(); self.data.vertices.len()];
        for face in self.data.faces.iter().enumerate() {
            let vn = self.data.face_normals[face.0];
            v_normals[face.1[0] as usize] += vn;
            v_normals[face.1[1] as usize] += vn;
            v_normals[face.1[2] as usize] += vn;
        }

        for n in &mut v_normals {
            *n = n.normalized()
        }

        self.data.vertex_normals = v_normals
    }
}


#[derive(Default)]
pub struct Plane {
    pub id: String,
    pub width: f64,
    pub height: f64,
    pub transform: Transform,
}

impl Plane {
    pub fn new() -> Plane {
        Plane{
            // @todo must grab from a global ID generator
            id: "plane_".to_string(),
            width: 1.0,
            height: 1.0,
            transform: Transform::default(),
        }
    }

    pub fn new_with_translation(translation: Vec3f) -> Plane {
        let mut plane = Plane::new();
        plane.transform.local.translate = translation;

        plane
    }
}