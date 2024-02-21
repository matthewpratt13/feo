use feo_error::{error::CompilerError, handler::ErrorEmitted};

use crate::parser::{Parser, Peeker};

pub trait ParseExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized;
}

// literals, attributes, paths, parenthesized expressions, helper types (e.g., `StructExprField`)
pub trait ParseTerm {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized;
}

pub trait Peek {
    fn peek(peeker: &Peeker<'_, '_>) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized;
}
