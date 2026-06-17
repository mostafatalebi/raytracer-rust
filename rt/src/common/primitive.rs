use std::fmt;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone, Default)]
pub enum PrimitiveType {
    #[default]
    Undefined,
    Geometry,
    Light,
    Camera,
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PrimitiveType::Undefined => "undefined",
            PrimitiveType::Geometry  => "geometry",
            PrimitiveType::Light     => "light",
            PrimitiveType::Camera    => "camera",
        };
        write!(f, "{}", s)
    }
}

// PrimitiveType → String
impl From<PrimitiveType> for String {
    fn from(p: PrimitiveType) -> Self {
        p.to_string()
    }
}

// &str → PrimitiveType
impl From<&str> for PrimitiveType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "geometry"  => PrimitiveType::Geometry,
            "light"     => PrimitiveType::Light,
            "camera"    => PrimitiveType::Camera,
            _           => PrimitiveType::Undefined,
        }
    }
}

// String → PrimitiveType
impl From<String> for PrimitiveType {
    fn from(s: String) -> Self {
        PrimitiveType::from(s.as_str())
    }
}