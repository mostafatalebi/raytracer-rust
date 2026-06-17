use crate::common::id::AutoId;
use crate::geometry::geometry::Geometry;
use crate::vector::types::Vec3i;
use crate::vector::vec3f::Vec3f;

pub fn create_cube(width: f64, height: f64, thickness: f64) -> Geometry {
    let mut cube = Geometry::default();
    cube.auto_id();
    cube.data.vertices.push(Vec3f::new(-width, 0.0, -thickness));
    cube.data.vertices.push(Vec3f::new(width, 0.0, -thickness));
    cube.data.vertices.push(Vec3f::new(-width, height, -thickness));
    cube.data.vertices.push(Vec3f::new(width, height, -thickness));

    cube.data.vertices.push(Vec3f::new(-width, 0.0, thickness));
    cube.data.vertices.push(Vec3f::new(width, 0.0, thickness));
    cube.data.vertices.push(Vec3f::new(-width, height, thickness));
    cube.data.vertices.push(Vec3f::new(width, height, thickness));

    cube.data.faces.push(Vec3i::new(1,0,2));
    cube.data.faces.push(Vec3i::new(3,1,2));

    cube.data.faces.push(Vec3i::new(7,6,5));
    cube.data.faces.push(Vec3i::new(5,6,4));

    cube.data.faces.push(Vec3i::new(3,5,1));
    cube.data.faces.push(Vec3i::new(3,7,5));

    cube.data.faces.push(Vec3i::new(2,0,4));
    cube.data.faces.push(Vec3i::new(6,2,4));

    cube.data.faces.push(Vec3i::new(2,7,3));
    cube.data.faces.push(Vec3i::new(6,7,2));

    cube.data.faces.push(Vec3i::new(0,1,5));
    cube.data.faces.push(Vec3i::new(4,0,5));

    cube.prepare_geometry();
    cube
}