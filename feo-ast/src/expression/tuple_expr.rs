use crate::type_utils::{Comma, Dot, Parenthesis};

use super::ExpressionKind;

pub struct TupleExpr {
    open_parenthesis: Parenthesis,
    elements_opt: Option<TupleElements>,
    close_parenthesis: Parenthesis,
}

pub struct TupleElements {
    elements: Vec<(ExpressionKind, Comma)>,
    trailing_element_opt: Option<Box<ExpressionKind>>,
}

pub struct TupleIndexingExpr {
    object: Box<ExpressionKind>,
    dot: Dot,
    index: usize,
}
