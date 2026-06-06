use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Copy, Default, Deserialize, Serialize)]
pub enum AntiAliasingMethod {
    #[default]
    Uniform,
    MonteCarlo,
}