use crate::error::kinds::ErrorKind;

#[derive(Debug, Clone)]
pub struct SysError {
    kind: ErrorKind,
    msg: String,
}

impl SysError {
    pub fn new(kind: ErrorKind, msg: String) -> Self {
        SysError {kind, msg: msg}
    }

    pub fn new_str(kind: ErrorKind, msg: &str) -> Self {
        SysError {kind, msg: msg.to_string()}
    }

    pub fn to_string(&self) -> String {
        return format!("kind={} msg={}", String::from(self.kind.clone()), self.msg);
    }
}