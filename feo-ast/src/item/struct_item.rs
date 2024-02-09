use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, Colon, Comma, KwStruct, Parenthesis, Semicolon},
    Identifier,
};

use crate::{expression::OuterAttr, ty::Type};

use super::{VisibilityKind, WhereClause};

// pub trait StructItem
// where
//     Self: Sized + Item + 'static,
// {
// }

#[derive(Clone)]
pub enum StructDefKind {
    Struct(StructDef),
    TupleStruct(TupleStructDef),
    UnitStruct(UnitStructDef),
}

pub type StructFieldName = Identifier;

#[derive(Clone)]
pub struct StructDef {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_struct: KwStruct,
    struct_name: Identifier,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    struct_fields_opt: Option<StructDefFields>,
    close_brace: Brace,
}

impl Spanned for StructDef {
    fn span(&self) -> Span {
        let start_pos = match self.attributes.first() {
            Some(a) => a.span().start(),
            None => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => self.kw_struct.span().start(),
            },
        };

        let end_pos = self.close_brace.span().end();
        let source = self.kw_struct.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Clone)]
pub struct StructDefFields {
    first_field: StructDefField,
    subsequent_fields: Vec<(Comma, StructDefField)>,
    trailing_comma_opt: Option<Comma>,
}

#[derive(Clone)]
pub struct StructDefField {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    field_name: StructFieldName,
    colon: Colon,
    field_type: Box<Type>,
}

#[derive(Clone)]
pub struct TupleStructDef {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_struct: KwStruct,
    struct_name: Identifier,
    open_parenthesis: Parenthesis,
    tuple_elements_opt: Option<TupleStructDefElements>,
    close_parenthesis: Parenthesis,
    where_clause_opt: Option<WhereClause>,
    semicolon: Semicolon,
}

impl Spanned for TupleStructDef {
    fn span(&self) -> Span {
        let start_pos = match self.attributes.first() {
            Some(a) => a.span().start(),
            None => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => self.kw_struct.span().start(),
            },
        };

        let end_pos = self.semicolon.span().end();
        let source = self.kw_struct.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Clone)]
pub struct TupleStructDefElements {
    first_field: TupleStructDefField,
    subsequent_fields: Vec<(Comma, TupleStructDefField)>,
    trailing_comma_opt: Option<Comma>,
}

#[derive(Clone)]
pub struct TupleStructDefField {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    field_type: Box<Type>,
}

#[derive(Clone)]
pub struct UnitStructDef {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_struct: KwStruct,
    struct_name: Identifier,
    open_brace: Brace,
    close_brace: Brace,
}

impl Spanned for UnitStructDef {
    fn span(&self) -> Span {
        let start_pos = match self.attributes.first() {
            Some(a) => a.span().start(),
            None => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => self.kw_struct.span().start(),
            },
        };

        let end_pos = self.close_brace.span().end();
        let source = self.kw_struct.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
