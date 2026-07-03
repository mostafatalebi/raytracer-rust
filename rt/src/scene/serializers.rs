use image::DynamicImage;
use serde::{Serialize, Serializer};


fn serialize_image<S>(
    image: &Option<DynamicImage>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match image {
        Some(i) => serializer.serialize_str("nothing"),
        None => serializer.serialize_none(),
    }
}