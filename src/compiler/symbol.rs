
use super::datatype::Datatype;

#[derive(Clone)]
pub enum Scope {
    Global,
    Parameter,
    Local,
    Extern,
}

#[derive(Clone)]
pub struct Symbol<'a> {
    // name: Indexed in hash table
    pub datatype: Datatype<'a>,
    pub scope: Scope,
}
