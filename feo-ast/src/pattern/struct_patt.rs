use feo_types::{
    span::{Span, Spanned},
    type_utils::{Brace, Parenthesis},
    Identifier,
};

use crate::{attribute::OuterAttr, expression::TermCollection};

use super::Pattern;

#[derive(Debug, Clone)]
pub struct StructPatt {
    pub id: Identifier,
    pub open_brace: Brace,
    pub fields_opt: Option<TermCollection<StructPattField>>,
    pub close_brace: Brace,
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
    pub first_field: StructPattField,
    pub subsequent_fields_opt: Option<Vec<StructPattField>>,
}

#[derive(Debug, Clone)]
pub struct StructPattField {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub field_content: (Identifier, Box<Pattern>),
}

#[derive(Debug, Clone)]
pub struct TupleStructPatt {
    pub id: Identifier,
    pub open_parenthesis: Parenthesis,
    pub fields_opt: Option<TupleStructPattFields>,
    pub close_parenthesis: Parenthesis,
}

impl Spanned for TupleStructPatt {
    fn span(&self) -> Span {
        let s1 = self.id.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}
#[derive(Debug, Clone)]
pub struct TupleStructPattFields {
    pub first_field: Box<Pattern>,
    pub subsequent_fields_opt: Option<Vec<Pattern>>,
}
