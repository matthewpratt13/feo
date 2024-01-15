use crate::type_utils::{Bracket, Comma, Semicolon};

use super::Expression;

pub struct ArrayExpr {
    open_bracket: Bracket,
    elements_opt: Option<ArrayElements>,
    close_bracket: Bracket,
}

pub struct ArrayElements {
    first_element: Box<dyn Expression>,
    subsequent_elements: Vec<(Comma, Box<dyn Expression>)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct ArrayDefinition {
    element_type: Box<dyn Expression>,
    semicolon: Semicolon,
    num_elements: usize,
}

pub struct IndexExpr {
    object: Box<dyn Expression>,
    open_bracket: Bracket,
    index: usize,
    close_bracket: Bracket,
}
