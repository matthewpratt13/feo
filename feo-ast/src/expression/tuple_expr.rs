use feo_types::{
    span::{Span, Spanned},
    type_utils::Parenthesis,
    U64Primitive,
};

use super::{Value, ValueCollection};

#[derive(Debug, Clone)]
pub struct TupleExpr {
    pub open_parenthesis: Parenthesis,
    pub elements: ValueCollection,
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
pub struct TupleIndexExpr {
    pub operand: Box<Value>,
    pub index: U64Primitive,
}

impl Spanned for TupleIndexExpr {
    fn span(&self) -> Span {
        let s1 = self.operand.span();
        let s2 = self.index.span();

        Span::join(s1, s2)
    }
}
