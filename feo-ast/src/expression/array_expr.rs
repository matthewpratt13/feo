use crate::type_utils::{Bracket, Comma, Semicolon};

use super::ExpressionKind;

pub struct ArrayExpr {
    open_bracket: Bracket,
    elements_opt: Option<ArrayElements>,
    close_bracket: Bracket,
}

pub struct ArrayElements {
    first_element: Box<ExpressionKind>,
    subsequent_elements: Vec<(Comma, ExpressionKind)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct ArrayDefinition {
    element_type: Box<ExpressionKind>,
    semicolon: Semicolon,
    num_elements: usize,
}

pub struct IndexExpr {
    object: Box<ExpressionKind>,
    open_bracket: Bracket,
    index: usize,
    close_bracket: Bracket,
}
