use std::os::unix::raw::gid_t;
use serde::{de, Deserialize, Deserializer};
use serde::de::Error;
use crate::geometry::geometry::Geometry;
use crate::scene::imports::GeometriesJsonImport;
use crate::scene::imports::SceneImportTypes::Obj;
use crate::scene::obj_importer::ObjImporter;

pub fn deserialize_geometries<'de, D>(deserializer: D) -> Result<Vec<Geometry>, D::Error>
where
    D: Deserializer<'de>,
{
    let res = GeometriesJsonImport::deserialize(deserializer);

    match res {
        Ok(gij) => {
            match gij.imp_type {
                Obj => {
                    let res = ObjImporter::parse(&gij.path, true);
                    if res.is_ok() {
                        return Ok(res.unwrap());
                    }
                    return Err(de::Error::custom(res.err().unwrap().to_string()))
                },
                _ => {
                    panic!("import type not supported");
                }
            }
        },
        Err(e) => {
            panic!("import error: {}", e);
        }
    }

    Ok(vec![])
}