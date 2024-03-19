use feo_types::{
    span::{Span, Spanned},
    type_utils::Bracket,
    U64Primitive,
};

use super::Type;

#[derive(Debug, Clone)]
pub struct ArrayType {
    pub open_bracket: Bracket,
    pub element_type: Box<Type>,
    pub num_elements: U64Primitive,
    pub close_bracket: Bracket,
}

impl Spanned for ArrayType {
    fn span(&self) -> Span {
        let s1 = self.open_bracket.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)
    }
}
