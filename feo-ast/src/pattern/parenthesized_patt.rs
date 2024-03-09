use feo_types::{
    span::{Span, Spanned},
    utils::Parenthesis,
};

use super::Pattern;

#[derive(Debug, Clone)]
pub struct ParenthesizedPatt {
    open_parenthesis: Parenthesis,
    pattern: Box<Pattern>,
    close_parenthesis: Parenthesis,
}

impl Spanned for ParenthesizedPatt {
    fn span(&self) -> Span {
        let s1 = self.open_parenthesis.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}
