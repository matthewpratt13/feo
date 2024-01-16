use crate::type_utils::{Comma, Dot, Parenthesis};

use super::{ExprWithoutBlock, Expression};

pub struct TupleExpr {
    open_parenthesis: Parenthesis,
    elements_opt: Option<TupleElements>,
    close_parenthesis: Parenthesis,
}

impl Expression for TupleExpr {}

impl<E> ExprWithoutBlock<E> for TupleExpr where E: Expression {}

pub struct TupleElements {
    elements: Vec<(Box<dyn Expression>, Comma)>,
    trailing_element_opt: Option<Box<dyn Expression>>,
}

pub struct TupleIndexingExpr {
    object: Box<dyn Expression>,
    dot: Dot,
    index: usize,
}

impl Expression for TupleIndexingExpr {}

impl<E> ExprWithoutBlock<E> for TupleIndexingExpr where E: Expression {}
