use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, Colon, Comma, Parenthesis},
    Identifier,
};

use crate::attribute::OuterAttr;

use super::Pattern;

#[derive(Debug, Clone)]
pub struct StructPatt {
    id: Identifier,
    open_brace: Brace,
    fields_opt: Option<StructPattFields>,
    close_brace: Brace,
}

impl Spanned for StructPatt {
    fn span(&self) -> Span {
        let s1 = self.id.span();
        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct StructPattFields {
    first_field: StructPattField,
    subsequent_fields: Option<Vec<(Comma, StructPattField)>>,
}

#[derive(Debug, Clone)]
pub struct StructPattField(
    pub Option<Vec<OuterAttr>>,
    pub (Identifier, Colon, Box<Pattern>),
);

#[derive(Debug, Clone)]
pub struct TupleStructPatt {
    id: Identifier,
    open_parenthesis: Parenthesis,
    elements_opt: Option<TupleStructPattFields>,
    close_parenthesis: Parenthesis,
}

impl Spanned for TupleStructPatt {
    fn span(&self) -> Span {
        let s1 = self.id.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}
#[derive(Debug, Clone)]
pub struct TupleStructPattFields((Box<Pattern>, Option<Vec<(Comma, Pattern)>>, Option<Comma>));
