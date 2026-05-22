use serde::{Deserialize, Serialize};

pub enum ObjType {
    Geometry = 100,
    Faces    = 101,
    Vertex   = 102,
    Normal   = 103,
    Light    = 200,
    Camera   = 300,
    Material = 400,
    Invalid  = 99,
}

impl From<&str> for ObjType {
    fn from(s: &str) -> ObjType {
        match s {
            "geometry" => ObjType::Geometry,
            "faces"    => ObjType::Faces,
            "vertex"   => ObjType::Vertex,
            "normal"   => ObjType::Normal,
            "light"    => ObjType::Light,
            "camera"   => ObjType::Camera,
            "material" => ObjType::Material,
            _          => ObjType::Invalid
        }
    }
}


#[derive(Default, Deserialize, Serialize)]
pub enum LightType {
    #[default]
    Invalid,
    Point,
    Directional,
    Spot,
    Area
}

impl From<&str> for LightType {
    fn from(s: &str) -> LightType {
        match s {
            "point" => LightType::Point,
            "directional" => LightType::Directional,
            "spot" => LightType::Spot,
            "area" => LightType::Area,
            _ => LightType::Invalid
        }
    }
}


impl From<LightType> for String {
    fn from(s: LightType) -> String {
        match s {
            LightType::Point   => String::from("point"),
            LightType::Directional => String::from("directional"),
            LightType::Spot  => String::from("spot"),
            LightType::Area   => String::from("area"),
            LightType::Invalid   => String::from("invalid")
        }
    }
}
