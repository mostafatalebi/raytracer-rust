use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::bounding_box::aabb::AABB;
use crate::common::id::{AutoId, Id};
use crate::common::params::Params;
use crate::common::transform::Transform;
use crate::vector::types::{Vec3i, Vector};
use crate::scene::render_attributes::RenderAttributes;
use crate::vector::vec3f::Vec3f;

#[derive(Deserialize, Serialize, Default, Clone, PartialEq, Debug)]
pub enum GeometryType {
    #[serde(rename = "polygon")]
    #[default]
    Polygon,

    #[serde(rename = "procedural")]
    Procedural,
}
impl FromStr for GeometryType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "polygon" => Ok(Self::Polygon),
            "procedural" => Ok(Self::Procedural),
            _ => Err(format!("Invalid GeometryType: {}", s)),
        }
    }
}

impl fmt::Display for GeometryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Polygon => "polygon",
            Self::Procedural => "procedural",
        };

        write!(f, "{s}")
    }
}
impl GeometryType {
    fn to_string(&self) -> String {
        match self {
            Self::Polygon => "polygon".to_string(),
            Self::Procedural => "procedural".to_string(),
        }
    }

    fn to_u8(&self) -> u8 {
        match self {
            Self::Polygon => 1,
            Self::Procedural => 2,
        }
    }
}

impl PartialEq<str> for GeometryType {
    fn eq(&self, other: &str) -> bool {
        if self.to_string() == other {
            return true;
        }
        false
    }
}

impl PartialEq<String> for GeometryType {
    fn eq(&self, other: &String) -> bool {
        if self.to_string() == *other {
            return true;
        }
        false
    }

}
impl PartialEq<GeometryType> for String {
    fn eq(&self, other: &GeometryType) -> bool {
        if self == &other.to_string() {
            return true;
        }
        false
    }
}
impl PartialEq<GeometryType> for u8 {
    fn eq(&self, other: &GeometryType) -> bool {
        if self == &other.to_u8() {
            return true;
        }
        false
    }
}
impl PartialEq<u8> for GeometryType {
    fn eq(&self, other: &u8) -> bool {
        if self.to_u8() == *other {
            return true;
        }
        false
    }
}


#[derive(Deserialize, Serialize, Default, Clone, PartialEq, Debug)]
pub enum GeometrySubType {
    #[default]
    None,

    #[serde(rename = "sphere")]
    Sphere,

    #[serde(rename = "cube")]
    Cube,

    #[serde(rename = "area_light_shape")]
    AreaLightShape,
}


impl FromStr for GeometrySubType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sphere" => Ok(Self::Sphere),
            "cube" => Ok(Self::Cube),
            "area_light_shape" => Ok(Self::AreaLightShape),
            _ => Err(format!("Invalid GeometrySubType: {}", s)),
        }
    }
}

impl fmt::Display for GeometrySubType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::None   => "none",
            Self::Sphere => "sphere",
            Self::Cube   => "cube",
            Self::AreaLightShape   => "area_light_shape",
        };

        write!(f, "{s}")
    }
}

impl GeometrySubType {
    fn to_string(&self) -> String {
        match self {
            Self::None => "none".to_string(),
            Self::Sphere => "sphere".to_string(),
            Self::Cube => "cube".to_string(),
            Self::AreaLightShape => "area_light_shape".to_string(),
        }
    }

    fn to_u8(&self) -> u8 {
        match self {
            Self::None => 50,
            Self::Sphere => 51,
            Self::Cube => 52,
            Self::AreaLightShape => 53,
        }
    }
}

impl PartialEq<str> for GeometrySubType {
    fn eq(&self, other: &str) -> bool {
        if self.to_string() == other {
            return true;
        }
        false
    }
}

impl PartialEq<String> for GeometrySubType {
    fn eq(&self, other: &String) -> bool {
        if self.to_string() == *other {
            return true;
        }
        false
    }
}

impl PartialEq<u8> for GeometrySubType {
    fn eq(&self, other: &u8) -> bool {
        if self.to_u8() == *other {
            return true;
        }
        false
    }
}

impl PartialEq<GeometrySubType> for String {
    fn eq(&self, other: &GeometrySubType) -> bool {
        if *self == other.to_string() {
            return true;
        }
        false
    }
}
impl PartialEq<GeometrySubType> for u8 {
    fn eq(&self, other: &GeometrySubType) -> bool {
        if *self == other.to_u8() {
            return true;
        }
        false
    }
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

    // @todo this field must retire
    // vertex normals computed on the fly
    pub vertex_normals:  Vec<Vec3f>,

    pub face_to_v_normals: Vec<Vec3i>,

    #[serde(skip)]
    pub params: Params,
}

#[derive(Default, Clone)]
pub struct Geometry {
    pub geometry_type: GeometryType,
    pub geometry_subtype: GeometrySubType,
    centroid: Vec3f,
    pub id: String,
    pub transform: Transform,
    pub render_attributes: RenderAttributes,
    pub(crate) bounding_box: AABB,
    pub data: GeometryData
}

impl Geometry {

    // when doing move, rotate and scale, a call to this function
    // is needed to apply the actual transformation to the geometry,
    // otherwise most of the transformation values will be ignored
    // in the final render
    pub fn apply_transformations(&mut self) {
        self.data.vertices = self.transform.run_transform_pipeline(&self.data.vertices);
        self.calc_bounding_volume();
        self.calculate_centroid();
    }

    pub fn assign_shader(&mut self, material_name: &str) {
        self.render_attributes.material_name = material_name.to_string();
    }

    pub fn calc_all_normals(&mut self) {
        if self.geometry_type != GeometryType::Polygon {
            return;
        }
        self.calc_face_normals();
        self.compute_vertices_normals();
    }
    pub fn prepare_geometry(&mut self) {
        self.apply_transformations();
        // self.calc_all_normals();
    }


    pub fn set_centroid_manually(&mut self, c: Vec3f) {
        self.centroid = c;
    }

    fn calculate_centroid(&mut self) {
        if self.geometry_type != GeometryType::Polygon {
            return;
        }
        let mut centroid = Vec3f::default();
        let mut total_area = 0.0;
        for face in &self.data.faces {
            let v1 = self.data.vertices[face[0] as usize];
            let v2 = self.data.vertices[face[1] as usize];
            let v3 = self.data.vertices[face[2] as usize];
            let area = 0.5 * &(&v2 - &v1).cross3(&(&v3 - &v1)).magnitude();

            let triangular_centroid = (v1+v2+v3).divide_by_scalar(3.0);

            centroid += triangular_centroid*area;
            total_area += area;
        }

        self.centroid = centroid.divide_by_scalar(total_area)
    }

    pub fn calc_face_normals(&mut self) {
        if self.geometry_type != GeometryType::Polygon {
            return;
        }
        self.data.face_normals = vec![Vec3f::default(); self.data.faces.len()];
        for face in self.data.faces.iter().enumerate() {
            self.data.face_normals[face.0] = self.calc_single_face_normal(&face.1);
        }
    }

    #[inline(always)]
    pub fn calc_single_face_normal(&self, v: &Vec3i) -> Vec3f {
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
        if self.geometry_type != GeometryType::Polygon {
            return;
        }
        let mut v_normals = vec![Vec3f::default(); self.data.vertices.len()];
        self.data.face_to_v_normals = vec![Vec3i::default(); self.data.faces.len()];

        for (face_index, vertices) in self.data.faces.iter().enumerate() {
            let vn = self.data.face_normals[face_index];
            v_normals[vertices[0] as usize] += vn;
            v_normals[vertices[1] as usize] += vn;
            v_normals[vertices[2] as usize] += vn;

            let f_to_v_normal = Vec3i::new(vertices[0], vertices[2], vertices[2]);
            self.data.face_to_v_normals[face_index] = f_to_v_normal;
        }

        for n in &mut v_normals {
            *n = n.normalized()
        }

        self.data.vertex_normals = v_normals

    }

    fn calc_bounding_volume(&mut self) {
        if self.geometry_type == GeometryType::Polygon {
            self.bounding_box = AABB::calc_bounding_volume_for_polygons(&self.data.vertices)
        } else {
            if self.geometry_subtype == GeometrySubType::Sphere {
                self.bounding_box = AABB::calc_bounding_volume_for_proc_sphere(&self.centroid, self.data.params.get("radius").unwrap().v_f64);
            }
        }
    }

    pub fn get_centroid(&self) -> Vec3f {
        self.centroid
    }
    pub fn get_bb(&self) -> AABB {
        self.bounding_box.clone()
    }

    pub fn enable_smooth(&mut self) {
        self.render_attributes.smooth.enable = true;
    }

    pub fn disable_smooth(&mut self) {
        self.render_attributes.smooth.enable = false;
    }
}


impl Id for Geometry {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}


impl AutoId for Geometry {
    fn auto_id(&mut self) {
        self.id = format!("geo::{}", Uuid::new_v4().to_string());
    }
}