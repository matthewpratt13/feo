use crate::item::{Bracket, Comma, Semicolon};

use super::Expression;

pub struct ArrayExpr {
    open_bracket: Bracket,
    elements_opt: Option<ArrayElements>,
    close_bracket: Bracket,
}

pub struct ArrayElements {
    first_element: Box<Expression>,
    subsequent_elements: Vec<(Comma, Expression)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct ArrayDefinition {
    element_type: Box<Expression>,
    semicolon: Semicolon,
    num_elements: usize,
}

pub struct IndexExpr {
    object: Box<Expression>,
    open_bracket: Bracket,
    index: usize,
    close_bracket: Bracket,
}
