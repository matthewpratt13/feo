use feo_types::{
    literal::UIntType,
    span::{Span, Spanned},
    utils::{Comma, Parenthesis},
    Literal,
};

use crate::ty::TupleType;

use super::{Returnable, TupleStructExpr};

#[derive(Debug, Clone)]
enum TupleKind {
    Tuple(TupleType),
    TupleStruct(TupleStructExpr),
}

impl Spanned for TupleKind {
    fn span(&self) -> Span {
        match self {
            TupleKind::Tuple(t) => t.span(),
            TupleKind::TupleStruct(ts) => ts.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TupleExpr {
    pub open_parenthesis: Parenthesis,
    pub elements: TupleElements,
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
pub struct TupleElements {
    pub first_element: Box<Returnable>,
    pub subsequent_elements_opt: Option<Vec<Returnable>>,
    pub trailing_comma_opt: Option<Comma>,
}

#[derive(Debug, Clone)]
pub struct TupleIndexExpr {
    operand: TupleKind,
    index: Literal<UIntType>,
}

impl Spanned for TupleIndexExpr {
    fn span(&self) -> Span {
        let s1 = self.operand.span();
        let s2 = self.index.span();

        Span::join(s1, s2)
    }
}
