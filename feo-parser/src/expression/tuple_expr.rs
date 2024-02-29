use feo_ast::expression::{TupleElements, TupleExpr, TupleIndexExpr};
use feo_error::error::CompilerError;

use crate::{
    parse::{ParseExpr, ParseTerm},
    parser::Parser,
};

// for reference:
// pub struct TupleElements {
//     pub first_element: Box<Returnable>,
//     pub subsequent_elements_opt: Option<Vec<(Comma, Returnable)>>,
//     pub trailing_comma_opt: Option<Comma>,
// }

impl ParseTerm for TupleElements {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for TupleExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for TupleIndexExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
