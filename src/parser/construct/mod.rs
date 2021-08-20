
use std::fmt;

mod datatype;
pub use datatype::*;
mod operation;
pub use operation::*;
mod statement;
pub use statement::*;
mod expression;
pub use expression::*;


#[derive(Clone)]
pub enum Construct {
    Program,
    // { function | struct }

    // === General symbols ===
    // Anything an identifier can refer to, in general.

    Function(String), // name
    // { argument } , { returned } , block

    Structure(String), // name
    // { member }

    Variable(String), // name
    // datatype

    // === Restricted symbols ===
    // Symbols that are created in specific situations.

    Argument(String), // name
    // datatype
    // Variable created in function argument list

    Returned(String), // name
    // datatype
    // Variable created in function return list. May be referred to implicitly.

    Member(String), // name
    // datatype
    // Variable created with a declare statement, within a struct

    // === Block and statements ===

    Block,
    // { statement }

    Statement(Statement), // Statement type
    // ( Statement::Declare | ... )

    // === Datatype and expressions ===

    Datatype(Datatype),
    // ( Datatype::Terminal | ... )

    Qualifier(Qualifier),
    // Terminal

    Expression(Expression),
    // ( Expression::Function | ... )

    Primitive(Primitive),
    // Terminal

    Identifier(String),
    // Terminal
}

impl fmt::Display for Construct {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Construct::Program => write!(fmt, "Program"),
            Construct::Function(name) => write!(fmt, "Function({})", name),
            Construct::Structure(name) => write!(fmt, "Structure({})", name),
            Construct::Variable(name) => write!(fmt, "Variable({})", name),

            Construct::Argument(name) => write!(fmt, "Argument({})", name),
            Construct::Returned(name) => write!(fmt, "Returned({})", name),
            Construct::Member(name) => write!(fmt, "Member({})", name),

            Construct::Block => write!(fmt, "Block"),
            Construct::Statement(statement) => write!(fmt, "{}", statement),
            Construct::Datatype(datatype) => write!(fmt, "{}", datatype),
            Construct::Qualifier(qualifier) => write!(fmt, "{}", qualifier),
            Construct::Expression(expression) => write!(fmt, "{}", expression),
            Construct::Primitive(primitive) => write!(fmt, "{}", primitive),
            Construct::Identifier(identifier) => write!(fmt, "Identifier({})", identifier),
        }
    }
}
