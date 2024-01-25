#![allow(dead_code)]

use crate::{
    expression::{BooleanOperand, Constant, ExprWithoutBlock, Expression, IterableExpr},
    identifier::Identifier,
    item::Item,
    keyword::Keyword,
    pattern::{Pattern, PatternWithoutRange, RangePattBound},
    span::{Span, Spanned},
    statement::Statement,
    ty::{Type, TypeWithoutBounds},
    type_utils::DblColon,
};

pub enum PathSegmentKind {
    Iden(Identifier),
    KwCrate(Keyword),
    KwSelf(Keyword),
    KwSuper(Keyword),
}

impl Item for PathSegmentKind {}

impl Statement for PathSegmentKind {}

impl Spanned for PathSegmentKind {
    fn span(&self) -> Span {
        match &self {
            PathSegmentKind::Iden(i) => i.span(),
            PathSegmentKind::KwCrate(c) => c.span(),
            PathSegmentKind::KwSelf(se) => se.span(),
            PathSegmentKind::KwSuper(su) => su.span(),
        }
    }
}

// points to either a local variable or an item
pub struct SimplePath {
    dbl_colon_opt: Option<DblColon>,
    first_segment: PathSegmentKind,
    subsequent_segments: Vec<(DblColon, PathSegmentKind)>,
}

impl Expression for SimplePath {}

impl<E> ExprWithoutBlock<E> for SimplePath {}

impl Statement for SimplePath {}

impl Constant for SimplePath {}

impl BooleanOperand for SimplePath {}

impl IterableExpr for SimplePath {}

impl Item for SimplePath {}

impl Pattern for SimplePath {}

impl PatternWithoutRange for SimplePath {}

impl RangePattBound for SimplePath {}

impl TypeWithoutBounds for SimplePath {}

impl Type for SimplePath {}

impl Spanned for SimplePath {
    fn span(&self) -> Span {
        let start_pos = if let Some(d) = &self.dbl_colon_opt {
            d.span().start()
        } else {
            self.first_segment.span().start()
        };

        let end_pos = if let Some(s) = self.subsequent_segments.last() {
            s.1.span().end()
        } else {
            self.first_segment.span().end()
        };

        let source = self.first_segment.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
