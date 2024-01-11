use crate::{
    delimiter::{DelimKind, DelimOrientation},
    item::{Comma, Semicolon},
};

use super::Expression;

pub struct ArrayExpr {
    open_bracket: (DelimKind, DelimOrientation),
    elements_opt: Option<ArrayElements>,
    close_bracket: (DelimKind, DelimOrientation),
}

pub struct ArrayElements {
    first_element: Box<Expression>,
    subsequent_elements: Vec<(Comma, Expression)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct ArrayElementsFixedSize {
    element_type: Box<Expression>,
    semicolon: Semicolon,
    num_elements: usize,
}

pub struct IndexExpr {
    item: Box<Expression>,
    open_bracket: (DelimKind, DelimOrientation),
    index: usize,
    close_bracket: (DelimKind, DelimOrientation),
}
