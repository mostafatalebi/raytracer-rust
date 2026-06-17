

pub trait Id {
    fn get_id(&self) -> String;
}


pub trait AutoId {
    fn auto_id(&mut self);
}