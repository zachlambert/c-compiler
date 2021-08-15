
use crate::lexer::token::Primitive;

pub enum Qualifier {
    Const,
    Static,
    Extern,
}

pub struct Pointer<'a> {
    datatype: Datatype<'a>
}

pub struct Struct<'a> {
    members: Vec<(String, Datatype<'a>)>,
}

const MAX_QUALIFIERS: usize = 3;
pub struct Qualified<'a> {
    qualifiers: [Qualifier; MAX_QUALIFIERS],
    num_qualifiers: usize,
    datatype: Datatype<'a>
}

pub struct Function<'a> {
    return_datatype: Datatype<'a>,
    arg_datatypes: Vec<Datatype<'a>>,
}
impl<'a> Function<'a> {
    pub fn new(datatype: Datatype<'a>) -> Function<'a> {
        Function {
            return_datatype: datatype,
            arg_datatypes: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub enum Datatype<'a> {
    Primitive(Primitive),
    Pointer(&'a Pointer<'a>),
    Struct(&'a Struct<'a>),
    Qualified(&'a Qualified<'a>),
    FunctionPointer(&'a Function<'a>),
}
