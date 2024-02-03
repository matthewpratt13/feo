use feo_types::{
    primitive::Primitive,
    span::{Span, Spanned},
    utils::{Comma, Dot, Parenthesis},
};

use crate::{item::TupleStruct, ty::TupleType};

use super::{Assignable, BooleanOperand, Constant, ExprWithoutBlock, Expression, IterableExpr};

pub enum TupleKind {
    Tuple(TupleType),
    TupleStruct(TupleStruct),
}

impl Spanned for TupleKind {
    fn span(&self) -> Span {
        match self {
            TupleKind::Tuple(t) => t.span(),
            TupleKind::TupleStruct(ts) => ts.span(),
        }
    }
}

pub struct TupleExpr {
    open_parenthesis: Parenthesis,
    elements_opt: Option<TupleElements>,
    close_parenthesis: Parenthesis,
}

// impl Expression for TupleExpr {}

impl<E> ExprWithoutBlock<E> for TupleExpr {}

impl Assignable for TupleExpr {}

impl BooleanOperand for TupleExpr {}

impl IterableExpr for TupleExpr {}

impl Constant for TupleExpr {}

impl Spanned for TupleExpr {
    fn span(&self) -> Span {
        let start_pos = self.open_parenthesis.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.open_parenthesis.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct TupleElements {
    initializer_operands: Vec<(Expression, Comma)>,
    trailing_operand_opt: Option<Expression>,
}

pub struct TupleIndexExpr {
    operand: TupleKind,
    dot: Dot,
    index: Primitive<u64>,
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

// impl Expression for TupleIndexExpr {}

impl<E> ExprWithoutBlock<E> for TupleIndexExpr {}

impl BooleanOperand for TupleIndexExpr {}
