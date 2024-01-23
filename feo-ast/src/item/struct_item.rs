use crate::{
    expression::{Constant, OuterAttr},
    identifier::Identifier,
    keyword::Keyword,
    program::{ContractItem, LibraryItem},
    span::{Span, Spanned},
    statement::Statement,
    ty::Type,
    type_utils::{Brace, Colon, Comma, Parenthesis, Semicolon},
};

use super::{Item, VisibilityKind, WhereClause};

pub trait StructItem
where
    Self: Sized + 'static + Item + LibraryItem + Type,
{
}

pub struct StructType {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_struct: Keyword,
    struct_name: Identifier,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    struct_fields_opt: Option<StructFields>,
    close_brace: Brace,
}

impl StructItem for StructType {}

impl Item for StructType {}

impl Statement for StructType {}

impl LibraryItem for StructType {}

impl Type for StructType {}

impl ContractItem for StructType {}

impl Spanned for StructType {
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
    field_name: Identifier,
    colon: Colon,
    field_type: Box<dyn Type>,
}

pub struct TupleStructType {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_struct: Keyword,
    struct_name: Identifier,
    open_parenthesis: Parenthesis,
    tuple_fields_opt: Option<TupleFields>,
    close_parenthesis: Parenthesis,
    where_clause_opt: Option<WhereClause>,
    semicolon: Semicolon,
}

impl StructItem for TupleStructType {}

impl Item for TupleStructType {}

impl Statement for TupleStructType {}

impl LibraryItem for TupleStructType {}

impl Type for TupleStructType {}

impl ContractItem for TupleStructType {}

impl Spanned for TupleStructType {
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

pub struct UnitStructType {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_struct: Keyword,
    struct_name: Identifier,
    open_brace: Brace,
    close_brace: Brace,
}

impl StructItem for UnitStructType {}

impl Item for UnitStructType {}

impl Statement for UnitStructType {}

impl LibraryItem for UnitStructType {}

impl Constant for UnitStructType {}

impl Type for UnitStructType {}

impl ContractItem for UnitStructType {}

impl Spanned for UnitStructType {
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
