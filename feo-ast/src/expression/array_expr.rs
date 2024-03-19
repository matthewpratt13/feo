use feo_types::{
    span::{Span, Spanned},
    type_utils::Bracket,
    U64Primitive,
};

use super::{Value, ValueCollection};

#[derive(Debug, Clone)]
pub struct ArrayExpr {
    pub open_bracket: Bracket,
    pub elements_opt: Option<ValueCollection>,
    pub close_bracket: Bracket,
}

impl Spanned for ArrayExpr {
    fn span(&self) -> Span {
        let s1 = self.open_bracket.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct IndexExpr {
    pub indexed_operand: Box<Value>,
    pub open_bracket: Bracket,
    pub index: U64Primitive,
    pub close_bracket: Bracket,
}

impl Spanned for IndexExpr {
    fn span(&self) -> Span {
        let s1 = self.indexed_operand.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)
    }
}
