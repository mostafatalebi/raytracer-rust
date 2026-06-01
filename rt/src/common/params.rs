use std::collections::HashMap;
use crate::vector::vec3f::Vec3f;
use crate::vector::vec4f::Vec4f;

#[derive(Default, Debug, Clone)]
pub struct Value {
    pub v_i64: i64,
    pub v_f64: f64,
    pub v_str: String,
    pub v_bool: bool,
    pub v_vec: Option<Vec<Value>>,
    pub v_vec3f: Option<Vec3f>,
    pub v_vec4f: Option<Vec4f>,
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

    pub fn set(&mut self, k: String, v: Value) -> &mut Self {
        self.values.insert(k, v);

        self
    }

    pub fn get(&self, k: &str) -> Option<&Value> {
        self.values.get(k)
    }
}

impl Value {
    pub fn from_f64(f: f64) -> Value {
        let mut v = Value::default();
        v.v_f64 = f;

        v
    }
    pub fn from_vec3f(val: Vec3f) -> Value {
        let mut v = Value::default();
        v.v_vec3f = Some(val);

        v
    }
    pub fn from_str(val: String) -> Value {
        let mut v = Value::default();
        v.v_str = val;

        v
    }
    pub fn from_i64(i: i64) -> Value {
        let mut v = Value::default();
        v.v_i64 = i;

        v
    }
    pub fn from_bool(b: bool) -> Value {
        let mut v = Value::default();
        v.v_bool = b;

        v
    }
}