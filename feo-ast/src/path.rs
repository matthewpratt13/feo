#![allow(dead_code)]

use feo_types::{
    span::{Span, Spanned},
    utils::{DblColon, KwCrate, KwSelf, KwSelfType, KwSuper},
    Identifier,
};

#[derive(Clone)]
pub enum SimplePathSegmentKind {
    Iden(Identifier),
    KwCrate(KwCrate),
    KwSelf(KwSelf),
    KwSuper(KwSuper),
}

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

#[derive(Clone)]
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

impl Spanned for PathExpr {
    fn span(&self) -> Span {
        let start_pos = self.first_segment.span().start();

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

// points to either a local variable or an item
#[derive(Clone)]
pub struct SimplePath {
    pub first_segment: SimplePathSegmentKind,
    pub subsequent_segments: Vec<(DblColon, SimplePathSegmentKind)>,
}

impl Spanned for SimplePath {
    fn span(&self) -> Span {
        let start_pos = self.first_segment.span().start();

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

#[derive(Clone)]
pub struct PathInExpr {
    pub first_segment: PathExprSegment,
    pub subsequent_segments: Vec<(DblColon, PathExprSegment)>,
}

#[derive(Clone)]
pub struct PathType {
    pub first_segment: PathTypeSegment,
    pub subsequent_segments: Vec<(DblColon, PathTypeSegment)>,
}

impl Spanned for PathType {
    fn span(&self) -> Span {
        let start_pos = self.first_segment.span().start();

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
