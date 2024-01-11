use crate::{
    delimiter::{DelimKind, DelimOrientation},
    item::{Comma, Dot},
};

use super::Expression;

pub struct TupleExpr {
    open_parenthesis: (DelimKind, DelimOrientation),
    elements_opt: Option<TupleElements>,
    close_parenthesis: (DelimKind, DelimOrientation),
}

pub struct TupleElements {
    elements: Vec<(Expression, Comma)>,
    trailing_element_opt: Option<Box<Expression>>,
}

pub struct TupleIndexingExpr {
    item: Box<Expression>,
    dot: Dot,
    index: usize,
}
