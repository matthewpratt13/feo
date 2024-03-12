use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, Parenthesis},
    Identifier,
};

use crate::attribute::OuterAttr;

use super::Returnable;

#[derive(Debug, Clone)]
pub enum StructExprKind {
    Struct(StructExpr),
    TupleStruct(TupleStructExpr),
}

impl Spanned for StructExprKind {
    fn span(&self) -> Span {
        match self {
            StructExprKind::Struct(s) => s.span(),
            StructExprKind::TupleStruct(ts) => ts.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructExpr {
    pub id: Identifier,
    pub open_brace: Brace,
    pub fields_opt: Option<StructExprFields>,
    pub close_brace: Brace,
}

impl Spanned for StructExpr {
    fn span(&self) -> Span {
        let s1 = self.id.span();
        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct StructExprFields {
    pub first_field: StructExprField,
    pub subsequent_fields_opt: Option<Vec<StructExprField>>,
}

#[derive(Debug, Clone)]
pub struct StructExprField {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub field_content: (Identifier, Box<Returnable>),
}

#[derive(Debug, Clone)]
pub struct TupleStructExpr {
    pub id: Identifier,
    pub open_parenthesis: Parenthesis,
    pub fields_opt: Option<TupleStructExprFields>,
    pub close_parenthesis: Parenthesis,
}

impl Spanned for TupleStructExpr {
    fn span(&self) -> Span {
        let s1 = self.id.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct TupleStructExprFields {
    pub first_field: Box<Returnable>,
    pub subsequent_fields_opt: Option<Vec<Returnable>>,
}
