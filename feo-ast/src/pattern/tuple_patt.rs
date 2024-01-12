use crate::item::{Comma, Parenthesis};

use super::Pattern;

pub struct TuplePatt {
    open_parenthesis: Parenthesis,
    tuple_patt_elements_opt: Option<TuplePattElements>,
    close_parenthesis: Parenthesis,
}

pub struct TuplePattElements {
    first_element: Box<Pattern>,
    subsequent_elements: Vec<(Comma, Pattern)>,
    trailing_comma_opt: Option<Comma>,
}
