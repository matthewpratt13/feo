use crate::type_utils::{Comma, Dot, Parenthesis};

use super::Expression;

pub struct TupleExpr {
    open_parenthesis: Parenthesis,
    elements_opt: Option<TupleElements>,
    close_parenthesis: Parenthesis,
}

pub struct TupleElements {
    elements: Vec<(Box<dyn Expression>, Comma)>,
    trailing_element_opt: Option<Box<dyn Expression>>,
}

pub struct TupleIndexingExpr {
    object: Box<dyn Expression>,
    dot: Dot,
    index: usize,
}
