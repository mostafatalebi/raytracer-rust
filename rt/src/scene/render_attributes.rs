use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::colors::types::{Color, NColor3};
use crate::vector::constants::BLACK;
#[derive(Deserialize, Serialize, Clone)]
pub struct RenderAttributes {
    pub renderable: bool,
    pub material_name: String,
    #[serde(default)]
    pub shadows: GeometryShadowAttributes,
    #[serde(default)]
    pub smooth: SmoothAttributes,
}

impl<'a> Default for RenderAttributes {
    fn default() -> Self {
        RenderAttributes{
            renderable: true,
            material_name: "".to_string(),
            shadows: Default::default(),
            smooth: Default::default(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GeometryShadowAttributes {
    // if the geometry cast shadow or not
    pub cast: bool,

    // if other objects can cast shadows on this
    pub receive: bool,

    // a color that can multiply the shadow color
    pub color: NColor3,
}

impl Default for GeometryShadowAttributes {
    fn default() -> Self {
        GeometryShadowAttributes{cast: true, receive: true, color: Color::r_to_n(&BLACK)}
    }
}

#[derive(Default, Deserialize, Serialize, Clone)]
pub enum SmoothLevel {
    #[default]
    One = 1,
    Two = 2,
    Three = 3
}

impl From<i8> for SmoothLevel {
    fn from(value: i8) -> Self {
        match value {
            1 => SmoothLevel::One,
            2 => SmoothLevel::Two,
            3 => SmoothLevel::Three,
            _ => SmoothLevel::One
        }
    }
}

impl From<&str> for SmoothLevel {
    fn from(value: &str) -> Self {
        match value {
            "1" => SmoothLevel::One,
            "2" => SmoothLevel::Two,
            "3" => SmoothLevel::Three,
            _ => SmoothLevel::One
        }
    }
}

impl From<&SmoothLevel> for String {
    fn from(value: &SmoothLevel) -> Self {
        match value {
            SmoothLevel::One => String::from("1"),
            SmoothLevel::Two => String::from("2"),
            SmoothLevel::Three => String::from("3"),
        }
    }
}




pub fn de_smooth_attr<'de, D>(deserializer: D) -> Result<SmoothLevel, D::Error>
where
    D: Deserializer<'de>,
{
    let s = i8::deserialize(deserializer)?;
    Ok(SmoothLevel::from(s))
}

pub(crate) fn ser_smooth_attr<S>(value: &SmoothLevel, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("level: {}", String::from(value)))
}



#[derive(Default, Deserialize, Serialize, Clone)]
pub struct SmoothAttributes {
    // smooth the geometry in the render or not
    pub enable: bool,

    // level of smoothness
    // NOT implemented 
    #[serde(default, serialize_with = "ser_smooth_attr", deserialize_with = "de_smooth_attr")]
    pub level: SmoothLevel,
}

impl SmoothAttributes {
    pub fn default() -> Self {
        SmoothAttributes{enable: true, level: SmoothLevel::default()}
    }
}