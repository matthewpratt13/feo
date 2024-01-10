use crate::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
};

use super::Expression;

pub struct ArrayExpr {
    open_bracket: (DelimKind, DelimOrientation),
    elements_opt: Option<Box<ArrayElements>>,
    close_bracket: (DelimKind, DelimOrientation),
}

pub struct ArrayElements {
    first_element: Expression,
    subsequent_elements: Vec<(PuncKind, Expression)>,
    trailing_comma_opt: Option<PuncKind>,
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
