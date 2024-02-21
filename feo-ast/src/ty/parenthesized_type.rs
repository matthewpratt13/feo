use feo_types::{
    span::{Span, Spanned},
    utils::Parenthesis,
};

use super::Type;

#[derive(Debug, Clone)]
pub struct ParenthesizedType {
    open_parenthesis: Parenthesis,
    ty: Box<Type>,
    close_parenthesis: Parenthesis,
}

impl Spanned for ParenthesizedType {
    fn span(&self) -> Span {
        let start_pos = self.open_parenthesis.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.open_parenthesis.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
