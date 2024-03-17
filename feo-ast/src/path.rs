#![allow(dead_code)]

use feo_types::{
    span::{Span, Spanned},
    utils::{KwCrate, KwSelf, KwSelfType, KwSuper},
    Identifier,
};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
        let s1 = self.first_segment.span();

        let s2 = match &self.subsequent_segments {
            Some(p) => match p.last() {
                Some(s) => s.span(),
                None => self.first_segment.span(),
            },
            None => self.first_segment.span(),
        };

        Span::join(s1, s2)
    }
}

pub type PathPatt = PathExpr;

// points to either a local variable or an item
#[derive(Debug, Clone)]
pub struct SimplePath {
    pub first_segment: SimplePathSegmentKind,
    pub subsequent_segments: Option<Vec<SimplePathSegmentKind>>,
}

impl Spanned for SimplePath {
    fn span(&self) -> Span {
        let s1 = self.first_segment.span();

        let s2 = match &self.subsequent_segments {
            Some(p) => match p.last() {
                Some(s) => s.span(),
                None => self.first_segment.span(),
            },
            None => self.first_segment.span(),
        };

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct PathInExpr {
    pub first_segment: PathExprSegment,
    pub subsequent_segments: Option<Vec<PathExprSegment>>,
}

#[derive(Debug, Clone)]
pub struct PathType {
    pub first_segment: PathTypeSegment,
    pub subsequent_segments: Option<Vec<PathTypeSegment>>,
}

impl Spanned for PathType {
    fn span(&self) -> Span {
        let s1 = self.first_segment.span();

        let s2 = match &self.subsequent_segments {
            Some(p) => match p.last() {
                Some(s) => s.span(),
                None => self.first_segment.span(),
            },
            None => self.first_segment.span(),
        };

        Span::join(s1, s2)
    }
}
