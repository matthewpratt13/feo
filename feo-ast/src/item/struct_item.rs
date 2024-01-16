use crate::{
    expression::Attribute,
    identifier::Identifier,
    keyword::KeywordKind,
    ty::Type,
    type_utils::{Brace, Colon, Comma, Parenthesis, Semicolon},
};

use super::{Item, StructItem, VisibilityKind, WhereClause};

pub struct Struct {
    visibility_opt: Option<VisibilityKind>,
    kw_struct: KeywordKind,
    name: Identifier,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    struct_fields_opt: Option<StructFields>,
    close_brace: Brace,
}

impl Item for Struct {}

impl<S> StructItem<S> for Struct where S: Item {}

impl Type for Struct {}

pub struct StructFields {
    first_field: StructField,
    subsequent_fields: Vec<(Comma, StructField)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct StructField {
    attributes: Vec<Attribute>,
    visibility_opt: Option<VisibilityKind>,
    field_name: Identifier,
    colon: Colon,
    field_type: Box<dyn Type>,
}

pub struct TupleStruct {
    visibility_opt: Option<VisibilityKind>,
    kw_struct: KeywordKind,
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

pub struct TupleFields {
    first_field: TupleField,
    subsequent_fields: Vec<(Comma, TupleField)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct TupleField {
    attributes: Vec<Attribute>,
    visibility_opt: Option<VisibilityKind>,
    field_type: Box<dyn Type>,
}
