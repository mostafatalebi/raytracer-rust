use std::collections::HashMap;
use crate::vector::vec3f::Vec3f;
use crate::vector::vec4f::Vec4f;

#[derive(Default, Debug, Clone)]
pub struct Value {
    num_int: i64,
    num_float: f64,
    str: String,
    bool: bool,
    vec: Vec<Value>,
    v_vec3f: Option<Vec3f>,
    v_vec4f: Option<Vec4f>,
}

#[derive(Default, Debug, Clone)]
pub struct Params {
    pub values: HashMap<String, Value>,
}


impl Params {
    pub fn new() -> Params {
        Params {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, k: String, v: Value) {
        self.values.insert(k, v);
    }

    pub fn get(&mut self, k: String) -> Option<&Value> {
        self.values.get(&k)
    }
}