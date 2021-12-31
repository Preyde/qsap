use crate::net::Destination;

use super::SendWith;

pub trait Create {
    fn create(&self) -> Box<dyn SendWith>;
}

pub trait CopyTo {
    fn copy_to(&self, target_name: &str) -> Box<dyn SendWith>;
}

pub trait Delete {
    fn delete(&mut self) -> Box<dyn SendWith>;
}

pub trait CopyToSys {
    fn copy_to_sys<'a>(&'a self, dest: &Destination) -> Box<dyn SendWith + 'a>;
}

pub trait Source {
    fn source(&self) -> Box<dyn SendWith>;
    fn update_source(&self, source: &str) -> Box<dyn SendWith>;
    fn get_source(&self) -> String;
}
