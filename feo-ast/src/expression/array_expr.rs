use crate::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
};

use super::Expression;

pub struct ArrayExpr {
    open_bracket: (DelimKind, DelimOrientation),
    array_elements: Box<ArrayElements>,
    close_bracket: (DelimKind, DelimOrientation),
}

pub struct ArrayElements {
    opening_expression: Expression,
    elements: Vec<(PuncKind, Expression)>,
    trailing_comma: Option<PuncKind>,
}

pub struct ArrayElementsFixedSize {
    element_type: Expression,
    semicolon: PuncKind,
    num_elements: usize,
}

pub struct IndexExpr {
    item: Box<Expression>,
    open_bracket: PuncKind,
    index: usize,
    close_bracket: PuncKind,
}
