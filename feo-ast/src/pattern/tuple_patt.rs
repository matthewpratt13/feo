use feo_types::{
    span::{Span, Spanned},
    utils::{Comma, Parenthesis},
};

use super::Pattern;

#[derive(Debug, Clone)]
pub struct TuplePatt {
    pub open_parenthesis: Parenthesis,
    pub elements: TuplePattElements,
    pub close_parenthesis: Parenthesis,
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
    pub first_element: Box<Pattern>,
    pub subsequent_elements_opt: Option<Vec<Pattern>>,
    pub trailing_comma_opt: Option<Comma>,
}
