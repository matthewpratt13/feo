use feo_types::{
    primitive::Primitive,
    span::{Span, Spanned},
    utils::{Bracket, Semicolon},
};

use super::Type;

pub struct ArrayType {
    open_bracket: Bracket,
    element_type: Box<dyn Type>,
    semicolon: Semicolon,
    num_elements: Primitive<u64>,
    close_bracket: Bracket,
}

impl Type for ArrayType {}

impl Spanned for ArrayType {
    fn span(&self) -> Span {
        let start_pos = self.open_bracket.span().start();
        let end_pos = self.close_bracket.span().end();
        let source = self.open_bracket.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
