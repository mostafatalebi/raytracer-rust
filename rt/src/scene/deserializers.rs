use std::f64;
use std::os::unix::raw::gid_t;
use image::{DynamicImage, ImageResult};
use serde::{de, Deserialize, Deserializer};
use serde::de::Error;
use crate::geometry::geometry::Geometry;
use crate::scene::environment::Environment;
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

pub fn deserialize_environment<'de, D>(deserializer: D) -> Result<Option<Environment>, D::Error>
where
    D: Deserializer<'de>,
{
    let res = Environment::deserialize(deserializer);

    match res {
        Ok(env) => {
            return Ok(Some(env));
        },
        Err(e) => {
            panic!("import error: {}", e);
        }
    }

}

pub fn deserialize_image<'de, D>(deserializer: D) -> Result<Option<DynamicImage>, D::Error>
where
    D: Deserializer<'de>,
{
    let res = String::deserialize(deserializer);

    match res {
        Ok(image_path) => {
            let mut image = image::open(image_path);
            match image {
                Ok(i) => {
                    return Ok(Some(i));
                }
                Err(e) => {
                    return Err(de::Error::custom(e))
                }
            }
        },
        Err(e) => {
            return Err(de::Error::custom(e))
        }
    }

}

pub fn deserialize_degree_to_radian<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let res = f64::deserialize(deserializer);

    match res {
        Ok(degree) => {
           return Ok(degree.to_radians());
        },
        Err(e) => {
            return Err(de::Error::custom(e))
        }
    }

}
