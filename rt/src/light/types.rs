use serde::{Deserialize, Serialize};

pub const AMBIENT_LIGHT: i8 = 3;
pub const POINT_LIGHT: i8 = 4;
pub const DIRECTIONAL_LIGHT: i8 = 5;
pub const SPOT_LIGHT: i8 = 6;


#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub enum Attenuation {
    #[default]
    Flat,
    Linear,
    Quadratic,
    Cube
}