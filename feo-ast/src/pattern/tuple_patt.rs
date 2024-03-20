use feo_types::{
    span::{Span, Spanned},
    type_utils::Parenthesis,
};

use crate::expression::TermCollection;

use super::Pattern;

#[derive(Debug, Clone)]
pub struct TuplePatt {
    pub open_parenthesis: Parenthesis,
    pub elements: TermCollection<TuplePattElement>,
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
pub struct TuplePattElement(pub Box<Pattern>);
