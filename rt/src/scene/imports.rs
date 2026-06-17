use serde::Deserialize;

#[derive(Deserialize)]
pub enum SceneImportTypes {
    Obj,
}

#[derive(Deserialize)]
pub struct GeometriesJsonImport {
    #[serde(rename = "type")]
    pub imp_type: SceneImportTypes,
    pub path: String,
}