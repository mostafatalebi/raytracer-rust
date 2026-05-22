use std::collections::HashMap;
use crate::common::obj_types::ObjType;

pub struct IdGen {
    by_object_types: HashMap<ObjType, String>
}