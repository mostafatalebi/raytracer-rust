use serde::Deserialize;

#[derive(Deserialize)]
pub enum SceneImportTypes {
    #[serde(rename = "obj")]
    Obj,
}

#[derive(Deserialize)]
pub struct GeometriesJsonImport {
    #[serde(rename = "type")]
    pub imp_type: SceneImportTypes,
    pub path: String,
}