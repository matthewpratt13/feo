use crate::item::{Bracket, Comma};

use super::Pattern;

pub struct SlicePatt {
    open_bracket: Bracket,
    slice_pattern_elements_opt: Option<SlicePattElements>,
    close_bracket: Bracket,
}

pub struct SlicePattElements {
    first_pattern: Box<Pattern>,
    subsequent_patterns: Vec<(Comma, Pattern)>,
    trailing_comma_opt: Option<Comma>,
}
