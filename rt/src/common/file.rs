use crate::error::error::SysError;
use crate::error::kinds::ErrorKind;

pub struct File {

}


impl File {


    pub fn load(file_addr: &str) -> Result<String, SysError> {
        let file_content = std::fs::read_to_string(file_addr);

        if file_content.is_err() {
            return Err(SysError::new(ErrorKind::FileLoadFailed, file_content.err().unwrap().to_string()));
        }

        Ok(file_content.unwrap())
    }
}