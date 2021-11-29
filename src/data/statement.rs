trait Statement {}

pub struct ValueStatement {}
/// References an ABAP table or struc with its type name
pub enum AbapType {
    Table(String),
    Struc(String),
}
// pub struct InternalTable
impl ValueStatement {
    // pub fn new(t: AbapType) -> Self {
    //     match t {
    //         Table(s) => (),
    //         Struc(s) => ()
    //     }
    // }
    // fn build_from_table(t: &str) -> Self {

    // }
}
