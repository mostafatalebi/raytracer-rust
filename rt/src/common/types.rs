


// used for values form 0.0 to 1.0
pub type NormalizedF = f64;

pub trait ToF64 {
    fn to_f64(&self) -> f64;
}

impl ToF64 for f64 {
    fn to_f64(&self) -> f64 { *self }
}

impl ToF64 for f32 {
    fn to_f64(&self) -> f64 { *self as f64 }
}

impl ToF64 for i64 {
    fn to_f64(&self) -> f64 { *self as f64 }
}

impl ToF64 for i32 {
    fn to_f64(&self) -> f64 { *self as f64 }
}