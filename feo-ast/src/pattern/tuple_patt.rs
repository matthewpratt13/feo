use feo_types::{
    span::{Span, Spanned},
    utils::{Comma, Parenthesis},
};

use super::Pattern;

#[derive(Debug, Clone)]
pub struct TuplePatt {
    open_parenthesis: Parenthesis,
    tuple_patt_elements_opt: Option<TuplePattElements>,
    close_parenthesis: Parenthesis,
}

impl Spanned for TuplePatt {
    fn span(&self) -> Span {
        let s1 = self.open_parenthesis.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct TuplePattElements {
    first_element: Box<Pattern>,
    subsequent_elements: Option<Vec<Pattern>>,
    trailing_comma_opt: Option<Comma>,
}
