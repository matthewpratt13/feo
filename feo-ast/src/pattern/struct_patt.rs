use feo_types::span::{Span, Spanned};

use crate::{
    expression::OuterAttr,
    identifier::Identifier,
    path::SimplePath,
    type_utils::{Brace, Colon, Comma, Parenthesis},
};

use super::Pattern;

pub struct StructPatt {
    object_path: SimplePath,
    open_brace: Brace,
    fields_opt: Option<StructPattFields>,
    close_brace: Brace,
}

impl Pattern for StructPatt {}

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
    struct_pattern_kind: Identifier,
    colon: Colon,
    data: Box<dyn Pattern>,
}

pub struct TupleStructPatt {
    object_path: SimplePath,
    open_parenthesis: Parenthesis,
    fields_opt: Option<TupleStructItems>,
    close_parenthesis: Parenthesis,
}

impl Pattern for TupleStructPatt {}

impl Spanned for TupleStructPatt {
    fn span(&self) -> Span {
        let start_pos = self.object_path.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.object_path.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct TupleStructItems {
    first_item: Box<dyn Pattern>,
    subsequent_fields: Vec<(Comma, Box<dyn Pattern>)>,
    trailing_comma_opt: Option<Comma>,
}
