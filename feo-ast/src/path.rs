#![allow(dead_code)]

use crate::{identifier::Identifier, keyword::KeywordKind, type_utils::DblColon};

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
