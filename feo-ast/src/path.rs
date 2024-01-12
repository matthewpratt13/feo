#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{identifier::Identifier, item::DblColon, keyword::KeywordKind};

pub enum PathSegment {
    Identifier(Identifier),
    CrateKeyword(KeywordKind),
    SelfKeyword(KeywordKind),
    SuperKeyword(KeywordKind),
}

pub struct SimplePath {
    dbl_colon_opt: Option<DblColon>,
    root: PathSegment,
    segments: Vec<(DblColon, PathSegment)>,
}
