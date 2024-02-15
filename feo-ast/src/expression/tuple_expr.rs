use feo_types::{
    literal::UIntType,
    span::{Span, Spanned},
    utils::{Comma, Dot, Parenthesis},
    Literal,
};

use crate::{item::TupleStructDef, ty::TupleType};

use super::Expression;

#[derive(Clone)]
pub enum TupleKind {
    Tuple(TupleType),
    TupleStruct(TupleStructDef),
}

impl Spanned for TupleKind {
    fn span(&self) -> Span {
        match self {
            TupleKind::Tuple(t) => t.span(),
            TupleKind::TupleStruct(ts) => ts.span(),
        }
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct TupleElements {
    initializer_operands: Vec<(Expression, Comma)>, // TODO: limit to a specific kind of `Expression`
    trailing_operand_opt: Option<Box<Expression>>,
}

#[derive(Clone)]
pub struct TupleIndexExpr {
    operand: TupleKind,
    dot: Dot,
    index: Literal<UIntType>,
}

impl Spanned for TupleIndexExpr {
    fn span(&self) -> Span {
        let start_pos = self.operand.span().start();
        let end_pos = self.dot.span().end() + format!("{:?}", self.index).len();
        let source = self.operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
