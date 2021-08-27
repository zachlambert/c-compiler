
mod content;
mod function;
mod statement;
mod expression;
mod datatype;

use super::generator;
use super::instructions;
use super::construct;
use super::resolve;

pub use content::generate_content;
pub use statement::generate_statement;
