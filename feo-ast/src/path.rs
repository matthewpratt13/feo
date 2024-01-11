#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{identifier::Identifier, item::DblColon, keyword::KeywordKind};

pub enum PathSegment {
    IdentifierSeg(Identifier),
    CrateSeg(KeywordKind),
    SelfSeg(KeywordKind),
    SuperSeg(KeywordKind),
}

pub struct SimplePath {
    dbl_colon_opt: Option<DblColon>,
    root: PathSegment,
    segments: Vec<(DblColon, PathSegment)>,
}
