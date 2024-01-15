use crate::type_utils::{Comma, Parenthesis};

use super::PatternKind;

pub struct TuplePatt {
    open_parenthesis: Parenthesis,
    tuple_patt_elements_opt: Option<TuplePattElements>,
    close_parenthesis: Parenthesis,
}

pub struct TuplePattElements {
    first_element: Box<PatternKind>,
    subsequent_elements: Vec<(Comma, PatternKind)>,
    trailing_comma_opt: Option<Comma>,
}
