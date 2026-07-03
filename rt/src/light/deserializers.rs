use serde::{de, Deserialize, Deserializer};
use crate::light::area_light::AreaLight;

pub fn deserialize_area_light<'de, D>(deserializer: D) -> Result<AreaLight, D::Error>
where
    D: Deserializer<'de>,
{
    let res = AreaLight::deserialize(deserializer);

    match res {
        Ok(mut light) => {
            if light.flip_normals {
                light.flip();
            }
            Ok(light)
        },
        Err(e) => {
            return Err(de::Error::custom(e))
        }
    }

}
