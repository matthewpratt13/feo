use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, Colon, Comma, Parenthesis},
    Identifier,
};

use crate::{attribute::OuterAttr, path::PathInExpr};

use super::Pattern;

#[derive(Clone)]
pub struct StructPatt {
    object_path: PathInExpr,
    open_brace: Brace,
    fields_opt: Option<StructPattFields>,
    close_brace: Brace,
}

impl Spanned for StructPatt {
    fn span(&self) -> Span {
        let start_pos = self.object_path.span().start();
        let end_pos = self.close_brace.span().end();
        let source = self.object_path.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Clone)]
pub struct StructPattFields {
    first_field: StructPattField,
    subsequent_fields: Vec<(Comma, StructPattField)>,
}

#[derive(Clone)]
pub struct StructPattField {
    attributes: Vec<OuterAttr>,
    field_name: Identifier,
    colon: Colon,
    data: Box<Pattern>,
}

#[derive(Clone)]
pub struct TupleStructPatt {
    object_path: PathInExpr,
    open_parenthesis: Parenthesis,
    elements_opt: Option<TupleStructElements>,
    close_parenthesis: Parenthesis,
}

impl Spanned for TupleStructPatt {
    fn span(&self) -> Span {
        let start_pos = self.object_path.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.object_path.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Clone)]
pub struct TupleStructElements {
    first_item: Box<Pattern>,
    subsequent_fields: Vec<(Comma, Pattern)>,
    trailing_comma_opt: Option<Comma>,
}
