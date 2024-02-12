use feo_types::{
    span::{Span, Spanned},
    utils::Parenthesis,
};

use super::Expression;

#[derive(Clone)]
pub struct ParenthesizedExpr {
    open_parenthesis: Parenthesis,
    enclosed_operand: Box<Expression>,
    close_parenthesis: Parenthesis,
}

impl Spanned for ParenthesizedExpr {
    fn span(&self) -> Span {
        let s1 = self.open_parenthesis.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}
