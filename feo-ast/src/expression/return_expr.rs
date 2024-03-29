use feo_types::{
    span::{Span, Spanned},
    type_utils::KwReturn,
};

use super::Expression;

#[derive(Debug, Clone)]
pub struct ReturnExpr {
    pub kw_return: KwReturn,
    pub expression_opt: Option<Box<Expression>>,
}

impl Spanned for ReturnExpr {
    fn span(&self) -> Span {
        let s1 = self.kw_return.span();
        let s2 = if let Some(e) = &self.expression_opt {
            e.span()
        } else {
            self.kw_return.span()
        };

        Span::join(s1, s2)
    }
}
