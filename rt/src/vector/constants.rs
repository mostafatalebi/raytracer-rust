use crate::vector::vec3f::Vec3f;
use crate::vector::vec4f::Vec4f;

pub const WHITE: Vec4f = Vec4f([250.0, 250.0, 250.0, 255.0]);
pub const GRAY: Vec4f = Vec4f([125.0, 125.0, 125.0, 255.0]);
pub const OCEAN_BLUE: Vec4f = Vec4f([37.0, 150.0, 190.0, 255.0]);
pub const MUTED_PLUM: Vec4f = Vec4f([135.0, 84.0, 115.0, 255.0]);
pub const BLACK: Vec4f = Vec4f([0.0, 0.0, 0.0, 255.0]);
pub const SKY_BLUE: Vec4f      = Vec4f([135.0, 206.0, 235.0, 255.0]);
pub const CYAN: Vec4f          = Vec4f([0.0,   255.0, 255.0, 255.0]);
pub const SOFT_BLUE: Vec4f     = Vec4f([100.0, 149.0, 237.0, 255.0]);
pub const SOFT_PINK: Vec4f     = Vec4f([255.0, 182.0, 193.0, 255.0]);
pub const MAGENTA: Vec4f       = Vec4f([255.0, 0.0,   255.0, 255.0]);

pub const CAST_DAY: Vec4f = Vec4f([255.0, 244.0, 214.0, 255.0]);


// i hat
pub const WORLD_UP: Vec3f = Vec3f([0.0, 1.0, 0.0]);
// j hat
pub const WORLD_RIGHT: Vec3f = Vec3f([1.0, 0.0, 0.0]);
// k hat
pub const WORLD_Z: Vec3f = Vec3f([0.0, 0.0, -1.0]);