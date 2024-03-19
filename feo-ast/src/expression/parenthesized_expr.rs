use feo_types::{
    span::{Span, Spanned},
    type_utils::Parenthesis,
};

use super::Expression;

#[derive(Debug, Clone)]
pub struct ParenthesizedExpr {
    pub open_parenthesis: Parenthesis,
    pub enclosed_operand: Box<Expression>,
    pub close_parenthesis: Parenthesis,
}

impl Spanned for ParenthesizedExpr {
    fn span(&self) -> Span {
        let s1 = self.open_parenthesis.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}
