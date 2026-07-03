use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Copy, Default, Deserialize, Serialize)]
pub enum AntiAliasingMethod {
    #[default]
    #[serde(rename = "uniform")]
    Uniform,
    #[serde(rename="monte_carlo")]
    MonteCarlo,
}