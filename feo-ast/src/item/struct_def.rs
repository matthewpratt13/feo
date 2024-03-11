use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, Comma, KwStruct, Parenthesis, Semicolon},
    Identifier,
};

use crate::{attribute::OuterAttr, ty::Type};

use super::{VisibilityKind, WhereClause};

#[derive(Debug, Clone)]
pub enum StructDefKind {
    Struct(StructDef),
    TupleStruct(TupleStructDef),
}

pub type StructFieldName = Identifier;

#[derive(Debug, Clone)]
pub struct StructDef {
    pub attributes: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub kw_struct: KwStruct,
    pub struct_name: Identifier,
    pub where_clause_opt: Option<WhereClause>,
    pub open_brace: Brace,
    pub fields_opt: Option<StructDefFields>,
    pub close_brace: Brace,
}

impl Spanned for StructDef {
    fn span(&self) -> Span {
        let s1 = match &self.attributes {
            Some(a) => match a.first() {
                Some(oa) => oa.span(),
                None => match &self.visibility_opt {
                    Some(v) => v.span(),
                    None => self.kw_struct.span(),
                },
            },
            None => self.kw_struct.span(),
        };

        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct StructDefFields {
    pub first_field: StructDefField,
    pub subsequent_fields: Option<Vec<StructDefField>>,
    pub trailing_comma_opt: Option<Comma>,
}

#[derive(Debug, Clone)]

pub struct StructDefField(
    pub Option<Vec<OuterAttr>>,
    pub Option<VisibilityKind>,
    pub (Identifier, Box<Type>),
);

#[derive(Debug, Clone)]
pub struct TupleStructDef {
    pub attributes: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub kw_struct: KwStruct,
    pub struct_name: Identifier,
    pub open_parenthesis: Parenthesis,
    pub fields_opt: Option<TupleStructDefFields>,
    pub close_parenthesis: Parenthesis,
    pub where_clause_opt: Option<WhereClause>,
    pub semicolon: Semicolon,
}

impl Spanned for TupleStructDef {
    fn span(&self) -> Span {
        let s1 = match &self.attributes {
            Some(a) => match a.first() {
                Some(oa) => oa.span(),
                None => match &self.visibility_opt {
                    Some(v) => v.span(),
                    None => self.kw_struct.span(),
                },
            },
            None => self.kw_struct.span(),
        };

        let s2 = self.semicolon.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct TupleStructDefFields {
    pub first_field: TupleStructDefField,
    pub subsequent_fields: Option<Vec<TupleStructDefField>>,
    pub trailing_comma_opt: Option<Comma>,
}

#[derive(Debug, Clone)]
pub struct TupleStructDefField(
    pub Option<Vec<OuterAttr>>,
    pub Option<VisibilityKind>,
    pub Box<Type>,
);
