use crate::{
    expression::OuterAttr,
    identifier::Identifier,
    path::PathInExpr,
    span::{Span, Spanned},
    type_utils::{Brace, Colon, Comma, Parenthesis},
};

use super::{Pattern, PatternWithoutRange};

pub struct StructPatt {
    object_path: PathInExpr,
    open_brace: Brace,
    fields_opt: Option<StructPattFields>,
    close_brace: Brace,
}

impl Pattern for StructPatt {}

impl PatternWithoutRange for StructPatt {}

impl Spanned for StructPatt {
    fn span(&self) -> Span {
        let start_pos = self.object_path.span().start();
        let end_pos = self.close_brace.span().end();
        let source = self.object_path.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct StructPattFields {
    first_field: StructPattField,
    subsequent_fields: Vec<(Comma, StructPattField)>,
}

pub struct StructPattField {
    attributes: Vec<OuterAttr>,
    field_name: Identifier,
    colon: Colon,
    data: Box<dyn Pattern>,
}

pub struct TupleStructPatt {
    object_path: PathInExpr,
    open_parenthesis: Parenthesis,
    elements_opt: Option<TupleStructElements>,
    close_parenthesis: Parenthesis,
}

impl Pattern for TupleStructPatt {}

impl PatternWithoutRange for TupleStructPatt {}

impl Spanned for TupleStructPatt {
    fn span(&self) -> Span {
        let start_pos = self.object_path.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.object_path.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct TupleStructElements {
    first_item: Box<dyn Pattern>,
    subsequent_fields: Vec<(Comma, Box<dyn Pattern>)>,
    trailing_comma_opt: Option<Comma>,
}
