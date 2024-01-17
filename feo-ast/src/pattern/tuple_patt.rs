use feo_types::span::{Span, Spanned};

use crate::type_utils::{Comma, Parenthesis};

use super::Pattern;

pub struct TuplePatt {
    open_parenthesis: Parenthesis,
    tuple_patt_elements_opt: Option<TuplePattElements>,
    close_parenthesis: Parenthesis,
}

impl Pattern for TuplePatt {}

impl Spanned for TuplePatt {
    fn span(&self) -> Span {
        let start_pos = self.open_parenthesis.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.open_parenthesis.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct TuplePattElements {
    first_element: Box<dyn Pattern>,
    subsequent_elements: Vec<(Comma, Box<dyn Pattern>)>,
    trailing_comma_opt: Option<Comma>,
}
