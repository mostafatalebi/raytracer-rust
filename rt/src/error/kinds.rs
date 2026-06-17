use crate::error::kinds::ErrorKind::Generic;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    FileLoadFailed,
    InvalidData,
    InvalidVersion,
    BadSceneDescription,
    InvalidGeometryType,
    InvalidLightType,
    InvalidMaterialType,
    InvalidCameraType,
    NoGeometry,
    NoLight,
    NoCamera,
    MaterialNotFound,
    CameraNotFound,
    LightNotFound,
    GeometryNotFound,
    Generic,
    NonTriangularFace,
    BadFaceStructure,
    FailedToSaveImage,
    FaceIdNotFound,
    ObjectWithoutShader,
    SurfaceNormalNotFound,
    GeometryTypeUndefined,
    Unparsable,
}


impl From<&str> for ErrorKind {
    fn from(s: &str) -> ErrorKind {
        match s {
            "FileLoadFailed" =>            ErrorKind::FileLoadFailed,
            "InvalidData" =>            ErrorKind::InvalidData,
            "InvalidVersion" =>         ErrorKind::InvalidVersion,
            "InvalidGeometryType" =>    ErrorKind::InvalidGeometryType,
            "InvalidLightType" =>       ErrorKind::InvalidLightType,
            "InvalidMaterialType" =>    ErrorKind::InvalidMaterialType,
            "InvalidCameraType" =>      ErrorKind::InvalidCameraType,
            "BadSceneDescription" =>    ErrorKind::BadSceneDescription,
            "NoGeometry" =>             ErrorKind::NoGeometry,
            "NoLight" =>                ErrorKind::NoLight,
            "NoCamera" =>               ErrorKind::NoCamera,
            "generic" =>                ErrorKind::Generic,
            "NonTriangularFace" =>      ErrorKind::NonTriangularFace,
            "BadFaceStructure" =>       ErrorKind::BadFaceStructure,
            "FailedToSaveImage" =>      ErrorKind::FailedToSaveImage,
            "FaceIdNotFound" =>         ErrorKind::FaceIdNotFound,
            "ObjectWithoutShader" =>         ErrorKind::ObjectWithoutShader,
            "SurfaceNormalNotFound" =>         ErrorKind::SurfaceNormalNotFound,
            "Unparsable" =>         ErrorKind::Unparsable,
            &_ => Generic,
        }
    }
}

impl From<ErrorKind> for String {
    fn from(kind: ErrorKind) -> String {
        match kind {
            ErrorKind::FileLoadFailed =>           String::from("FileLoadFailed"),
            ErrorKind::InvalidData =>           String::from("InvalidData"),
            ErrorKind::InvalidVersion =>        String::from("InvalidVersion"),
            ErrorKind::InvalidGeometryType =>   String::from("InvalidGeometryType"),
            ErrorKind::InvalidLightType =>      String::from("InvalidLightType"),
            ErrorKind::InvalidMaterialType =>   String::from("InvalidMaterialType"),
            ErrorKind::InvalidCameraType =>     String::from("InvalidCameraType"),
            ErrorKind::BadSceneDescription =>   String::from("BadSceneDescription"),
            ErrorKind::NoGeometry =>            String::from("NoGeometry"),
            ErrorKind::NoLight =>               String::from("NoLight"),
            ErrorKind::NoCamera =>              String::from("NoCamera"),
            ErrorKind::Generic =>               String::from("generic"),
            ErrorKind::MaterialNotFound =>      String::from("MaterialNotFound"),
            ErrorKind::CameraNotFound =>        String::from("CameraNotFound"),
            ErrorKind::LightNotFound =>         String::from("LightNotFound"),
            ErrorKind::GeometryNotFound =>      String::from("GeometryNotFound"),
            ErrorKind::NonTriangularFace =>     String::from("NonTriangularFace"),
            ErrorKind::BadFaceStructure =>      String::from("BadFaceStructure"),
            ErrorKind::FailedToSaveImage =>     String::from("FailedToSaveImage"),
            ErrorKind::FaceIdNotFound =>        String::from("FaceIdNotFound"),
            ErrorKind::ObjectWithoutShader =>        String::from("ObjectWithoutShader"),
            ErrorKind::SurfaceNormalNotFound =>        String::from("SurfaceNormalNotFound"),
            ErrorKind::GeometryTypeUndefined => String::from("GeometryTypeUndefined"),
            ErrorKind::Unparsable => String::from("Unparsable"),
        }
    }
}