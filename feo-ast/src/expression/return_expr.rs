use feo_types::{
    span::{Span, Spanned},
    utils::KwReturn,
};

use super::Expression;

#[derive(Clone)]
pub struct ReturnExpr {
    kw_return: KwReturn,
    expression_opt: Option<Box<Expression>>,
}

impl Spanned for ReturnExpr {
    fn span(&self) -> Span {
        let start_pos = self.kw_return.span().start();
        let end_pos = if let Some(e) = &self.expression_opt {
            e.span().end()
        } else {
            self.kw_return.span().end()
        };

        let source = self.kw_return.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
