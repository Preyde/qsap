use crate::{
    data::abap_table::{Dd02v, Dd09l, DD03P_TAB},
    net::Destination,
};

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

pub trait Details {
    fn details(&self) -> Box<dyn SendWith>;
    fn update_details(&self, dd02v: &Dd02v, dd09l: &Dd09l, dd03p: &DD03P_TAB) -> Box<dyn SendWith>;
}
