use crate::vector::vec3f::Vec3f;
use crate::vector::vec4f::Vec4f;

pub const WHITE: Vec3f        = Vec3f([250.0, 250.0, 250.0]);
pub const GRAY:  Vec3f         = Vec3f([125.0, 125.0, 125.0]);
pub const OCEAN_BLUE: Vec3f   = Vec3f([37.0,  150.0, 190.0]);
pub const MUTED_PLUM: Vec3f   = Vec3f([135.0, 84.0,  115.0]);
pub const BLACK: Vec3f        = Vec3f([0.0,   0.0,   0.0  ]);
pub const SKY_BLUE: Vec3f     = Vec3f([135.0, 206.0, 235.0]);
pub const SUN: Vec3f          = Vec3f([255.0, 236.0, 153.0]);
pub const FAINT_BLUE_WHITE: Vec3f = Vec3f([240.0, 244.0, 255.0]);
pub const FAINT_GREEN: Vec3f = Vec3f([193.0, 219.0, 200.0]);
pub const CYAN: Vec3f         = Vec3f([0.0,   255.0, 255.0]);
pub const SOFT_BLUE: Vec3f    = Vec3f([100.0, 149.0, 237.0]);
pub const SOFT_PINK: Vec3f    = Vec3f([255.0, 182.0, 193.0]);
pub const MAGENTA: Vec3f      = Vec3f([255.0, 0.0,   255.0]);
pub const CAST_DAY: Vec3f     = Vec3f([255.0, 244.0, 214.0]);

// i hat
pub const WORLD_UP: Vec3f = Vec3f([0.0, 1.0, 0.0]);
// j hat
pub const WORLD_RIGHT: Vec3f = Vec3f([1.0, 0.0, 0.0]);
// k hat
pub const WORLD_Z: Vec3f = Vec3f([0.0, 0.0, -1.0]);