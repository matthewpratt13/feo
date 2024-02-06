use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, Colon, Comma, KwStruct, Parenthesis, Semicolon},
    Identifier,
};

use crate::{expression::OuterAttr, ty::Type};

use super::{Item, VisibilityKind, WhereClause};

pub trait StructItem
where
    Self: Sized + Item + 'static,
{
}

pub type StructFieldName = Identifier;

pub struct Struct {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_struct: KwStruct,
    struct_name: Identifier,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    struct_fields_opt: Option<StructFields>,
    close_brace: Brace,
}

impl StructItem for Struct {}

impl Item for Struct {}

impl Spanned for Struct {
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

pub struct StructFields {
    first_field: StructField,
    subsequent_fields: Vec<(Comma, StructField)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct StructField {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    field_name: StructFieldName,
    colon: Colon,
    field_type: Box<dyn Type>,
}

pub struct TupleStruct {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_struct: KwStruct,
    struct_name: Identifier,
    open_parenthesis: Parenthesis,
    tuple_elements_opt: Option<TupleElements>,
    close_parenthesis: Parenthesis,
    where_clause_opt: Option<WhereClause>,
    semicolon: Semicolon,
}

impl StructItem for TupleStruct {}

impl Item for TupleStruct {}

impl Spanned for TupleStruct {
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

pub struct TupleElements {
    first_field: TupleField,
    subsequent_fields: Vec<(Comma, TupleField)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct TupleField {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    field_type: Box<dyn Type>,
}

pub struct UnitStruct {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_struct: KwStruct,
    struct_name: Identifier,
    open_brace: Brace,
    close_brace: Brace,
}

impl StructItem for UnitStruct {}

impl Item for UnitStruct {}

impl Spanned for UnitStruct {
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
