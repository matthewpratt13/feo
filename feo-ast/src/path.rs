#![allow(dead_code)]

use feo_types::span::{Span, Spanned};

use crate::{
    expression::{Constant, ExprWithoutBlock, Expression},
    identifier::Identifier,
    keyword::Keyword,
    statement::Statement,
    ty::Type,
    type_utils::DblColon,
};

pub enum PathSegmentKind {
    Iden(Identifier),
    KwCrate(Keyword),
    KwSelf(Keyword),
    KwSuper(Keyword),
}

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
