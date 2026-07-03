use std::str::FromStr;
use crate::common::file::File;
use crate::error::error::SysError;
use crate::error::kinds::ErrorKind;
use crate::geometry::geometry::Geometry;
use crate::vector::types::Vector;
use crate::vector::vec3f::Vec3f;
use crate::vector::vec3i::Vec3i;

pub struct ObjImporter {

}


impl ObjImporter {
    pub fn parse(file: &str, auto_calc_face_normals: bool) -> Result<Vec<Geometry>, SysError> {
        match File::load(file) {
            Err(e) => {
                return Err(e)
            },
            Ok(f) => {
                return Self::parse_data(&f, auto_calc_face_normals);
            }
        }
    }

    /// @todo currently if the imported file doesn't include vt (vertex texture
    /// entry in the face info (1/2/3 1/2/3 1/2/3) the parser fails. Parse must be modified
    /// so if vt is absent (1/3 1/3 1/3) it properly handles it
    /// obj files use 1-based index (and not the common 0-based)
    fn parse_data(str: &str, auto_calc_face_normals: bool) -> Result<Vec<Geometry>, SysError> {
        let mut geometries: Vec<Geometry> = vec![Geometry::default(); 0];
        let mut obj_name = String::new();
        let mut any = false;
        let mut object_index = 0;
        let mut global_vertex_ptr = 0;
        let mut global_normal_vertex_ptr = 0;
        let mut global_vn_vec: Vec<Vec3f> = vec![];
        let mut local_vn_index = 0;


        let mut face_vertices_info = Vec3i::default();
        let mut face_main_v = Vec3i::default();
        let mut face_normal_v = Vec3i::default();
        for (k, l) in str.lines().enumerate() {
            let line = l.trim();
            if line.starts_with("#") || line.is_empty() {
                continue;
            } else if line.starts_with("o") {
                // this condition is for the beginning of
                // each new object. A place to reset stuff.
                if any == true {
                    object_index += 1;
                } else {
                    any = true;
                }
                obj_name = line.trim_start_matches('o').trim().to_string();
                geometries.push(Geometry::default());
                geometries[object_index].id = obj_name;
                local_vn_index = 0;
                any = true;
            } else {
                // actual data of geometries (vertex, v normal, v texture, face)
                if line.starts_with("v ") {
                    let vertex_coordinate = Vec3f::try_from(line.trim_start_matches("v ").to_string());
                    match vertex_coordinate {
                        Ok(v) => {
                            geometries[object_index].data.vertices.push(v.clone());
                            global_vertex_ptr += 1;
                            continue
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                } else if line.starts_with("vn ") {
                    if geometries[object_index].data.vertex_normals.len() == 0 {
                        // geometries[object_index].data.vertex_normals = vec![Vec3f::default(); geometries[object_index].data.vertices.len()]
                    }
                    let vertex_coordinate = Vec3f::try_from(line.trim_start_matches("vn ").to_string());
                    match vertex_coordinate {
                        Ok(v) => {
                            geometries[object_index].data.vertex_normals.push(v.clone());
                            global_vn_vec.push(v.clone());
                            global_normal_vertex_ptr += 1;
                            local_vn_index += 1;
                            continue
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                } else if line.starts_with("vt ") {
                    // nothing to implement for vertex texture coordinate, skip
                } else if line.starts_with("f ") {
                    face_vertices_info.reset();
                    // f 4652/4929/4740               4088/4942/4182 4089/4943/4184
                    //   v_index/vt_index/vn_index    ....
                    // obj face
                    let ff = line.replace("f ", "");
                    if ff == "103/145/7 109/155/8 102/143/9" {
                        // println!("hey");
                    }
                    let face_pieces: Vec<_> = ff.trim().split(' ').collect();
                    if face_pieces.len() < 3 {
                        return Err(SysError::new_str(ErrorKind::BadFaceStructure, "bad face pieces"));
                    } else if face_pieces.len() > 3 {
                        return Err(SysError::new_str(ErrorKind::BadFaceStructure, "only triangular faces are supported (faces with exactly three vertices)"));
                    }

                    let mut face_info_local_i = 0;

                    for each_piece in face_pieces.iter() {
                        let mut j = 0;
                        for x in each_piece.trim().split('/') {
                            match i64::from_str(x) {
                                Ok(i_num) => {
                                    face_vertices_info[j] = i_num;
                                    j += 1;
                                },
                                Err(e) => {

                                }
                            }
                        };

                        if global_vertex_ptr > face_vertices_info[0] as usize -1 {
                            // why subtraction? because vertex indices in obj files are global
                            // while in our scene format, we keep them local to each geometry.
                            // so each time we insert a vertex index, we have to translate it to
                            // local index by subtracting all previous indices from it.
                            let vertex_index = (face_vertices_info[0]-1) - (global_vertex_ptr - geometries[object_index].data.vertices.len()) as i64;
                            face_main_v[face_info_local_i] = vertex_index;
                            let vn_index  = (face_vertices_info[2]-1)-(global_normal_vertex_ptr-local_vn_index);
                            face_normal_v[face_info_local_i] = vn_index;
                        } else {
                            return Err(SysError::new(ErrorKind::BadFaceStructure, format!("[face] vertex not found: {}", face_vertices_info[0])));
                        }
                        face_info_local_i += 1;

                    }

                    geometries[object_index].data.faces.push(face_main_v.clone());
                    geometries[object_index].data.face_to_v_normals.push(face_normal_v.clone());

                    // since our system uses f_normals by default, and since
                    // obj only has v_normals, we have to calculate
                    // face normals anyway
                    if auto_calc_face_normals {
                        let edge1 = geometries[object_index].data.vertices[face_main_v[1] as usize] - geometries[object_index].data.vertices[face_main_v[0] as usize];
                        let edge2 = geometries[object_index].data.vertices[face_main_v[2] as usize] - geometries[object_index].data.vertices[face_main_v[0] as usize];
                        geometries[object_index].data.face_normals.push((&edge1).cross3(&edge2).normalized());
                    }




                }
            }
        }

        Ok(geometries)
    }


}





#[cfg(test)]
mod tests {
    use crate::scene::obj_importer::ObjImporter;

    #[test]
    fn test_parse_obj_file() {
        // the following defined objects are not meaningful
        // they are only used for test assertions
        let sample_obj = "# sample test from Mostafa
        o object_1
        v -4.0 -0.2 1.0
        v -2.0 3.2 0.0
        v 4.0 0.2 0.0
        v 8.0 -9.2 1.0
        vn -1.0 2.0 3.0
        vn -4.0 5.0 6.0
        vn -7.0 8.0 9.0
        vt 0 0 0
        f 1/1/1 2/1/1 3/1/1
        f 2/1/2 3/1/2 1/1/2
        f 3/1/3 2/1/3 1/1/3
        f 4/1/1 1/1/2 3/1/3
        o object_2
        v -1.0 -2.0 3.0
        v -4.0 5.0 6.0
        v 7.0 8.0 9.0
        v -10.0 11.0 12.0
        vn -1.0 2.0 3.0
        vn -4.0 5.0 6.0
        vn -7.0 8.0 9.0
        vn -10.0 11.0 12.0
        vt 0 0 0
        f 1/1/4 2/1/4 3/1/4
        f 2/1/5 3/1/5 1/1/5
        f 3/1/6 2/1/6 1/1/6
        f 4/1/4 1/1/4 3/1/4
        ";


        let result = ObjImporter::parse_data(sample_obj, false);
        assert_eq!(true, result.is_ok());
        let obj = result.unwrap();
        assert_eq!(2, obj.len());
        assert_eq!(4, obj[0].data.vertices.len());
        assert_eq!(3, obj[0].data.vertex_normals.len());
        assert_eq!(4, obj[0].data.faces.len());

        assert_eq!(0, obj[0].data.face_to_v_normals[0][0]);
        assert_eq!(0, obj[0].data.face_to_v_normals[0][1]);
        assert_eq!(0, obj[0].data.face_to_v_normals[0][2]);
        assert_eq!(1, obj[0].data.face_to_v_normals[1][0]);
        assert_eq!(1, obj[0].data.face_to_v_normals[1][1]);
        assert_eq!(1, obj[0].data.face_to_v_normals[1][2]);
        assert_eq!(2, obj[0].data.face_to_v_normals[2][0]);
        assert_eq!(2, obj[0].data.face_to_v_normals[2][1]);
        assert_eq!(2, obj[0].data.face_to_v_normals[2][2]);
        assert_eq!(0, obj[0].data.face_to_v_normals[3][0]);
        assert_eq!(1, obj[0].data.face_to_v_normals[3][1]);
        assert_eq!(2, obj[0].data.face_to_v_normals[3][2]);

        assert_eq!(0, obj[1].data.face_to_v_normals[0][0]);
        assert_eq!(0, obj[1].data.face_to_v_normals[0][1]);
        assert_eq!(0, obj[1].data.face_to_v_normals[0][2]);
        assert_eq!(1, obj[1].data.face_to_v_normals[1][0]);
        assert_eq!(1, obj[1].data.face_to_v_normals[1][1]);
        assert_eq!(1, obj[1].data.face_to_v_normals[1][2]);
        assert_eq!(2, obj[1].data.face_to_v_normals[2][0]);
        assert_eq!(2, obj[1].data.face_to_v_normals[2][1]);
        assert_eq!(2, obj[1].data.face_to_v_normals[2][2]);
        assert_eq!(0, obj[1].data.face_to_v_normals[3][0]);
        assert_eq!(0, obj[1].data.face_to_v_normals[3][1]);
        assert_eq!(0, obj[1].data.face_to_v_normals[3][2]);


        assert_eq!(-1.0, obj[1].data.vertex_normals[0][0]);
        assert_eq!(2.0, obj[1].data.vertex_normals[0][1]);
        assert_eq!(3.0, obj[1].data.vertex_normals[0][2]);
    }
}