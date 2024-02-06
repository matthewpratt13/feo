#![allow(dead_code)]

mod struct_expr;

use feo_ast::expression::Expression;
use feo_error::parser_error::ParserError;

use crate::{parse::Parse, parser::Parser};

impl Parse for Expression {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ParserError>
    where
        Self: Sized,
    {
        todo!()
    }
}
