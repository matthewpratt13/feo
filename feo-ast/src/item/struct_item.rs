use feo_types::span::{Span, Spanned};

use crate::{
    expression::OuterAttr,
    identifier::Identifier,
    keyword::Keyword,
    ty::Type,
    type_utils::{Brace, Colon, Comma, Parenthesis, Semicolon},
};

use super::{Item, StructItem, VisibilityKind, WhereClause};

pub struct Struct {
    visibility_opt: Option<VisibilityKind>,
    kw_struct: Keyword,
    name: Identifier,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    struct_fields_opt: Option<StructFields>,
    close_brace: Brace,
}

impl Item for Struct {}

impl<S> StructItem<S> for Struct where S: Item {}

impl Type for Struct {}

impl Spanned for Struct {
    fn span(&self) -> Span {
        let start_pos = if let Some(v) = &self.visibility_opt {
            v.span().start()
        } else {
            self.kw_struct.span().start()
        };

        let end_pos = self.close_brace.span().end();
        let source = self.name.span().source();

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

pub struct TupleStruct {
    visibility_opt: Option<VisibilityKind>,
    kw_struct: Keyword,
    name: Identifier,
    open_parenthesis: Parenthesis,
    tuple_fields_opt: Option<TupleFields>,
    close_parenthesis: Parenthesis,
    where_clause_opt: Option<WhereClause>,
    semicolon: Semicolon,
}

impl Item for TupleStruct {}

impl<S> StructItem<S> for TupleStruct where S: Item {}

impl Type for TupleStruct {}

impl Spanned for TupleStruct {
    fn span(&self) -> Span {
        let start_pos = if let Some(v) = &self.visibility_opt {
            v.span().start()
        } else {
            self.kw_struct.span().start()
        };

        let end_pos = self.semicolon.span().end();
        let source = self.name.span().source();

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
