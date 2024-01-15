use crate::type_utils::{Comma, Parenthesis};

use super::Pattern;

pub struct TuplePatt {
    open_parenthesis: Parenthesis,
    tuple_patt_elements_opt: Option<TuplePattElements>,
    close_parenthesis: Parenthesis,
}

pub struct TuplePattElements {
    first_element: Box<dyn Pattern>,
    subsequent_elements: Vec<(Comma, Box<dyn Pattern>)>,
    trailing_comma_opt: Option<Comma>,
}
