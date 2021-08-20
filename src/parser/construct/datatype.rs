
use std::fmt;

#[derive(Clone)]
pub enum Primitive {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    C8,
}

impl fmt::Display for Primitive {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primitive::I8 => write!(fmt, "Primitive(i8)"),
            Primitive::I16 => write!(fmt, "Primitive(i16)"),
            Primitive::I32 => write!(fmt, "Primitive(i32)"),
            Primitive::I64 => write!(fmt, "Primitive(i64)"),
            Primitive::U8 => write!(fmt, "Primitive(u8)"),
            Primitive::U16 => write!(fmt, "Primitive(u16)"),
            Primitive::U32 => write!(fmt, "Primitive(u32)"),
            Primitive::U64 => write!(fmt, "Primitive(u64)"),
            Primitive::F32 => write!(fmt, "Primitive(f32)"),
            Primitive::F64 => write!(fmt, "Primitive(f64)"),
            Primitive::C8 => write!(fmt, "Primitive(c8)"),
        }
    }
}

#[derive(Clone)]
pub enum Datatype {
    Terminal,
    // { qualifier } , ( primitive | identifier | struct )

    Pointer,
    // { qualifier } , datatype
}

impl fmt::Display for Datatype {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Datatype::Terminal => write!(fmt, "Datatype(Terminal)"),
            Datatype::Pointer => write!(fmt, "Datatype(Pointer)"),
        }
    }
}

#[derive(Clone)]
pub enum Qualifier {
    Mut,
}

impl fmt::Display for Qualifier {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Qualifier::Mut => write!(fmt, "Qualifier(mut)"),
        }
    }
}


