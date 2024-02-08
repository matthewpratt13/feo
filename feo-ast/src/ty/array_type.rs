use feo_types::{
    primitive::Primitive,
    span::{Span, Spanned},
    utils::{Bracket, Semicolon},
};

use super::Type;

#[derive(Clone)]
pub struct ArrayType {
    open_bracket: Bracket,
    element_type: Box<Type>,
    semicolon: Semicolon,
    num_elements: Primitive,
    close_bracket: Bracket,
}

impl Spanned for ArrayType {
    fn span(&self) -> Span {
        let start_pos = self.open_bracket.span().start();
        let end_pos = self.close_bracket.span().end();
        let source = self.open_bracket.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
