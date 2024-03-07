use feo_types::{
    primitive::Primitive,
    span::{Span, Spanned},
    utils::{Bracket, Semicolon},
};

use super::Type;

#[derive(Debug, Clone)]
pub struct ArrayType {
    open_bracket: Bracket,
    element_type: Box<Type>,
    semicolon: Semicolon,
    num_elements: Primitive<u64>,
    close_bracket: Bracket,
}

impl Spanned for ArrayType {
    fn span(&self) -> Span {
        let s1 = self.open_bracket.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)
    }
}
