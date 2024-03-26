use feo_types::{
    span::{Span, Spanned},
    type_utils::{KwPackage, KwSelf, KwSelfType, KwSuper},
    Identifier,
};

/// Element within a `SimplePath`
#[derive(Debug, Clone)]
pub enum SimplePathSegmentKind {
    Identifier(Identifier),
    KwPackage(KwPackage),
    KwSelf(KwSelf),
    KwSuper(KwSuper),
}

impl Spanned for SimplePathSegmentKind {
    fn span(&self) -> Span {
        match &self {
            SimplePathSegmentKind::Identifier(i) => i.span(),
            SimplePathSegmentKind::KwPackage(c) => c.span(),
            SimplePathSegmentKind::KwSelf(se) => se.span(),
            SimplePathSegmentKind::KwSuper(su) => su.span(),
        }
    }
}

/// Element within a `PathInExpr`
#[derive(Debug, Clone)]
pub enum PathIdenSegmentKind {
    Identifier(Identifier),
    KwPackage(KwPackage),
    KwSelf(KwSelf),
    KwSelfType(KwSelfType),
    KwSuper(KwSuper),
}

impl Spanned for PathIdenSegmentKind {
    fn span(&self) -> Span {
        match &self {
            PathIdenSegmentKind::Identifier(i) => i.span(),
            PathIdenSegmentKind::KwPackage(c) => c.span(),
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


/// Points to either a local variable or an `Item`.
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

/// Points to an `Identifier` or `Item`.
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
