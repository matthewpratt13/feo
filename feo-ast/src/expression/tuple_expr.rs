use feo_types::{
    literal::UIntType,
    span::{Span, Spanned},
    utils::{Comma, FullStop, Parenthesis},
    Literal,
};

use crate::ty::TupleType;

use super::{Returnable, TupleStructExpr};

#[derive(Debug, Clone)]
pub enum TupleKind {
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
    open_parenthesis: Parenthesis,
    elements_opt: Option<TupleElements>,
    close_parenthesis: Parenthesis,
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
    pub subsequent_elements_opt: Option<Vec<(Comma, Returnable)>>,
    pub trailing_comma_opt: Option<Comma>,
}

#[derive(Debug, Clone)]
pub struct TupleIndexExpr {
    operand: TupleKind,
    full_stop: FullStop,
    index: Literal<UIntType>,
}

impl Spanned for TupleIndexExpr {
    fn span(&self) -> Span {
        let start_pos = self.operand.span().start();
        let end_pos = self.full_stop.span().end() + format!("{:?}", self.index).len();
        let source = self.operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
