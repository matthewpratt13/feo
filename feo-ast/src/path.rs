#![allow(dead_code)]

use feo_types::{
    span::{Span, Spanned},
    utils::{DblColon, KwCrate, KwSelf, KwSelfType, KwSuper},
    Identifier,
};

use crate::{
    item::Item,
    pattern::{Pattern, PatternWithoutRange, RangePattBound},
    ty::Type,
};

pub enum SimplePathSegmentKind {
    Iden(Identifier),
    KwCrate(KwCrate),
    KwSelf(KwSelf),
    KwSuper(KwSuper),
}

impl Item for SimplePathSegmentKind {}

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
    KwCrate(KwCrate),
    KwSelf(KwSelf),
    KwSelfType(KwSelfType),
    KwSuper(KwSuper),
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

// impl Expression for PathExpr {}

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
    pub dbl_colon_opt: Option<DblColon>, // TODO: remove this
    pub first_segment: SimplePathSegmentKind,
    pub subsequent_segments: Vec<(DblColon, SimplePathSegmentKind)>,
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
    pub dbl_colon_opt: Option<DblColon>, // TODO: remove this
    pub first_segment: PathExprSegment,
    pub subsequent_segments: Vec<(DblColon, PathExprSegment)>,
}

pub struct PathType {
    pub dbl_colon_opt: Option<DblColon>, // TODO: remove this
    pub first_segment: PathTypeSegment,
    pub subsequent_segments: Vec<(DblColon, PathTypeSegment)>,
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
