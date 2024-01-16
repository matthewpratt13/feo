use crate::{
    expression::Attribute,
    identifier::Identifier,
    keyword::KeywordKind,
    ty::Type,
    type_utils::{Brace, Colon, Comma, Parenthesis, Semicolon},
};

use super::{VisibilityKind, WhereClause};

pub struct Struct {
    visibility_opt: Option<VisibilityKind>,
    kw_struct: KeywordKind,
    name: Identifier,
    open_brace: Brace,
    where_clause_opt: WhereClause,
    struct_fields_opt: Option<StructFields>,
    close_brace: Brace,
}

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
    semicolon: Semicolon,
}

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
