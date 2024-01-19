use feo_types::span::{Span, Spanned};

use crate::{
    expression::OuterAttr,
    identifier::Identifier,
    keyword::Keyword,
    program::{ContractItem, LibraryItem},
    statement::Statement,
    ty::Type,
    type_utils::{Brace, Colon, Comma, Parenthesis, Semicolon},
};

use super::{Item, VisibilityKind, WhereClause};

pub trait StructItem
where
    Self: Item + LibraryItem + Type,
{
}

pub struct StructDef {
    visibility_opt: Option<VisibilityKind>,
    kw_struct: Keyword,
    identifier: Identifier,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    struct_fields_opt: Option<StructFields>,
    close_brace: Brace,
}

impl ContractItem for StructDef {}

impl Item for StructDef {}

impl LibraryItem for StructDef {}

impl Statement for StructDef {}

impl StructItem for StructDef {}

impl Type for StructDef {}

impl Spanned for StructDef {
    fn span(&self) -> Span {
        let start_pos = if let Some(v) = &self.visibility_opt {
            v.span().start()
        } else {
            self.kw_struct.span().start()
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
    field_name: Identifier,
    colon: Colon,
    field_type: Box<dyn Type>,
}

pub struct TupleStructDef {
    visibility_opt: Option<VisibilityKind>,
    kw_struct: Keyword,
    identifier: Identifier,
    open_parenthesis: Parenthesis,
    tuple_fields_opt: Option<TupleFields>,
    close_parenthesis: Parenthesis,
    where_clause_opt: Option<WhereClause>,
    semicolon: Semicolon,
}

impl ContractItem for TupleStructDef {}

impl Item for TupleStructDef {}

impl LibraryItem for TupleStructDef {}

impl Statement for TupleStructDef {}

impl StructItem for TupleStructDef {}

impl Type for TupleStructDef {}

impl Spanned for TupleStructDef {
    fn span(&self) -> Span {
        let start_pos = if let Some(v) = &self.visibility_opt {
            v.span().start()
        } else {
            self.kw_struct.span().start()
        };

        let end_pos = self.semicolon.span().end();
        let source = self.kw_struct.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct TupleFields {
    first_field: TupleField,
    subsequent_fields: Vec<(Comma, TupleField)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct TupleField {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    field_type: Box<dyn Type>,
}
