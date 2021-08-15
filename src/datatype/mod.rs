
use std::fmt;
use bitflags::bitflags;

#[derive(Clone)]
pub enum Primitive {
    Int,
    Float,
    Double,
    Char,
    Void,
}

impl fmt::Display for Primitive {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primitive::Int => write!(fmt, "Primitive(int)"),
            Primitive::Float => write!(fmt, "Primitive(float)"),
            Primitive::Double => write!(fmt, "Primitive(double)"),
            Primitive::Char => write!(fmt, "Primitive(char)"),
            Primitive::Void => write!(fmt, "Primitive(void)"),
        }
    }
}

pub struct Struct {
    return_type: Datatype,
    argument_types: Vec<Datatype>,
}

#[macro_use]
bitflags! {
    struct Flags: u8 {
        const CONST = 1<<1;
        const STATIC = 1<<2;
        const EXTERN = 1<<3;
        const MUTABLE = 1<<4;
    }
}

pub enum Compound {
    Pointer(Datatype),
    Struct(Struct),
}

pub enum DatatypeEnum {
    Unresolved(String),
    Primitive(Primitive),
    Compound(usize), // Index to compount type
}

pub struct Datatype {
    qualifiers: u8,
    datatype: DatatypeEnum,
}
