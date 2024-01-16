#![allow(dead_code)]

use crate::{identifier::Identifier, keyword::KeywordKind, type_utils::DblColon, expression::{Expression, ExprWithoutBlock}};

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
