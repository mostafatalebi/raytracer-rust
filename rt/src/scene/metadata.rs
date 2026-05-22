use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Metadata {
    title: String,
    created_at: String,
}