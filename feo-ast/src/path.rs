#![allow(dead_code)]

use feo_types::span::{Span, Spanned};

use crate::{
    expression::{ExprWithoutBlock, Expression},
    identifier::Identifier,
    keyword::Keyword,
    pattern::Pattern,
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
    fn span(&self) -> &Span {
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

impl Expression for SimplePath {}

impl<E> ExprWithoutBlock<E> for SimplePath where E: Expression {}

impl Pattern for SimplePath {}

impl Type for SimplePath {}
