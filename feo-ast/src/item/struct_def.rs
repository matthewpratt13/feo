use feo_types::{
    span::{Span, Spanned},
    type_utils::{Brace, KwStruct, Parenthesis, Semicolon},
    Identifier,
};

use crate::{attribute::OuterAttr, expression::TermCollection, ty::Type};

use super::VisibilityKind;

#[derive(Debug, Clone)]
pub enum StructDefKind {
    Struct(StructDef),
    TupleStruct(TupleStructDef),
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub kw_struct: KwStruct,
    pub struct_name: Identifier,
    pub open_brace: Brace,
    pub fields_opt: Option<TermCollection<StructDefField>>,
    pub close_brace: Brace,
}

impl Spanned for StructDef {
    fn span(&self) -> Span {
        let s1 = match &self.attributes_opt {
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

pub struct StructDefField {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub field_type: (Identifier, Box<Type>),
}

#[derive(Debug, Clone)]
pub struct TupleStructDef {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub kw_struct: KwStruct,
    pub struct_name: Identifier,
    pub open_parenthesis: Parenthesis,
    pub fields_opt: Option<TermCollection<TupleStructDefField>>,
    pub close_parenthesis: Parenthesis,
    pub semicolon: Semicolon,
}

impl Spanned for TupleStructDef {
    fn span(&self) -> Span {
        let s1 = match &self.attributes_opt {
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

// #[derive(Debug, Clone)]
// pub struct TupleStructDefFields {
//     pub first_field: TupleStructDefField,
//     pub subsequent_fields_opt: Option<Vec<TupleStructDefField>>,
// }

#[derive(Debug, Clone)]
pub struct TupleStructDefField {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub field_type: Box<Type>,
}
