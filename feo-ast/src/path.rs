#![allow(dead_code)]

use feo_types::{
    span::{Span, Spanned},
    utils::DblColon,
    Identifier, Keyword,
};

use crate::{
    expression::{
        Assignable, BooleanOperand, Constant, ExprWithoutBlock, Expression, IterableExpr,
    },
    item::Item,
    pattern::{Pattern, PatternWithoutRange, RangePattBound},
    statement::Statement,
    ty::Type,
};

pub enum SimplePathSegmentKind {
    Iden(Identifier),
    KwCrate(Keyword),
    KwSelf(Keyword),
    KwSuper(Keyword),
}

impl Item for SimplePathSegmentKind {}

impl Statement for SimplePathSegmentKind {}

impl Spanned for SimplePathSegmentKind {
    fn span(&self) -> Span {
        match &self {
            SimplePathSegmentKind::Iden(i) => i.span(),
            SimplePathSegmentKind::KwCrate(c) => c.span(),
            SimplePathSegmentKind::KwSelf(se) => se.span(),
            SimplePathSegmentKind::KwSuper(su) => su.span(),
        }
    }
}

pub enum PathIdenSegmentKind {
    Iden(Identifier),
    KwCrate(Keyword),
    KwSelf(Keyword),
    KwSelfType(Keyword),
    KwSuper(Keyword),
}

impl Spanned for PathIdenSegmentKind {
    fn span(&self) -> Span {
        match &self {
            PathIdenSegmentKind::Iden(i) => i.span(),
            PathIdenSegmentKind::KwCrate(c) => c.span(),
            PathIdenSegmentKind::KwSelf(se) => se.span(),
            PathIdenSegmentKind::KwSelfType(st) => st.span(),
            PathIdenSegmentKind::KwSuper(su) => su.span(),
        }
    }
}

pub type PathExprSegment = PathIdenSegmentKind;
pub type PathTypeSegment = PathIdenSegmentKind;

pub type PathExpr = PathInExpr;

impl Expression for PathExpr {}

impl<E> ExprWithoutBlock<E> for PathExpr {}

impl Statement for PathExpr {}

impl Assignable for PathExpr {}

impl BooleanOperand for PathExpr {}

impl IterableExpr for PathExpr {}

impl Constant for PathExpr {}

impl Item for PathExpr {}

impl RangePattBound for PathExpr {}

impl Spanned for PathExpr {
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

pub type PathPatt = PathExpr;

impl Pattern for PathPatt {}

impl PatternWithoutRange for PathPatt {}

// points to either a local variable or an item
pub struct SimplePath {
    dbl_colon_opt: Option<DblColon>,
    first_segment: SimplePathSegmentKind,
    subsequent_segments: Vec<(DblColon, SimplePathSegmentKind)>,
}

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

pub struct PathInExpr {
    dbl_colon_opt: Option<DblColon>,
    first_segment: PathExprSegment,
    subsequent_segments: Vec<(DblColon, PathExprSegment)>,
}

pub struct PathType {
    dbl_colon_opt: Option<DblColon>,
    first_segment: PathTypeSegment,
    subsequent_segments: Vec<(DblColon, PathExprSegment)>,
}

impl Type for PathType {}

impl Spanned for PathType {
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
