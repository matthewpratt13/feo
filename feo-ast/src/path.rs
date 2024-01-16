#![allow(dead_code)]

use crate::{
    expression::{ExprWithoutBlock, Expression},
    identifier::Identifier,
    keyword::KeywordKind,
    pattern::Pattern,
    ty::Type,
    type_utils::DblColon,
};

pub enum PathSegmentKind {
    Identifier(Identifier),
    CrateKeyword(KeywordKind),
    SelfKeyword(KeywordKind),
    SuperKeyword(KeywordKind),
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
