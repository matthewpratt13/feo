use feo_types::{
    span::{Span, Spanned},
    utils::{Comma, Parenthesis},
};

use super::Type;

#[derive(Debug, Clone)]
pub struct TupleType {
    open_parenthesis: Parenthesis,
    elements: Vec<(Type, Comma)>,
    trailing_element: Box<Type>,
    close_parenthesis: Parenthesis,
}

impl Spanned for TupleType {
    fn span(&self) -> Span {
        let start_pos = self.open_parenthesis.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.open_parenthesis.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct UnitType {
    open_parenthesis: Parenthesis,
    close_parenthesis: Parenthesis,
}

impl Spanned for UnitType {
    fn span(&self) -> Span {
        let start_pos = self.open_parenthesis.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.open_parenthesis.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
