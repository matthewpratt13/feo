use crate::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
};

use super::Expression;

pub struct TupleExpr {
    open_parenthesis: (DelimKind, DelimOrientation),
    elements_opt: Option<TupleElements>,
    close_parenthesis: (DelimKind, DelimOrientation),
}

pub struct TupleElements {
    elements: Vec<(Expression, PuncKind)>,
    trailing_element_opt: Option<Box<Expression>>,
}

pub struct TupleIndexingExpr {
    item: Box<Expression>,
    dot: PuncKind,
    index: usize,
}
