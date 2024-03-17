use feo_types::{
    span::{Span, Spanned},
    utils::Parenthesis,
    U64Primitive,
};

use super::Value;

#[derive(Debug, Clone)]
pub struct TupleExpr {
    pub open_parenthesis: Parenthesis,
    pub elements: TupleExprElements,
    pub close_parenthesis: Parenthesis,
}

impl Spanned for TupleExpr {
    fn span(&self) -> Span {
        let s1 = self.open_parenthesis.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct TupleExprElements {
    pub first_element: Box<Value>,
    pub subsequent_elements_opt: Option<Vec<Value>>,
}

#[derive(Debug, Clone)]
pub struct TupleIndexExpr {
    operand: Box<Value>,
    index: U64Primitive,
}

impl Spanned for TupleIndexExpr {
    fn span(&self) -> Span {
        let s1 = self.operand.span();
        let s2 = self.index.span();

        Span::join(s1, s2)
    }
}
