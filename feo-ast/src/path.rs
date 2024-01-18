#![allow(dead_code)]

use feo_types::span::{Span, Spanned};

use crate::{
    expression::{Constant, ExprWithoutBlock, Expression},
    identifier::Identifier,
    keyword::Keyword,
    pattern::Pattern,
    statement::Statement,
    ty::Type,
    type_utils::DblColon,
};

pub enum PathSegmentKind {
    Identifier(Identifier),
    CrateKeyword(Keyword),
    SelfKeyword(Keyword),
    SuperKeyword(Keyword),
}

impl Spanned for PathSegmentKind {
    fn span(&self) -> Span {
        match &self {
            PathSegmentKind::Identifier(i) => i.span(),
            PathSegmentKind::CrateKeyword(c) => c.span(),
            PathSegmentKind::SelfKeyword(se) => se.span(),
            PathSegmentKind::SuperKeyword(su) => su.span(),
        }
    }
}

pub struct SimplePath {
    dbl_colon_opt: Option<DblColon>,
    first_segment: PathSegmentKind,
    subsequent_segments: Vec<(DblColon, PathSegmentKind)>,
}

impl Constant for SimplePath {}

impl Expression for SimplePath {}

impl<E> ExprWithoutBlock<E> for SimplePath {}

impl Pattern for SimplePath {}

impl Statement for SimplePath {}

impl Type for SimplePath {}

impl Spanned for SimplePath {
    fn span(&self) -> Span {
        let start_pos = match &self.dbl_colon_opt {
            Some(d) => d.span().start(),
            None => self.first_segment.span().start(),
        };

        let end_pos = match self.subsequent_segments.last() {
            Some(s) => s.1.span().end(),
            None => self.first_segment.span().end(),
        };

        let source = self.first_segment.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
