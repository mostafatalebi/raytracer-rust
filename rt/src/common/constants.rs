use crate::common::types::NormalizedF;

pub const EPS: f64 = 1e-9;

/// How many rays at maximum should be sampled
/// for each camera ray (when reflection glossiness
/// is not 0.0).
pub const MAX_REFLECTION_SAMPLES: u16 = 64;

/// how scattered should reflection rays be emitted in relation to the reference ray;
/// a higher value (e.g 1.0) means more scattered blurriness; a lower
/// value keeps the blurriness more focused.
/// Cannot be zero
pub const REFLECTION_GLOSSINESS_SCATTER_FACTOR: NormalizedF = 0.5;